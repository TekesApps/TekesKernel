import Foundation
import Testing

@testable import Tekes

/// Slice-9 cross-process acceptance. The Kernel harness owns the real
/// HTTP/WebSocket listener and injects the path of its private ready file into
/// the temporary clean Client worktree; this test uses the production carrier.
@Suite(.serialized)
struct LiveKernelTransportClientTests {
  @Test(.enabled(if: LiveKernelTransportGate.isEnabled))
  func productionClientCarrierExercisesKernelHTTPAndWebSocket() async throws {
    let rawURL = try #require(LiveKernelTransportGate.rawURL)
    let token = try #require(LiveKernelTransportGate.token)
    let projectPath = try #require(LiveKernelTransportGate.projectPath)
    let baseURL = try #require(URL(string: rawURL))
    let identity = SessionEndpointIdentity(
      id: "kernel-live-transport",
      kind: .tekes,
      baseURL: baseURL.absoluteString,
      displayName: "Kernel Live Transport")
    let endpoint = DSHHostEndpoint(
      configuration: try DSHEndpointConfiguration(
        identity: identity,
        baseURL: baseURL,
        authorizationProvider: { .bearer(token) }))
    defer { Task { await endpoint.close() } }

    let host = try await endpoint.describeHost()
    #expect(host.version == "0.1.0-rc.5")

    let workspace = try await endpoint.createWorkspace(path: projectPath)
    #expect(workspace.created)
    #expect(!workspace.workspace.workspaceId.isEmpty)

    let session = try await endpoint.createSession(
      workspaceId: workspace.workspace.workspaceId,
      cwd: nil,
      sessionId: LiveKernelTransportGate.sessionID,
      agentPreset: nil)
    #expect(session.sessionId == LiveKernelTransportGate.sessionID)

    let history = try await endpoint.history(
      sessionId: session.sessionId,
      beforeSeq: nil,
      maxMessages: 50)
    #expect(history.events.isEmpty)

    var mux = endpoint.muxFrames().makeAsyncIterator()
    let subscribedEnvelope = try #require(try await mux.next())
    #expect(subscribedEnvelope.method == "session/subscribed")
    let subscribed: SessionSubscribedFrame = try subscribedEnvelope.decodedPayload()
    #expect(subscribed.sessionId == session.sessionId)
    #expect(subscribed.lastSeq == -1)

    let registrationBarrier = try #require(try await mux.next())
    #expect(registrationBarrier.method == "session/queue")
    let eventEnvelope = try #require(try await mux.next())
    #expect(eventEnvelope.method == "session/event")
    let live: SessionEventFrame = try eventEnvelope.decodedPayload()
    #expect(live.sessionId == session.sessionId)
    #expect(live.event.seq == 0)

    let historyAfterLive = try await endpoint.history(
      sessionId: session.sessionId,
      beforeSeq: nil,
      maxMessages: 50)
    #expect(historyAfterLive.events.map(\.event.seq) == [0])

    let prompt = try await endpoint.prompt(
      rpcId: "live-client-prompt",
      sessionId: session.sessionId,
      mode: .queue,
      content: [.object(["type": .string("text"), "text": .string("hello")])],
      clientTimeZone: "Asia/Shanghai")
    #expect(prompt.rpcId == "live-client-prompt")

    let renamed = try await endpoint.renameSession(
      sessionId: session.sessionId,
      title: "Renamed by live Client")
    #expect(renamed.seq > 0)
    let archived = try await endpoint.archiveSession(sessionId: session.sessionId)
    #expect(archived.archivedSessionIds == [session.sessionId])

    let restored = try await endpoint.unarchiveSession(sessionId: session.sessionId)
    #expect(restored.sessionId == session.sessionId)
    let restoredEnvelope = try #require(try await mux.next())
    #expect(restoredEnvelope.method == "session/subscribed")
    let restoredSubscription: SessionSubscribedFrame = try restoredEnvelope.decodedPayload()
    #expect(restoredSubscription.sessionId == session.sessionId)
    #expect(restoredSubscription.lastSeq == 0)
  }

  @Test(.enabled(if: LiveKernelTransportGate.isEnabled))
  func productionArchiveRetiresMirrorAndUnarchiveRematerializes() async throws {
    let rawURL = try #require(LiveKernelTransportGate.rawURL)
    let token = try #require(LiveKernelTransportGate.token)
    let projectPath = try #require(LiveKernelTransportGate.projectPath)
    let baseURL = try #require(URL(string: rawURL))
    let identity = SessionEndpointIdentity(
      id: "kernel-live-mirror",
      kind: .tekes,
      baseURL: baseURL.absoluteString,
      displayName: "Kernel Live Mirror")
    // The harness owns this root and removes it only after xcodebuild exits.
    // Removing a SQLite/WAL directory from inside the async test can race
    // cancellation/deinitialization of Client actors even after runtime.stop().
    let mirrorRoot = URL(
      fileURLWithPath: "__TEKES_LIVE_KERNEL_MIRROR_ROOT__", isDirectory: true)
    let directory = mirrorRoot.appendingPathComponent(
      "kernel-live-mirror-\(UUID().uuidString)", isDirectory: true)
    try FileManager.default.createDirectory(
      at: directory, withIntermediateDirectories: true)
    var recordedError: (any Error)?
    do {
      let endpoint = DSHHostEndpoint(
        configuration: try DSHEndpointConfiguration(
          identity: identity,
          baseURL: baseURL,
          authorizationProvider: { .bearer(token) }))
      let mirror = try ClientMirrorStore(
        databaseURL: directory.appendingPathComponent("ClientMirror.sqlite3"),
        readConnectionCount: 2)
      let store = ClientSQLiteSessionStore(mirror: mirror)
      let runtime = SessionEndpointRuntime(
        endpoint: endpoint,
        store: store,
        reconnectDelays: [.zero])
      do {
        try await runtime.start()
        let workspace = try await runtime.createWorkspace(path: projectPath)
        let created = try await runtime.createSession(
          workspaceID: workspace.workspace.workspaceId,
          sessionID: LiveKernelTransportGate.mirrorSessionID)
        let threadID = SessionEndpointStorageIdentity.thread(
          endpointID: identity.id, sessionID: created.sessionId)
        _ = try await runtime.selectSession(created.sessionId)
        #expect(try await mirror.thread(id: threadID) != nil)
        #expect(
          try await store.sessionHistoryWindow(
            endpointID: identity.id, sessionID: created.sessionId) != nil)

        _ = try await runtime.archiveSession(sessionID: created.sessionId)
        #expect(try await mirror.thread(id: threadID) == nil)
        #expect(try await mirror.directSessionAddress(threadID: threadID) == nil)
        #expect(
          try await store.sessionHistoryWindow(
            endpointID: identity.id, sessionID: created.sessionId) == nil)

        let restored = try await runtime.unarchiveSession(sessionID: created.sessionId)
        #expect(restored.sessionId == created.sessionId)
        #expect(try await mirror.thread(id: threadID) != nil)
        // Inventory rematerializes identity only; history stays lazy until selection.
        #expect(
          try await store.sessionHistoryWindow(
            endpointID: identity.id, sessionID: created.sessionId) == nil)
        _ = try await runtime.selectSession(created.sessionId)
        #expect(
          try await store.sessionHistoryWindow(
            endpointID: identity.id, sessionID: created.sessionId) != nil)
      } catch {
        recordedError = error
      }
      await runtime.stop()
    }
    if let recordedError { throw recordedError }
  }
}

private enum LiveKernelTransportGate {
  private static let values: [String] = {
    guard
      let contents = try? String(
        contentsOfFile: "__TEKES_LIVE_KERNEL_READY_FILE__",
        encoding: .utf8)
    else {
      return []
    }
    return contents.split(whereSeparator: \.isNewline).map(String.init)
  }()

  static let rawURL = values.first
  static let token = values.count >= 2 ? values[1] : nil
  static let projectPath = values.count >= 3 ? values[2] : nil
  static let sessionID = "018f0000-0000-7000-8000-000000000003"
  static let mirrorSessionID = "018f0000-0000-7000-8000-000000000004"
  static let isEnabled = rawURL != nil && token != nil && projectPath != nil
}
