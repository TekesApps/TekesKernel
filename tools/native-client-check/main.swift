import Foundation

@main struct NativeClientCheck {
  static func main() async throws {
    let url = URL(string: CommandLine.arguments[1])!
    let root = CommandLine.arguments[2]
    let hex = ProcessInfo.processInfo.environment["TEKES_KERNEL_ENDPOINT_TOKEN"]!
    var bytes = Data()
    var index = hex.startIndex
    while index < hex.endIndex {
      let end = hex.index(index, offsetBy: 2)
      bytes.append(UInt8(hex[index..<end], radix: 16)!)
      index = end
    }
    let bearer = bytes.base64EncodedString().replacingOccurrences(of: "+", with: "-")
      .replacingOccurrences(of: "/", with: "_").replacingOccurrences(of: "=", with: "")
    let endpoint = NativeSessionEndpoint(identity: .init(id: "kernel-check", kind: .tekes,
      baseURL: url.absoluteString, displayName: "Kernel")) { .init(url: url, bearer: bearer) }
    do {
      let host = try await endpoint.describeHost()
      precondition(host.product.name == "TekesKernel")
      let draft = try await endpoint.draftModels()
      let attachmentPolicy = try await endpoint.attachmentPolicy(sessionId: nil)
      precondition(attachmentPolicy.inlineMediaTypes == ["image/png", "image/jpeg", "image/gif", "image/webp"])
      precondition(attachmentPolicy.maxAttachmentBytes == 8_388_608)
      precondition(attachmentPolicy.maxImageDimension == 16_384)
      precondition(attachmentPolicy.maxImagePixels == 67_108_864)
      precondition(!draft.groups.isEmpty)
      let threadApprovals = try await endpoint.approvalsAtThreadLevel()
      precondition(threadApprovals.map(\.value) == ["read-only", "workspace-write", "danger-full-access"])
      precondition(threadApprovals.map(\.title) == ["Read Only", "Workspace Write", "Full Access"])
      precondition(threadApprovals.allSatisfy { $0.description?.isEmpty == false })
      let serverApprovals = try await endpoint.approvalsAtServerLevel()
      precondition(serverApprovals.options.isEmpty)
      precondition(serverApprovals.currentValue == nil)
      print("NATIVE_APPROVAL_POLICY_OK")
      var workspaces = endpoint.workspaceSyncFrames().makeAsyncIterator()
      guard let baseline = try await workspaces.next() else { throw NativeSessionEndpoint.Failure.protocolError }
      try baseline.validate()
      let workspace = try await endpoint.createWorkspace(path: root)
      let renamedWorkspace = try await endpoint.renameWorkspace(workspaceId: workspace.workspace.workspaceId,
        title: "Kernel contract acceptance")
      precondition(renamedWorkspace.workspace.title == "Kernel contract acceptance")
      let created = try await endpoint.createSession(workspaceId: workspace.workspace.workspaceId,
        cwd: nil, sessionId: nil, agentPreset: nil)
      let catalog = try await endpoint.models(sessionId: created.sessionId)
      precondition(!catalog.groups.isEmpty)
      // Per-session permission mode: the composer applies a selection by
      // running the reserved `permission` command; `approvals.mode` reads it back.
      let defaultMode = try await endpoint.permissionMode(sessionId: created.sessionId)
      precondition(defaultMode == "workspace-write")
      let permissionReceipt = try await endpoint.executeCommand(sessionId: created.sessionId, name: "permission", arguments: "read-only")
      precondition(!permissionReceipt.commandId.isEmpty)
      let selectedMode = try await endpoint.permissionMode(sessionId: created.sessionId)
      precondition(selectedMode == "read-only")
      print("NATIVE_PERMISSION_MODE_OK")
      // Extension sweep: every new capability through the real Swift client.
      let listing = try await endpoint.listDirectory(path: root)
      precondition(listing.path == root && listing.crumbs.first?.path == "/")
      let createdDirectory = try await endpoint.createDirectory(path: root, name: "native-picked")
      precondition(createdDirectory.hasSuffix("/native-picked"))
      let fileRefs = try await endpoint.fileReferenceCandidates(sessionID: created.sessionId, query: "file-page")
      precondition(fileRefs.contains { $0.path == "file-page.txt" && $0.kind == "file" })
      let sessionRefs = try await endpoint.sessionReferenceCandidates(sessionID: created.sessionId, query: "")
      precondition(!sessionRefs.contains { $0.sessionId == created.sessionId })
      let stored = try await endpoint.putMessageFeedback(sessionId: created.sessionId, messageId: "m-1", rating: .positive, ifVersion: nil, note: "helpful")
      precondition(stored.note == "helpful")
      do {
        _ = try await endpoint.putMessageFeedback(sessionId: created.sessionId, messageId: "m-1", rating: .negative, ifVersion: nil)
        preconditionFailure("stale feedback version accepted")
      } catch SessionFeedbackFailure.versionConflict(let current) { precondition(current?.version == stored.version) }
      let feedbacks = try await endpoint.listMessageFeedback(sessionId: created.sessionId)
      precondition(feedbacks.map(\.messageId) == ["m-1"])
      try await endpoint.deleteMessageFeedback(sessionId: created.sessionId, messageId: "m-1", ifVersion: stored.version)
      let settings = try await endpoint.describeSettings()
      precondition(settings.hasDocument && settings.namespaces.map(\.ns) == ["policy"])
      let mutated = try await endpoint.mutateSettings(namespace: "policy", operations: [.set(path: ["network"], value: .bool(false))], expectedRevision: settings.namespaces[0].revision)
      precondition(mutated.revision == settings.namespaces[0].revision + 1)
      let goal = try await endpoint.editGoal(sessionId: created.sessionId, ref: .init(id: "new", revision: 0), objective: "ship")
      let paused = try await endpoint.controlGoal(sessionId: created.sessionId, ref: .init(id: goal.id, revision: goal.revision), action: .pause)
      let live = try await endpoint.liveGoal(sessionId: created.sessionId)
      precondition(paused.phase == "paused" && live?.revision == paused.revision)
      _ = try await endpoint.clearGoal(sessionId: created.sessionId, ref: .init(id: paused.id, revision: paused.revision))
      let cleared = try await endpoint.liveGoal(sessionId: created.sessionId)
      precondition(cleared == nil)
      let subagents = try await endpoint.subagentCatalog(parentSessionId: created.sessionId)
      precondition(subagents.parentAvailable && subagents.entries.first?.kind == "parent")
      let policy = try await endpoint.workspacePolicy(workspaceId: workspace.workspace.workspaceId)
      precondition(policy.format == 1 && !policy.policy.allowedTools.isEmpty)
      var relaxed = policy.policy
      relaxed.network = !relaxed.network
      let savedPolicy = try await endpoint.setWorkspacePolicy(workspaceId: workspace.workspace.workspaceId, expectedRevision: policy.revision, policy: relaxed)
      precondition(savedPolicy.revision > policy.revision && savedPolicy.policy == relaxed)
      do {
        _ = try await endpoint.setWorkspacePolicy(workspaceId: workspace.workspace.workspaceId, expectedRevision: policy.revision, policy: relaxed)
        preconditionFailure("stale policy revision accepted")
      } catch KernelWorkspacePolicyError.staleRevision {}
      print("NATIVE_EXTENSION_SWEEP_OK")
      let uploadURL = URL(fileURLWithPath: root).appendingPathComponent("native-upload.txt")
      let uploadBytes = Data("native file attachment round trip\n".utf8)
      try uploadBytes.write(to: uploadURL)
      let uploadReceipt = try await endpoint.uploadFile(sessionId: created.sessionId, fileURL: uploadURL)
      precondition(uploadReceipt.file.name == "native-upload.txt" && uploadReceipt.file.bytes == uploadBytes.count)
      precondition(uploadReceipt.file.attachmentId.hasPrefix("sha256:"))
      _ = try await endpoint.prompt(rpcId: UUID().uuidString.lowercased(), sessionId: created.sessionId, mode: .queue,
        content: [.object(["type": .string("text"), "text": .string("Native file attachment check")]),
          .object(["type": .string("file"), "receiptId": .string(uploadReceipt.receiptId)])], clientTimeZone: nil)
      let fetchedFile = try await endpoint.fileAttachment(sessionId: created.sessionId, attachmentId: uploadReceipt.file.attachmentId)
      precondition(fetchedFile.attachment.attachmentId == uploadReceipt.file.attachmentId)
      precondition(fetchedFile.attachment.bytes == uploadBytes.count && fetchedFile.attachment.mediaType == "text/plain")
      precondition(Data(base64Encoded: fetchedFile.data) == uploadBytes)
      print("NATIVE_FILE_ATTACHMENT_OK")
      let fileStat = try await endpoint.statSessionFile(sessionID: created.sessionId, path: "file-page.txt")
      let textPage = try await endpoint.readSessionFile(sessionID: created.sessionId, path: "file-page.txt", offset: 2, limit: 1)
      precondition(fileStat.bytes == 13 && textPage.text == "second" && textPage.eof)
      precondition(fileStat.version == textPage.version)
      let content = Data(repeating: 0xab, count: 600_000)
      try content.write(to: URL(fileURLWithPath: root).appendingPathComponent("multi-page.bin"))
      let completeFile = try await endpoint.readSessionFileAll(sessionID: created.sessionId, path: "multi-page.bin")
      precondition(completeFile.eof && completeFile.offset == 0 && completeFile.bytes == content.count)
      precondition(Data(base64Encoded: completeFile.data) == content)
      print("NATIVE_SESSION_FILE_PAGES_AND_COMPLETE_READ_OK")
      let watchedURL = URL(fileURLWithPath: root).appendingPathComponent("watched.txt")
      try Data("before".utf8).write(to: watchedURL)
      let observation = endpoint.openSessionFileChanges(sessionID: created.sessionId)
      var fileFrames = observation.frames.makeAsyncIterator()
      guard case .ready? = try await fileFrames.next() else { throw NativeSessionEndpoint.Failure.protocolError }
      let watchedStat = try await endpoint.statSessionFile(sessionID: created.sessionId, path: "watched.txt")
      try Data("after".utf8).write(to: watchedURL)
      guard case .changed(let changedPath, let changedVersion)? = try await fileFrames.next() else { throw NativeSessionEndpoint.Failure.protocolError }
      precondition(changedPath == watchedStat.absolutePath && changedVersion != watchedStat.version)
      try FileManager.default.removeItem(at: watchedURL)
      guard case .absent(let absentPath)? = try await fileFrames.next() else { throw NativeSessionEndpoint.Failure.protocolError }
      precondition(absentPath == watchedStat.absolutePath)
      _ = try await endpoint.renameWorkspace(workspaceId: workspace.workspace.workspaceId, title: "File authority changed")
      var generationEnded = false
      do { generationEnded = try await fileFrames.next() == nil } catch { generationEnded = true }
      precondition(generationEnded)
      await observation.close()
      let renewedObservation = endpoint.openSessionFileChanges(sessionID: created.sessionId)
      var renewedFrames = renewedObservation.frames.makeAsyncIterator()
      guard case .ready? = try await renewedFrames.next() else { throw NativeSessionEndpoint.Failure.protocolError }
      await renewedObservation.close()
      print("NATIVE_FILE_OBSERVATION_AUTHORITY_REFRESH_OK")
      print("NATIVE_FILE_OBSERVATION_WRITE_DELETE_CLOSE_OK")
      let resources = try await endpoint.createWorkspace(path: root + "/resource-project")
      let resourceSession = try await endpoint.createSession(workspaceId: resources.workspace.workspaceId,
        cwd: nil, sessionId: nil, agentPreset: nil)
      do {
        _ = try await endpoint.statSessionFile(sessionID: resourceSession.sessionId, path: root + "/file-page.txt")
        preconditionFailure("Session file read escaped its workspace")
      } catch NativeSessionEndpoint.Failure.rpc(let error) {
        precondition(error.code == "file-unavailable")
      }
      let skills = try await endpoint.skills(sessionId: resourceSession.sessionId)
      precondition(skills.map(\.name) == ["project-only"] && skills[0].modelInvocable)
      let commands = try await endpoint.commands(sessionId: resourceSession.sessionId)
      precondition(commands.map(\.name) == ["project-only"])
      do {
        _ = try await endpoint.executeCommand(sessionId: resourceSession.sessionId, name: "missing-command", arguments: nil)
        preconditionFailure("Unknown command unexpectedly succeeded")
      } catch NativeSessionEndpoint.Failure.rpc(let error) {
        precondition(error.code == "command-not-found")
      }
      _ = try await endpoint.archiveSession(sessionId: resourceSession.sessionId)
      do {
        _ = try await endpoint.executeCommand(sessionId: resourceSession.sessionId, name: "project-only", arguments: "test")
        preconditionFailure("Archived session accepted command input")
      } catch NativeSessionEndpoint.Failure.rpc(let error) {
        precondition(error.code == "session-archived")
      }
      _ = try await endpoint.unarchiveSession(sessionId: resourceSession.sessionId)
      if ProcessInfo.processInfo.environment["TEKES_TEST_IMAGE_PROMPT"] == "1" {
        let receipt = try await endpoint.executeCommand(sessionId: resourceSession.sessionId,
          name: "project-only", arguments: "swift-command-fixture")
        precondition(UUID(uuidString: receipt.commandId) != nil)
        print("NATIVE_COMMAND_RECEIPT_OK")
      }
      let otherSkills = try await endpoint.skills(sessionId: created.sessionId)
      precondition(otherSkills.isEmpty)
      var control = endpoint.sessionControlSyncFrames().makeAsyncIterator()
      var actionables = endpoint.actionableSyncFrames().makeAsyncIterator()
      guard let controlBaseline = try await control.next(), let actionableBaseline = try await actionables.next() else {
        throw NativeSessionEndpoint.Failure.protocolError
      }
      try controlBaseline.validate()
      try actionableBaseline.validate()
      do {
        _ = try await endpoint.models(sessionId: UUID().uuidString.lowercased())
        preconditionFailure("Missing session unexpectedly succeeded")
      } catch NativeSessionEndpoint.Failure.rpc(let error) {
        precondition(error.code == "session-not-found")
      }
      let provider = catalog.groups[0]
      try await endpoint.selectModel(sessionId: created.sessionId,
        selection: .init(provider: provider.id, model: provider.models[0].id))
      var journal = endpoint.followJournal(.init(sessionId: created.sessionId, maxMessages: 7)).makeAsyncIterator()
      guard case .snapshot(let snapshot)? = try await journal.next() else { throw NativeSessionEndpoint.Failure.protocolError }
      try snapshot.validate()
      _ = try await endpoint.historyPage(.init(sessionId: created.sessionId,
        throughSequence: snapshot.throughSequence, maxMessages: 7))
      let renamed = try await endpoint.renameSession(sessionId: created.sessionId, title: "Native contract check")
      precondition(renamed.title == "Native contract check")
      let search = try await endpoint.searchSessions(query: "native contract")
      precondition(search.items.contains { $0.sessionId == created.sessionId && $0.snippet.contains("native contract") })
      precondition(!search.hasMore)
      print("NATIVE_HOST_SEARCH_OK")
      let fork = try await endpoint.forkSession(sessionId: created.sessionId, atSeq: nil)
      precondition(fork.sessionId != created.sessionId)
      let archived = try await endpoint.archiveSession(sessionId: fork.sessionId)
      precondition(archived.archivedSessionIds.contains(fork.sessionId))
      let restored = try await endpoint.unarchiveSession(sessionId: fork.sessionId)
      precondition(restored.sessionId == fork.sessionId)
      try await endpoint.recoverInterruptedSessions(sessionIDs: [created.sessionId])
      try await endpoint.recoverInterruptedSessions(sessionIDs: [created.sessionId])
      await endpoint.close()
      let reconnected = try await endpoint.describeHost()
      precondition(reconnected.product.name == "TekesKernel")
      let preserved = try await endpoint.models(sessionId: created.sessionId)
      precondition(!preserved.groups.isEmpty)
      await endpoint.close()
      print("NATIVE_CLOSE_RECONNECT_PRESERVES_SESSION_OK")
      print("NATIVE_SESSION_ENDPOINT_RUNTIME_OK")
    } catch {
      await endpoint.close()
      throw error
    }
  }
}

/// Check-only helper: the app never reads the mode directly (its composer
/// state comes from the command receipt), so this stays out of Tekes.
extension NativeSessionEndpoint {
  func permissionMode(sessionId: String) async throws -> String {
    struct Mode: Decodable { let mode: String }
    let result: Mode = try await call("approvals.mode", ["sessionId": .string(sessionId)])
    return result.mode
  }
}
