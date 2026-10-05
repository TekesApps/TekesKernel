import CryptoKit
import Darwin
import Foundation
import Security

private let protocolName = "tekes-kernel-production-uat"
private let serviceLabel = "com.tekes.kernel.supervisor"
private let endpointService = "com.tekes.kernel.endpoint"
private let endpointAccount = "loopback-bearer"
private let providerService = "com.tekes.kernel.provider-secret"
// Services these records were stored under before the rename. The installer
// moves a found record to the current service; uninstall removes both so an
// upgraded machine leaves nothing behind.
private let legacyEndpointService = "com.tekes.kernel.endpoint.v1"
private let legacyProviderService = "com.tekes.kernel.provider-secret.v1"
private let providerAccount = "provider-uat"
private let operationService = "com.tekes.kernel.production-uat.v1"
private let designatedRequirementPrefix = "anchor apple generic and identifier "

private struct Failure: Error, CustomStringConvertible {
    let code: String
    let detail: String
    var description: String { "\(code): \(detail)" }
}

private struct Arguments {
    let gate: Int
    let fixtures: String
    let selector: String
    let supervisor: String
    let worker: String
    let helper: String
    let clientUAT: String
    let origin: String
    let releaseVersion: String
    let selectorConformance: String

    static func parsePrepare(_ argv: [String]) throws -> Arguments {
        let names = [
            "--gate", "--fixtures", "--selector", "--supervisor", "--worker",
            "--helper", "--client-uat", "--origin", "--release-version",
            "--selector-conformance",
        ]
        guard argv.count == names.count * 2 else {
            throw Failure(code: "usage", detail: "expected the closed production argv")
        }
        var values: [String: String] = [:]
        for index in names.indices {
            let offset = index * 2
            guard argv[offset] == names[index] else {
                throw Failure(code: "usage", detail: "unexpected argument at position \(offset + 1)")
            }
            values[names[index]] = argv[offset + 1]
        }
        guard let gate = Int(values["--gate"] ?? ""), gate == 72 || gate == 76 else {
            throw Failure(code: "usage", detail: "gate must be 72 or 76")
        }
        guard values["--origin"] == "http://127.0.0.1:7347" else {
            throw Failure(code: "usage", detail: "origin is fixed")
        }
        guard values["--release-version"] == "1.0.0" else {
            throw Failure(code: "usage", detail: "release version is fixed for the v1 oracle")
        }
        func absoluteFile(_ name: String, executable: Bool = false) throws -> String {
            let value = values[name]!
            guard value.hasPrefix("/"), URL(fileURLWithPath: value).standardized.path == value else {
                throw Failure(code: "usage", detail: "\(name) must be an absolute normalized path")
            }
            var isDirectory: ObjCBool = false
            guard FileManager.default.fileExists(atPath: value, isDirectory: &isDirectory),
                  !isDirectory.boolValue else {
                throw Failure(code: "missing-input", detail: name)
            }
            if executable && access(value, X_OK) != 0 {
                throw Failure(code: "missing-input", detail: "\(name) is not executable")
            }
            return value
        }
        func absoluteDirectory(_ name: String) throws -> String {
            let value = values[name]!
            guard value.hasPrefix("/"), URL(fileURLWithPath: value).standardized.path == value else {
                throw Failure(code: "usage", detail: "\(name) must be an absolute normalized path")
            }
            var isDirectory: ObjCBool = false
            guard FileManager.default.fileExists(atPath: value, isDirectory: &isDirectory),
                  isDirectory.boolValue else {
                throw Failure(code: "missing-input", detail: "\(name) is not a directory")
            }
            return value
        }
        func application(_ name: String, executable: String) throws -> String {
            let app = try absoluteDirectory(name)
            guard app.hasSuffix(".app"),
                  access(app + "/Contents/MacOS/" + executable, X_OK) == 0 else {
                throw Failure(code: "missing-input", detail: "\(name) is not the expected app")
            }
            try validateSignedApplication(app, executable: executable)
            return app
        }
        return try Arguments(
            gate: gate,
            fixtures: absoluteDirectory("--fixtures"),
            selector: absoluteFile("--selector", executable: true),
            supervisor: application("--supervisor", executable: "tekes-supervisor"),
            worker: absoluteFile("--worker", executable: true),
            helper: absoluteFile("--helper", executable: true),
            clientUAT: application("--client-uat", executable: "tekes-kernel-client-uat"),
            origin: values["--origin"]!,
            releaseVersion: values["--release-version"]!,
            selectorConformance: absoluteFile("--selector-conformance")
        )
    }
}

private enum Invocation {
    case describe
    case prepare(Arguments)
    case resume(String)
    case verify(String)
    case consume(String)
    case acknowledge(String)

    static func parse(_ argv: [String]) throws -> Invocation {
        if argv == ["--describe-contract"] { return .describe }
        if argv.count >= 2, argv[0] == "--mode", argv[1] == "prepare" {
            return .prepare(try Arguments.parsePrepare(Array(argv.dropFirst(2))))
        }
        if argv.count == 4, argv[0] == "--mode", argv[2] == "--operation",
           ["resume", "verify", "consume", "acknowledge"].contains(argv[1]),
           UUID(uuidString: argv[3]) != nil,
           argv[3] == argv[3].lowercased() {
            switch argv[1] {
            case "resume": return .resume(argv[3])
            case "verify": return .verify(argv[3])
            case "consume": return .consume(argv[3])
            default: return .acknowledge(argv[3])
            }
        }
        throw Failure(code: "usage", detail: "expected a closed production UAT invocation")
    }
}

private struct CommandResult {
    let status: Int32
    let stdout: Data
    let stderr: Data
}

private let commandTimeout: TimeInterval = 60
private let commandOutputLimit = 1_048_576

@discardableResult
private func run(_ executable: String, _ arguments: [String], stdin: Data? = nil) throws -> CommandResult {
    let scratch = NSTemporaryDirectory() + "/tekes-production-uat-command-\(UUID().uuidString.lowercased())"
    try FileManager.default.createDirectory(atPath: scratch, withIntermediateDirectories: false)
    defer { try? FileManager.default.removeItem(atPath: scratch) }
    let stdoutPath = scratch + "/stdout"
    let stderrPath = scratch + "/stderr"
    guard FileManager.default.createFile(atPath: stdoutPath, contents: nil),
          FileManager.default.createFile(atPath: stderrPath, contents: nil) else {
        throw Failure(code: "io", detail: "command output files")
    }
    let stdoutHandle = try FileHandle(forWritingTo: URL(fileURLWithPath: stdoutPath))
    let stderrHandle = try FileHandle(forWritingTo: URL(fileURLWithPath: stderrPath))
    defer {
        try? stdoutHandle.close()
        try? stderrHandle.close()
    }
    let process = Process()
    process.executableURL = URL(fileURLWithPath: "/usr/bin/python3")
    process.arguments = [
        "-c",
        "import os,sys; os.setsid(); os.execv(sys.argv[1], sys.argv[1:])",
        executable,
    ] + arguments
    process.environment = [:]
    process.standardOutput = stdoutHandle
    process.standardError = stderrHandle
    let input = Pipe()
    if stdin != nil { process.standardInput = input }
    try process.run()
    if let stdin {
        try input.fileHandleForWriting.write(contentsOf: stdin)
        try input.fileHandleForWriting.close()
    }
    let deadline = Date().addingTimeInterval(commandTimeout)
    var limitExceeded = false
    while process.isRunning && Date() < deadline {
        let stdoutBytes = (try? FileManager.default.attributesOfItem(atPath: stdoutPath)[.size] as? NSNumber)?.intValue ?? 0
        let stderrBytes = (try? FileManager.default.attributesOfItem(atPath: stderrPath)[.size] as? NSNumber)?.intValue ?? 0
        if stdoutBytes > commandOutputLimit || stderrBytes > commandOutputLimit {
            limitExceeded = true
            break
        }
        usleep(20_000)
    }
    let timedOut = process.isRunning && !limitExceeded
    if process.isRunning {
        _ = kill(-process.processIdentifier, SIGTERM)
        let grace = Date().addingTimeInterval(2)
        while process.isRunning && Date() < grace { usleep(20_000) }
        if process.isRunning { _ = kill(-process.processIdentifier, SIGKILL) }
    }
    process.waitUntilExit()
    if limitExceeded { throw Failure(code: "command-output-limit", detail: executable) }
    if timedOut { throw Failure(code: "command-timeout", detail: executable) }
    let stdout = try Data(contentsOf: URL(fileURLWithPath: stdoutPath))
    let stderr = try Data(contentsOf: URL(fileURLWithPath: stderrPath))
    guard stdout.count <= commandOutputLimit, stderr.count <= commandOutputLimit else {
        throw Failure(code: "command-output-limit", detail: executable)
    }
    return CommandResult(
        status: process.terminationStatus,
        stdout: stdout,
        stderr: stderr
    )
}

private func requireSuccess(_ executable: String, _ arguments: [String]) throws -> Data {
    let result = try run(executable, arguments)
    guard result.status == 0, result.stderr.isEmpty else {
        throw Failure(code: "command-failed", detail: "\(executable) \(arguments.first ?? "")")
    }
    return result.stdout
}

private func canonical(_ value: Any, lf: Bool = true) throws -> Data {
    guard JSONSerialization.isValidJSONObject(value) else {
        throw Failure(code: "invalid-json", detail: "non-I-JSON value")
    }
    var data = try JSONSerialization.data(withJSONObject: value, options: [.sortedKeys, .withoutEscapingSlashes])
    if lf { data.append(0x0a) }
    return data
}

private func decodeCanonical(_ data: Data) throws -> Any {
    guard data.last == 0x0a, !data.dropLast().contains(0x0a) else {
        throw Failure(code: "protocol", detail: "response must be exactly one JSON line")
    }
    let value = try JSONSerialization.jsonObject(with: data.dropLast())
    guard try canonical(value) == data else {
        throw Failure(code: "protocol", detail: "response is not canonical JSON plus LF")
    }
    return value
}

private func sha256(_ data: Data) -> String {
    SHA256.hash(data: data).map { String(format: "%02x", $0) }.joined()
}

private func hmacSHA256(_ data: Data, secret: Data) -> String {
    HMAC<SHA256>.authenticationCode(for: data, using: SymmetricKey(data: secret))
        .map { String(format: "%02x", $0) }
        .joined()
}

private func xmlEscaped(_ value: String) -> String {
    value
        .replacingOccurrences(of: "&", with: "&amp;")
        .replacingOccurrences(of: "<", with: "&lt;")
        .replacingOccurrences(of: ">", with: "&gt;")
        .replacingOccurrences(of: "\"", with: "&quot;")
        .replacingOccurrences(of: "'", with: "&apos;")
}

private func createDirectory(_ path: String) throws {
    try FileManager.default.createDirectory(atPath: path, withIntermediateDirectories: true)
    guard chmod(path, 0o700) == 0 else { throw Failure(code: "io", detail: "chmod \(path)") }
}

private func syncDirectory(_ path: String) throws {
    let descriptor = open(path, O_RDONLY | O_DIRECTORY | O_CLOEXEC)
    guard descriptor >= 0 else { throw Failure(code: "io", detail: "open directory \(path)") }
    defer { close(descriptor) }
    guard fsync(descriptor) == 0 else { throw Failure(code: "io", detail: "sync directory \(path)") }
}

private func durableWrite(_ path: String, data: Data, mode: mode_t) throws {
    let parent = URL(fileURLWithPath: path).deletingLastPathComponent().path
    try createDirectory(parent)
    let temporary = parent + "/.uat-\(UUID().uuidString.lowercased())"
    let descriptor = open(temporary, O_WRONLY | O_CREAT | O_EXCL | O_CLOEXEC | O_NOFOLLOW, mode)
    guard descriptor >= 0 else { throw Failure(code: "io", detail: "create \(path)") }
    do {
        try data.withUnsafeBytes { raw in
            var offset = 0
            while offset < raw.count {
                let count = write(descriptor, raw.baseAddress!.advanced(by: offset), raw.count - offset)
                if count < 0 && errno == EINTR { continue }
                guard count > 0 else { throw Failure(code: "io", detail: "write \(path)") }
                offset += count
            }
        }
        guard fcntl(descriptor, F_FULLFSYNC) == 0 else {
            throw Failure(code: "io", detail: "F_FULLFSYNC \(path)")
        }
        close(descriptor)
        guard rename(temporary, path) == 0, chmod(path, mode) == 0 else {
            throw Failure(code: "io", detail: "publish \(path)")
        }
        try syncDirectory(parent)
    } catch {
        close(descriptor)
        unlink(temporary)
        throw error
    }
}

private func durableRemove(_ path: String) throws {
    guard FileManager.default.fileExists(atPath: path) else { return }
    let parent = URL(fileURLWithPath: path).deletingLastPathComponent().path
    try FileManager.default.removeItem(atPath: path)
    try syncDirectory(parent)
}

private func copyExecutable(_ source: String, _ destination: String) throws -> [String: Any] {
    let parent = URL(fileURLWithPath: destination).deletingLastPathComponent().path
    try createDirectory(parent)
    try FileManager.default.copyItem(atPath: source, toPath: destination)
    guard chmod(destination, 0o755) == 0 else { throw Failure(code: "io", detail: destination) }
    let descriptor = open(destination, O_RDONLY | O_CLOEXEC | O_NOFOLLOW)
    guard descriptor >= 0 else { throw Failure(code: "io", detail: destination) }
    defer { close(descriptor) }
    guard fcntl(descriptor, F_FULLFSYNC) == 0 else {
        throw Failure(code: "io", detail: "sync \(destination)")
    }
    try syncDirectory(parent)
    let data = try Data(contentsOf: URL(fileURLWithPath: destination))
    return ["bytes": data.count, "mode": "0755", "path": "", "sha256": sha256(data)]
}

private let signedApplicationResources = [
    ("Contents/Info.plist", 0o644),
    ("Contents/MacOS", 0),
    ("Contents/_CodeSignature/CodeResources", 0o644),
    ("Contents/embedded.provisionprofile", 0o644),
]

private func applicationExecutable(_ app: String, _ executable: String) throws -> String {
    let path = app + "/Contents/MacOS/" + executable
    guard app.hasSuffix(".app"), access(path, X_OK) == 0 else {
        throw Failure(code: "invalid-app", detail: app)
    }
    return path
}

private func currentApplication(_ executable: String) throws -> String {
    let path = URL(fileURLWithPath: CommandLine.arguments[0]).standardized
    guard path.lastPathComponent == executable,
          path.deletingLastPathComponent().lastPathComponent == "MacOS",
          path.deletingLastPathComponent().deletingLastPathComponent().lastPathComponent == "Contents" else {
        throw Failure(code: "invalid-app", detail: path.path)
    }
    let app = path
        .deletingLastPathComponent()
        .deletingLastPathComponent()
        .deletingLastPathComponent()
        .path
    guard app.hasSuffix(".app") else { throw Failure(code: "invalid-app", detail: app) }
    return app
}

private func validateSignedApplication(_ app: String, executable: String) throws {
    try noSymlinkComponents(app)
    guard try directoryNames(app) == ["Contents"],
          try directoryNames(app + "/Contents") == [
              "Info.plist", "MacOS", "_CodeSignature", "embedded.provisionprofile",
          ],
          try directoryNames(app + "/Contents/MacOS") == [executable],
          try directoryNames(app + "/Contents/_CodeSignature") == ["CodeResources"] else {
        throw Failure(code: "invalid-app", detail: app)
    }
    _ = try applicationExecutable(app, executable)
    for path in [
        app + "/Contents",
        app + "/Contents/MacOS",
        app + "/Contents/_CodeSignature",
    ] {
        try noSymlinkComponents(path)
        var status = stat()
        guard lstat(path, &status) == 0,
              status.st_mode & S_IFMT == S_IFDIR else {
            throw Failure(code: "invalid-app", detail: path)
        }
    }
    for (relative, expectedMode) in signedApplicationResources where expectedMode != 0 {
        let path = app + "/" + relative
        try noSymlinkComponents(path)
        var status = stat()
        guard lstat(path, &status) == 0,
              status.st_mode & S_IFMT == S_IFREG,
              status.st_mode & 0o777 == expectedMode else {
            throw Failure(code: "invalid-app", detail: path)
        }
    }
    var executableStatus = stat()
    let executablePath = try applicationExecutable(app, executable)
    try noSymlinkComponents(executablePath)
    guard lstat(executablePath, &executableStatus) == 0,
          executableStatus.st_mode & S_IFMT == S_IFREG,
          executableStatus.st_mode & 0o777 == 0o755 else {
        throw Failure(code: "invalid-app", detail: executablePath)
    }
}

private func identifier(from requirement: String) throws -> String {
    guard requirement.hasPrefix(designatedRequirementPrefix) else {
        throw Failure(code: "invalid-signature", detail: "unsupported designated requirement")
    }
    let identifier = String(requirement.dropFirst(designatedRequirementPrefix.count))
    guard !identifier.isEmpty,
          identifier.unicodeScalars.allSatisfy({
              CharacterSet.alphanumerics.union(CharacterSet(charactersIn: ".-")).contains($0)
          }) else {
        throw Failure(code: "invalid-signature", detail: "invalid designated identifier")
    }
    return identifier
}

private func verifyEmbeddedProvisioningProfile(
    _ app: String,
    identifier: String,
    requiredGroups: Set<String>
) throws {
    let infoData = try Data(contentsOf: URL(fileURLWithPath: app + "/Contents/Info.plist"))
    guard let info = try PropertyListSerialization.propertyList(
        from: infoData,
        options: [],
        format: nil
    ) as? [String: Any],
          info["CFBundleIdentifier"] as? String == identifier else {
        throw Failure(code: "invalid-profile", detail: "bundle identifier mismatch")
    }
    let profilePath = app + "/Contents/embedded.provisionprofile"
    let decoded = try run("/usr/bin/security", ["cms", "-D", "-i", profilePath])
    guard decoded.status == 0,
          let profile = try PropertyListSerialization.propertyList(
              from: decoded.stdout,
              options: [],
              format: nil
          ) as? [String: Any],
          let teams = profile["TeamIdentifier"] as? [String],
          teams.contains(BuildIdentity.teamID),
          let expiration = profile["ExpirationDate"] as? Date,
          expiration > Date(),
          let entitlements = profile["Entitlements"] as? [String: Any],
          entitlements["com.apple.application-identifier"] as? String
              == "\(BuildIdentity.teamID).\(identifier)",
          entitlements["com.apple.developer.team-identifier"] as? String
              == BuildIdentity.teamID,
          let grantedGroups = entitlements["keychain-access-groups"] as? [String],
          grantedGroups.allSatisfy({ !$0.isEmpty }) else {
        throw Failure(code: "invalid-profile", detail: app)
    }
    let allowed = Set(grantedGroups)
    guard allowed.contains("\(BuildIdentity.teamID).*")
              || requiredGroups.isSubset(of: allowed) else {
        throw Failure(code: "invalid-profile", detail: "Keychain access-group authorization")
    }
}

private func verifyClientAdmissionProbe(_ args: Arguments, identifier: String) throws {
    let executable = try applicationExecutable(args.clientUAT, "tekes-kernel-client-uat")
    let output = try requireSuccess(executable, ["--describe-contract"])
    guard let value = try decodeCanonical(output) as? [String: Any],
          Set(value.keys) == Set(["format", "identifier", "protocol", "release_version"]),
          value["format"] as? Int == 1,
          value["identifier"] as? String == identifier,
          value["protocol"] as? String == protocolName,
          value["release_version"] as? String == args.releaseVersion else {
        throw Failure(code: "client-probe", detail: "signed Client contract/build mismatch")
    }
}

private func copySignedApplication(
    _ source: String,
    _ destination: String,
    executable: String
) throws -> [[String: Any]] {
    try validateSignedApplication(source, executable: executable)
    if FileManager.default.fileExists(atPath: destination) {
        try validateSignedApplication(destination, executable: executable)
        let sourceBinding = try inputBinding(source)
        let destinationBinding = try inputBinding(destination)
        guard sourceBinding["bytes"] as? Int == destinationBinding["bytes"] as? Int,
              sourceBinding["sha256"] as? String == destinationBinding["sha256"] as? String else {
            throw Failure(code: "operation-mismatch", detail: destination)
        }
    } else {
        try createDirectory(destination)
        try createDirectory(destination + "/Contents")
        try createDirectory(destination + "/Contents/MacOS")
        try createDirectory(destination + "/Contents/_CodeSignature")
        for (relative, mode) in signedApplicationResources where mode != 0 {
            let data = try Data(contentsOf: URL(fileURLWithPath: source + "/" + relative))
            try durableWrite(destination + "/" + relative, data: data, mode: mode_t(mode))
        }
        _ = try copyExecutable(
            try applicationExecutable(source, executable),
            destination + "/Contents/MacOS/" + executable
        )
        try syncDirectory(destination + "/Contents/MacOS")
        try syncDirectory(destination + "/Contents/_CodeSignature")
        try syncDirectory(destination + "/Contents")
        try syncDirectory(destination)
    }
    let files = [
        ("Contents/Info.plist", "0644"),
        ("Contents/MacOS/" + executable, "0755"),
        ("Contents/_CodeSignature/CodeResources", "0644"),
        ("Contents/embedded.provisionprofile", "0644"),
    ]
    return try files.map { relative, mode in
        let data = try Data(contentsOf: URL(fileURLWithPath: destination + "/" + relative))
        return ["bytes": data.count, "mode": mode, "path": relative, "sha256": sha256(data)]
    }
}

private final class KeychainItem {
    let service: String
    let account: String
    let accessGroup: String

    init(service: String, account: String, accessGroup: String) {
        self.service = service
        self.account = account
        self.accessGroup = accessGroup
    }

    private var identity: [CFString: Any] {
        [
            kSecClass: kSecClassGenericPassword,
            kSecAttrService: service,
            kSecAttrAccount: account,
            kSecAttrAccessGroup: accessGroup,
        ]
    }

    func read() throws -> Data? {
        var query = identity
        query[kSecReturnData] = true
        query[kSecMatchLimit] = kSecMatchLimitOne
        var result: CFTypeRef?
        let status = SecItemCopyMatching(query as CFDictionary, &result)
        if status == errSecItemNotFound { return nil }
        guard status == errSecSuccess, let data = result as? Data else {
            throw Failure(code: "keychain", detail: "read status \(status)")
        }
        return data
    }

    func create(_ data: Data) throws {
        guard try read() == nil else { throw Failure(code: "preexisting-keychain", detail: service) }
        var attributes = identity
        attributes[kSecValueData] = data
        attributes[kSecAttrAccessible] = kSecAttrAccessibleAfterFirstUnlockThisDeviceOnly
        let status = SecItemAdd(attributes as CFDictionary, nil)
        guard status == errSecSuccess else { throw Failure(code: "keychain", detail: "add status \(status)") }
    }

    func replace(_ data: Data) throws {
        let status = SecItemUpdate(identity as CFDictionary, [kSecValueData: data] as CFDictionary)
        guard status == errSecSuccess else { throw Failure(code: "keychain", detail: "update status \(status)") }
    }

    func delete() throws {
        let status = SecItemDelete(identity as CFDictionary)
        guard status == errSecSuccess || status == errSecItemNotFound else {
            throw Failure(code: "keychain", detail: "delete status \(status)")
        }
    }
}

private struct Layout {
    let home: String
    let base: String
    let data: String
    let kernel: String
    let threads: String
    let archive: String
    let installer: String
    let config: String
    let workspaces: String
    let uatWorkspace: String
    let plist: String
    let logs: String
    let uat: String

    static func currentUser() throws -> Layout {
        guard let record = getpwuid(getuid()), let home = record.pointee.pw_dir else {
            throw Failure(code: "platform", detail: "target user home unavailable")
        }
        let resolved = String(cString: home)
        let base = resolved + "/Library/Application Support/Tekes"
        let data = resolved + "/.agents"
        return Layout(
            home: resolved,
            base: base,
            data: data,
            kernel: base + "/Kernel",
            threads: data + "/threads",
            archive: data + "/archive",
            installer: base + "/Installer",
            config: data + "/config",
            workspaces: data + "/workspaces",
            uatWorkspace: data + "/runtime/production-uat-workspace",
            plist: resolved + "/Library/LaunchAgents/\(serviceLabel).plist",
            logs: data + "/logs/kernel",
            uat: data + "/runtime/production-uat"
        )
    }
}

private func bootSession() throws -> String {
    let raw = try requireSuccess("/usr/sbin/sysctl", ["-n", "kern.bootsessionuuid"])
    let value = String(decoding: raw, as: UTF8.self).trimmingCharacters(in: .whitespacesAndNewlines).lowercased()
    guard UUID(uuidString: value) != nil else {
        throw Failure(code: "platform", detail: "invalid boot session identity")
    }
    return value
}

private func hostIdentity() throws -> String {
    let raw = try requireSuccess("/usr/sbin/ioreg", ["-rd1", "-c", "IOPlatformExpertDevice"])
    let text = String(decoding: raw, as: UTF8.self)
    let expression = try NSRegularExpression(pattern: #"\"IOPlatformUUID\" = \"([0-9A-Fa-f-]{36})\""#)
    let range = NSRange(text.startIndex..<text.endIndex, in: text)
    guard let match = expression.firstMatch(in: text, range: range),
          let valueRange = Range(match.range(at: 1), in: text),
          UUID(uuidString: String(text[valueRange])) != nil else {
        throw Failure(code: "platform", detail: "host identity unavailable")
    }
    return String(text[valueRange]).lowercased()
}

private func operationDirectory(_ layout: Layout, _ operation: String) -> String {
    layout.uat + "/" + operation
}

private func deterministicOperationID(_ seed: Data) -> String {
    var bytes = Array(SHA256.hash(data: seed).prefix(16))
    bytes[6] = (bytes[6] & 0x0f) | 0x50
    bytes[8] = (bytes[8] & 0x3f) | 0x80
    let hex = bytes.map { String(format: "%02x", $0) }.joined()
    return "\(hex.prefix(8))-\(hex.dropFirst(8).prefix(4))-\(hex.dropFirst(12).prefix(4))-\(hex.dropFirst(16).prefix(4))-\(hex.dropFirst(20))"
}

private func noSymlinkComponents(_ path: String) throws {
    guard path.hasPrefix("/"), URL(fileURLWithPath: path).standardized.path == path else {
        throw Failure(code: "unsafe-path", detail: path)
    }
    var current = ""
    for component in path.split(separator: "/") {
        current += "/" + component
        var status = stat()
        if lstat(current, &status) == 0,
           status.st_mode & S_IFMT == S_IFLNK {
            throw Failure(code: "unsafe-path", detail: "symlink component \(current)")
        }
    }
}

private func directoryNames(_ path: String) throws -> Set<String> {
    guard FileManager.default.fileExists(atPath: path) else { return [] }
    return Set(try FileManager.default.contentsOfDirectory(atPath: path))
}

private func verifyProviderGenerationAuthority(
    _ layout: Layout,
    generation: Int,
    record: Data
) throws {
    let path = layout.data + "/credential-state/provider-secret-generations.json"
    let bytes = try Data(contentsOf: URL(fileURLWithPath: path))
    guard bytes.last == 0x0A,
          !bytes.dropLast().contains(0x0A),
          let root = try JSONSerialization.jsonObject(with: bytes.dropLast()) as? [String: Any],
          Set(root.keys) == Set(["access_group", "credentials", "format", "service"]),
          root["format"] as? Int == 1,
          root["service"] as? String == providerService,
          root["access_group"] as? String == "\(BuildIdentity.teamID).com.tekes.kernel.provider-secrets",
          let credentials = root["credentials"] as? [String: Any],
          let floor = credentials[providerAccount] as? [String: Any],
          Set(floor.keys) == Set(["generation", "record_sha256"]),
          floor["generation"] as? Int == generation,
          floor["record_sha256"] as? String == sha256(record),
          bytes.range(of: Data("material".utf8)) == nil else {
        throw Failure(code: "provider-generation-authority", detail: "generation \(generation)")
    }
}

private func assertOwnedRecoveryLayout(_ layout: Layout, operation: String) throws {
    try noSymlinkComponents(layout.base)
    try noSymlinkComponents(layout.data)
    let installAllowed: Set<String> = ["Kernel", "Installer"]
    let dataAllowed: Set<String> = [
        "archive", "threads", "config", "workspaces", "runtime", "logs", "staging",
        ".create-staging", ".rewrite-trash", "memory", "goals", "jobs", "tool-state",
        "credential-state", "cache", "endpoint-management", "plugins", "skills",
        "settings", "client", "not-in-project",
    ]
    guard try directoryNames(layout.base).isSubset(of: installAllowed),
          try directoryNames(layout.data).isSubset(of: dataAllowed),
          try directoryNames(layout.threads).isSubset(of: [".root-lock"]),
          try directoryNames(layout.archive).isEmpty,
          try directoryNames(layout.config).isSubset(of: ["providers.json", "settings.json"]),
          try directoryNames(layout.workspaces).isSubset(of: ["deployment-uat"]),
          try directoryNames(layout.uatWorkspace).isEmpty,
          try directoryNames(layout.uat).isSubset(of: [operation]) else {
        throw Failure(code: "preexisting-user-data", detail: layout.data)
    }
}

private func installIdentity() throws -> ([String: Any], Data, String) {
    let value: [String: Any] = [
        "access_group": "\(BuildIdentity.teamID).com.tekes.shared.endpoint",
        "client_requirement": BuildIdentity.clientRequirement,
        "format": 1,
        "installer_requirement": "anchor apple generic and identifier com.tekes.kernel.installer",
        "selector_requirement": "anchor apple generic and identifier com.tekes.kernel.selector",
        "supervisor_requirement": "anchor apple generic and identifier com.tekes.kernel.supervisor",
        "team_id": BuildIdentity.teamID,
    ]
    let bytes = try canonical(value)
    return (value, bytes, sha256(bytes))
}

private func authenticatedWrite(
    _ directory: String,
    name: String,
    value: [String: Any],
    secret: Data
) throws {
    let payload = try canonical(value)
    let record = try canonical([
        "format": 1,
        "hmac_sha256": hmacSHA256(payload, secret: secret),
        "payload": value,
    ])
    try durableWrite(directory + "/\(name).authenticated.canonical.json", data: record, mode: 0o600)
}

private func authenticatedRead(
    _ directory: String,
    name: String,
    secret: Data
) throws -> [String: Any] {
    let recordData = try Data(contentsOf: URL(fileURLWithPath: directory + "/\(name).authenticated.canonical.json"))
    guard let record = try decodeCanonical(recordData) as? [String: Any],
          Set(record.keys) == Set(["format", "hmac_sha256", "payload"]),
          record["format"] as? Int == 1,
          let value = record["payload"] as? [String: Any],
          let payload = try? canonical(value),
          record["hmac_sha256"] as? String == hmacSHA256(payload, secret: secret) else {
        throw Failure(code: "operation-authentication", detail: name)
    }
    return value
}

private func authenticatedExists(_ directory: String, name: String) -> Bool {
    FileManager.default.fileExists(atPath: directory + "/\(name).authenticated.canonical.json")
}

private func inputBinding(_ path: String) throws -> [String: Any] {
    try noSymlinkComponents(path)
    var isDirectory: ObjCBool = false
    guard FileManager.default.fileExists(atPath: path, isDirectory: &isDirectory) else {
        throw Failure(code: "missing-input", detail: path)
    }
    if !isDirectory.boolValue {
        let bytes = try Data(contentsOf: URL(fileURLWithPath: path))
        return ["bytes": bytes.count, "path": path, "sha256": sha256(bytes), "type": "file"]
    }
    guard let enumerator = FileManager.default.enumerator(atPath: path) else {
        throw Failure(code: "io", detail: "enumerate \(path)")
    }
    var rows: [[String: Any]] = []
    while let relative = enumerator.nextObject() as? String {
        let full = path + "/" + relative
        try noSymlinkComponents(full)
        var childDirectory: ObjCBool = false
        guard FileManager.default.fileExists(atPath: full, isDirectory: &childDirectory) else { continue }
        if !childDirectory.boolValue {
            let data = try Data(contentsOf: URL(fileURLWithPath: full))
            rows.append(["bytes": data.count, "path": relative, "sha256": sha256(data)])
        }
    }
    rows.sort { ($0["path"] as! String) < ($1["path"] as! String) }
    let bytes = try canonical(rows, lf: false)
    return ["bytes": rows.reduce(0) { $0 + ($1["bytes"] as! Int) }, "path": path, "sha256": sha256(bytes), "type": "directory"]
}

private func boundPath(_ value: Any, name: String) throws -> String {
    guard let binding = value as? [String: Any],
          Set(binding.keys) == Set(["bytes", "path", "sha256", "type"]),
          let path = binding["path"] as? String,
          let digest = binding["sha256"] as? String,
          let kind = binding["type"] as? String,
          path.hasPrefix("/"),
          let current = try? inputBinding(path),
          current["sha256"] as? String == digest,
          current["type"] as? String == kind,
          current["bytes"] as? Int == binding["bytes"] as? Int else {
        throw Failure(code: "input-mismatch", detail: name)
    }
    return path
}

private func arguments(from request: [String: Any]) throws -> Arguments {
    guard let gate = request["gate"] as? Int,
          let origin = request["origin"] as? String,
          let releaseVersion = request["release_version"] as? String,
          let inputs = request["inputs"] as? [String: Any],
          let fixtures = inputs["fixtures"],
          let selector = inputs["selector"],
          let supervisor = inputs["supervisor"],
          let worker = inputs["worker"],
          let helper = inputs["helper"],
          let clientUAT = inputs["client_uat"],
          let runner = inputs["runner"],
          let selectorConformance = inputs["selector_conformance"] else {
        throw Failure(code: "operation-corrupt", detail: "request fields")
    }
    _ = try boundPath(runner, name: "runner")
    return try Arguments(
        gate: gate,
        fixtures: boundPath(fixtures, name: "fixtures"),
        selector: boundPath(selector, name: "selector"),
        supervisor: boundPath(supervisor, name: "supervisor"),
        worker: boundPath(worker, name: "worker"),
        helper: boundPath(helper, name: "helper"),
        clientUAT: boundPath(clientUAT, name: "client_uat"),
        origin: origin,
        releaseVersion: releaseVersion,
        selectorConformance: boundPath(selectorConformance, name: "selector_conformance")
    )
}

private func withOperationLock<T>(_ directory: String, _ body: () throws -> T) throws -> T {
    let path = directory + "/operation.lock"
    let descriptor = open(path, O_CREAT | O_RDWR | O_CLOEXEC, 0o600)
    guard descriptor >= 0 else { throw Failure(code: "io", detail: "operation lock") }
    defer { close(descriptor) }
    guard flock(descriptor, LOCK_EX | LOCK_NB) == 0 else {
        throw Failure(code: "operation-busy", detail: directory)
    }
    return try body()
}

private func randomCredential() throws -> Data {
    var entropy = [UInt8](repeating: 0, count: 32)
    guard SecRandomCopyBytes(kSecRandomDefault, entropy.count, &entropy) == errSecSuccess else {
        throw Failure(code: "keychain", detail: "random generation")
    }
    let alphabet = Array("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_".utf8)
    return Data(entropy.map { alphabet[Int($0 & 63)] })
}

private func writeInstallerOperation(
    _ layout: Layout,
    type: String,
    phase: String,
    identitySHA: String,
    requestSHA: String,
    response: [String: Any]? = nil
) throws {
    var value: [String: Any] = [
        "format": 1,
        "install_identity_sha256": identitySHA,
        "op_id": "uat-\(type)",
        "phase": phase,
        "request_sha256": requestSHA,
        "type": type,
    ]
    if let response { value["response"] = response }
    try durableWrite(layout.installer + "/operation.json", data: try canonical(value), mode: 0o600)
}

private func verifySignature(
    _ path: String,
    teamID: String,
    requirement: String,
    requiredGroups: Set<String>,
    forbiddenGroups: Set<String>
) throws {
    let verified = try run("/usr/bin/codesign", ["--verify", "--strict", "-R", "=\(requirement)", path])
    guard verified.status == 0 else { throw Failure(code: "invalid-signature", detail: path) }
    let detail = try run("/usr/bin/codesign", ["-d", "--verbose=4", "-r-", path])
    let evidence = String(decoding: detail.stderr, as: UTF8.self)
    guard detail.status == 0, evidence.contains("TeamIdentifier=\(teamID)") else {
        throw Failure(code: "invalid-signature", detail: "team mismatch")
    }
    let entitlement = try run("/usr/bin/codesign", ["-d", "--entitlements", ":-", path])
    let text = String(decoding: entitlement.stdout + entitlement.stderr, as: UTF8.self)
    for group in requiredGroups where !text.contains(group) {
        throw Failure(code: "invalid-entitlement", detail: "missing \(group)")
    }
    for group in forbiddenGroups where text.contains(group) {
        throw Failure(code: "invalid-entitlement", detail: "forbidden \(group)")
    }
}

private func platformPreflight(_ layout: Layout) throws {
    let version = ProcessInfo.processInfo.operatingSystemVersion
    guard version.majorVersion >= 15 else { throw Failure(code: "platform", detail: "macOS 15+ required") }
    let df = try requireSuccess("/bin/df", ["-P", layout.home])
    guard String(decoding: df, as: UTF8.self).split(separator: "\n").last?.hasPrefix("/dev/") == true else {
        throw Failure(code: "platform", detail: "local filesystem required")
    }
    let disk = try requireSuccess("/usr/sbin/diskutil", ["info", layout.home])
    guard String(decoding: disk, as: UTF8.self).contains("File System Personality:   APFS") else {
        throw Failure(code: "platform", detail: "APFS required")
    }
}

private func verifySelectorConformance(_ args: Arguments) throws {
    let evidence = try Data(contentsOf: URL(fileURLWithPath: args.selectorConformance))
    _ = try decodeCanonical(evidence)
    let described = try requireSuccess(args.selector, ["describe-conformance"])
    guard let reply = try decodeCanonical(described) as? [String: Any],
          Set(reply.keys) == Set(["architecture", "conformance_sha256", "format", "operation", "version"]),
          reply["architecture"] as? String == "aarch64",
          reply["conformance_sha256"] as? String == sha256(evidence),
          reply["format"] as? Int == 1,
          reply["operation"] as? String == "describe-conformance",
          reply["version"] as? String == args.releaseVersion else {
        throw Failure(code: "selector-conformance", detail: "signed selector does not bind the supplied evidence")
    }
}

private func verifyProductionActors(_ args: Arguments, operationGroup: String) throws {
    let endpointGroup = "\(BuildIdentity.teamID).com.tekes.shared.endpoint"
    let providerGroup = "\(BuildIdentity.teamID).com.tekes.kernel.provider-secrets"
    let selfApp = try currentApplication("tekes-production-uat")
    let clientIdentifier = try identifier(from: BuildIdentity.clientRequirement)
    try validateSignedApplication(selfApp, executable: "tekes-production-uat")
    try validateSignedApplication(args.clientUAT, executable: "tekes-kernel-client-uat")
    try validateSignedApplication(args.supervisor, executable: "tekes-supervisor")
    try verifyEmbeddedProvisioningProfile(
        args.clientUAT,
        identifier: clientIdentifier,
        requiredGroups: [endpointGroup]
    )
    try verifySignature(
        selfApp,
        teamID: BuildIdentity.teamID,
        requirement: "anchor apple generic and identifier com.tekes.kernel.installer",
        requiredGroups: [endpointGroup, providerGroup, operationGroup],
        forbiddenGroups: []
    )
    try verifySignature(args.clientUAT, teamID: BuildIdentity.teamID, requirement: BuildIdentity.clientRequirement, requiredGroups: [endpointGroup], forbiddenGroups: [providerGroup, operationGroup])
    try verifySignature(args.supervisor, teamID: BuildIdentity.teamID, requirement: "anchor apple generic and identifier com.tekes.kernel.supervisor", requiredGroups: [endpointGroup, providerGroup], forbiddenGroups: [operationGroup])
    for (path, identifier) in [(args.selector, "selector"), (args.worker, "worker"), (args.helper, "helper")] {
        try verifySignature(path, teamID: BuildIdentity.teamID, requirement: "anchor apple generic and identifier com.tekes.kernel.\(identifier)", requiredGroups: [], forbiddenGroups: [endpointGroup, providerGroup, operationGroup])
    }
    try verifyClientAdmissionProbe(args, identifier: clientIdentifier)
    try verifySelectorConformance(args)
}

private func canaryRun(_ layout: Layout, session: String) throws -> String {
    guard UUID(uuidString: session) != nil, session == session.lowercased() else {
        throw Failure(code: "client-proof", detail: "session id")
    }
    let ledger = layout.threads + "/\(session)/main.jsonl"
    let data = try Data(contentsOf: URL(fileURLWithPath: ledger))
    guard data.last == 0x0a else { throw Failure(code: "ledger-proof", detail: "missing final LF") }
    var genesisMatches = false
    var runs: [String] = []
    var settledAfterRun = false
    for raw in data.split(separator: 0x0a, omittingEmptySubsequences: true) {
        guard let row = try decodeCanonical(Data(raw) + Data([0x0a])) as? [String: Any],
              let kind = row["kind"] as? String else {
            throw Failure(code: "ledger-proof", detail: "noncanonical ledger")
        }
        if kind == "genesis" { genesisMatches = row["thread"] as? String == session }
        if kind == "run_start", let run = row["run"] as? String {
            runs.append(run)
            settledAfterRun = false
        }
        if kind == "settle", !runs.isEmpty { settledAfterRun = true }
    }
    let unique = Array(Set(runs))
    guard genesisMatches, unique.count == 1, settledAfterRun else {
        throw Failure(code: "ledger-proof", detail: "canary run is not uniquely settled")
    }
    return unique[0]
}

private func retainedDataProof(_ layout: Layout, session: String) throws -> [String: Any] {
    guard UUID(uuidString: session) != nil, session == session.lowercased() else {
        throw Failure(code: "retained-data", detail: "session id")
    }
    var roots: [[String: Any]] = []
    for (name, path) in [
        ("active", layout.threads + "/" + session),
        ("archive", layout.archive + "/" + session),
    ] {
        var isDirectory: ObjCBool = false
        let exists = FileManager.default.fileExists(atPath: path, isDirectory: &isDirectory)
        guard !exists || isDirectory.boolValue else {
            throw Failure(code: "retained-data", detail: "session root is not a directory")
        }
        var files: [[String: Any]] = []
        if exists {
            guard let enumerator = FileManager.default.enumerator(
                at: URL(fileURLWithPath: path),
                includingPropertiesForKeys: [.isRegularFileKey, .isDirectoryKey, .isSymbolicLinkKey],
                options: []
            ) else {
                throw Failure(code: "retained-data", detail: "enumerate \(name)")
            }
            for case let url as URL in enumerator {
                let values = try url.resourceValues(forKeys: [.isRegularFileKey, .isDirectoryKey, .isSymbolicLinkKey])
                guard values.isSymbolicLink != true else {
                    throw Failure(code: "retained-data", detail: "symlink \(url.path)")
                }
                if values.isDirectory == true { continue }
                guard values.isRegularFile == true else {
                    throw Failure(code: "retained-data", detail: "non-regular \(url.path)")
                }
                let data = try Data(contentsOf: url)
                let relative = String(url.path.dropFirst(path.count + 1))
                files.append(["bytes": data.count, "path": relative, "sha256": sha256(data)])
            }
            files.sort { ($0["path"] as! String) < ($1["path"] as! String) }
        }
        roots.append(["exists": exists, "files": files, "name": name])
    }
    guard roots.contains(where: { $0["name"] as? String == "active" && $0["exists"] as? Bool == true }) else {
        throw Failure(code: "retained-data", detail: "active session absent after unarchive")
    }
    let rows = try canonical(roots, lf: false)
    return ["digest": sha256(rows), "format": 1, "roots": roots, "session_id": session]
}

private func launchdPID(_ label: String) throws -> Int32 {
    let output = try requireSuccess("/bin/launchctl", ["print", "gui/\(getuid())/\(label)"])
    let text = String(decoding: output, as: UTF8.self)
    let expression = try NSRegularExpression(pattern: #"(?m)^\s*pid = ([0-9]+)\s*$"#)
    let range = NSRange(text.startIndex..<text.endIndex, in: text)
    guard let match = expression.firstMatch(in: text, range: range),
          let pidRange = Range(match.range(at: 1), in: text),
          let pid = Int32(text[pidRange]), pid > 1 else {
        throw Failure(code: "launchctl", detail: "selector pid unavailable")
    }
    return pid
}

private func selectorLaunchID(_ layout: Layout) throws -> String {
    let output = try requireSuccess(
        layout.kernel + "/selector/bin/tekes-selector",
        ["--install-root", layout.kernel, "status"]
    )
    guard let status = try decodeCanonical(output) as? [String: Any],
          let observation = status["observation"] as? [String: Any],
          let launchID = observation["launch_id"] as? String,
          !launchID.isEmpty else {
        throw Failure(code: "selector-proof", detail: "launch id unavailable")
    }
    return launchID
}

private func processExists(_ pid: Int32) -> Bool {
    kill(pid, 0) == 0 || errno == EPERM
}

private func processIdentity(_ pid: Int32) throws -> String {
    let result = try run("/bin/ps", ["-o", "lstart=", "-p", String(pid)])
    let value = String(decoding: result.stdout, as: UTF8.self)
        .trimmingCharacters(in: .whitespacesAndNewlines)
    guard result.status == 0, result.stderr.isEmpty, !value.isEmpty else {
        throw Failure(code: "process-topology", detail: "process identity \(pid)")
    }
    return value
}

private func processGoneOrReplaced(_ pid: Int32, identity: String) -> Bool {
    guard processExists(pid) else { return true }
    return (try? processIdentity(pid)) != identity
}

private func descendantPIDs(_ root: Int32) throws -> Set<Int32> {
    var seen: Set<Int32> = []
    var pending = [root]
    while let parent = pending.popLast() {
        let result = try run("/usr/bin/pgrep", ["-P", String(parent)])
        if result.status == 1 { continue }
        guard result.status == 0, result.stderr.isEmpty else {
            throw Failure(code: "process-topology", detail: "pgrep descendants")
        }
        for raw in String(decoding: result.stdout, as: UTF8.self).split(whereSeparator: \.isWhitespace) {
            guard let child = Int32(raw), child > 1 else {
                throw Failure(code: "process-topology", detail: "invalid descendant pid")
            }
            if seen.insert(child).inserted { pending.append(child) }
        }
    }
    return seen
}

private func requireListenerOwner(_ pid: Int32) throws {
    let result = try run(
        "/usr/sbin/lsof",
        ["-nP", "-a", "-p", String(pid), "-iTCP:7347", "-sTCP:LISTEN", "-t"]
    )
    let owners = String(decoding: result.stdout, as: UTF8.self)
        .split(whereSeparator: \.isWhitespace)
        .compactMap { Int32($0) }
    guard result.status == 0, result.stderr.isEmpty, owners == [pid] else {
        throw Failure(code: "process-topology", detail: "supervisor does not own production listener")
    }
}

private func recordSelectorCrash(
    _ layout: Layout,
    directory: String,
    operation: String,
    requestSHA: String,
    secret: Data
) throws {
    let beforeProof = try postRebootProcessProof(layout)
    guard let beforeSupervisorPID = beforeProof["supervisor_pid"] as? Int else {
        throw Failure(code: "selector-proof", detail: "supervisor pid unavailable")
    }
    let oldPIDs = try descendantPIDs(Int32(beforeSupervisorPID)).union([Int32(beforeSupervisorPID)])
    let oldTree = try oldPIDs.map { pid in
        ["identity": try processIdentity(pid), "pid": Int(pid)] as [String: Any]
    }.sorted { ($0["pid"] as! Int) < ($1["pid"] as! Int) }
    let beforePID = try launchdPID(serviceLabel)
    let beforeLaunchID = try selectorLaunchID(layout)
    guard kill(beforePID, SIGKILL) == 0 else {
        throw Failure(code: "selector-proof", detail: "SIGKILL failed")
    }
    let deadline = Date().addingTimeInterval(30)
    var afterPID: Int32?
    var afterLaunchID: String?
    while Date() < deadline {
        usleep(200_000)
        if let candidatePID = try? launchdPID(serviceLabel), candidatePID != beforePID,
           let candidateLaunchID = try? selectorLaunchID(layout), candidateLaunchID != beforeLaunchID,
           let proof = try? postRebootProcessProof(layout),
           proof["supervisor_pid"] as? Int != beforeSupervisorPID,
           oldTree.allSatisfy({ row in
               processGoneOrReplaced(Int32(row["pid"] as! Int), identity: row["identity"] as! String)
           }) {
            afterPID = candidatePID
            afterLaunchID = candidateLaunchID
            break
        }
    }
    guard let afterPID, let afterLaunchID else {
        throw Failure(code: "selector-proof", detail: "launchd did not replace selector")
    }
    try authenticatedWrite(
        directory,
        name: "selector-crash",
        value: [
            "after_launch_id": afterLaunchID,
            "after_pid": Int(afterPID),
            "before_launch_id": beforeLaunchID,
            "before_pid": Int(beforePID),
            "before_supervisor_pid": beforeSupervisorPID,
            "boot_session": try bootSession(),
            "format": 1,
            "old_process_tree": oldTree,
            "old_process_tree_reaped": true,
            "operation": operation,
            "request_sha256": requestSHA,
        ],
        secret: secret
    )
}

private func postRebootProcessProof(_ layout: Layout) throws -> [String: Any] {
    let selectorPID = try launchdPID(serviceLabel)
    let children = try requireSuccess("/usr/bin/pgrep", ["-P", String(selectorPID)])
    let childPIDs = String(decoding: children, as: UTF8.self)
        .split(whereSeparator: \.isWhitespace)
        .compactMap { Int32($0) }
    guard childPIDs.count == 1, childPIDs[0] > 1 else {
        throw Failure(code: "process-topology", detail: "selector must own exactly one supervisor")
    }
    try requireListenerOwner(childPIDs[0])
    let lockPath = layout.threads + "/.root-lock"
    let descriptor = open(lockPath, O_RDWR | O_CLOEXEC | O_NOFOLLOW)
    guard descriptor >= 0 else { throw Failure(code: "root-lock", detail: "open") }
    defer { close(descriptor) }
    if flock(descriptor, LOCK_EX | LOCK_NB) == 0 {
        flock(descriptor, LOCK_UN)
        throw Failure(code: "root-lock", detail: "production supervisor is not the holder")
    }
    guard errno == EWOULDBLOCK else { throw Failure(code: "root-lock", detail: "unexpected errno") }
    return ["selector_pid": Int(selectorPID), "supervisor_pid": Int(childPIDs[0])]
}

private func waitForProcessProof(_ layout: Layout) throws -> [String: Any] {
    let deadline = Date().addingTimeInterval(30)
    while Date() < deadline {
        if let proof = try? postRebootProcessProof(layout) { return proof }
        usleep(200_000)
    }
    throw Failure(code: "process-topology", detail: "production service did not become ready")
}

private func install(
    _ args: Arguments,
    _ layout: Layout,
    endpoint: KeychainItem,
    provider: KeychainItem,
    ownershipStarted: inout Bool
) throws -> (String, String) {
    let fm = FileManager.default
    guard !fm.fileExists(atPath: layout.kernel), !fm.fileExists(atPath: layout.plist),
          !fm.fileExists(atPath: layout.config + "/providers.json"),
          !fm.fileExists(atPath: layout.config + "/settings.json"),
          !fm.fileExists(atPath: layout.workspaces + "/deployment-uat/workspace.json") else {
        throw Failure(code: "preexisting-install", detail: layout.kernel)
    }
    let domain = "gui/\(getuid())"
    let existing = try run("/bin/launchctl", ["print", "\(domain)/\(serviceLabel)"])
    guard existing.status != 0 else { throw Failure(code: "preexisting-launchd", detail: serviceLabel) }
    guard try endpoint.read() == nil, try provider.read() == nil else {
        throw Failure(code: "preexisting-keychain", detail: "UAT items")
    }
    ownershipStarted = true
    for path in [layout.base, layout.data, layout.kernel, layout.threads, layout.archive, layout.installer, layout.logs, layout.config, layout.workspaces, layout.workspaces + "/deployment-uat", layout.uatWorkspace] {
        try createDirectory(path)
    }
    try durableWrite(
        layout.config + "/providers.json",
        data: try canonical([
            "format": 1,
            "providers": [[
                "adapter": "responses",
                "credential_key": providerAccount,
                "endpoint": "http://127.0.0.1:7348/v1",
                "id": "provider-uat",
                "models": [[
                    "compact_trigger_tokens": 4096,
                    "context_window_tokens": 8192,
                    "enabled": true,
                    "id": "model-uat",
                ]],
            ]],
            "revision": 1,
        ]),
        mode: 0o600
    )
    try durableWrite(
        layout.config + "/settings.json",
        data: try canonical([
            "default_model": "model-uat",
            "default_provider": "provider-uat",
            "format": 1,
            "revision": 1,
        ]),
        mode: 0o600
    )
    try durableWrite(
        layout.workspaces + "/deployment-uat/workspace.json",
        data: try canonical([
            "folders": [["id": "folder-0001", "path": layout.uatWorkspace]],
            "format": 1,
            "id": "deployment-uat",
            "name": "Deployment UAT",
            "policy": ["model": "model-uat", "network": true, "provider": "provider-uat"],
            "revision": 1,
        ]),
        mode: 0o600
    )
    guard !fm.fileExists(atPath: layout.threads + "/threads") else {
        throw Failure(code: "nested-threads", detail: layout.threads)
    }
    let (_, identityBytes, identitySHA) = try installIdentity()
    let requestSHA = sha256(Data(CommandLine.arguments.joined(separator: "\u{0}").utf8))
    try writeInstallerOperation(layout, type: "install", phase: "prepared", identitySHA: identitySHA, requestSHA: requestSHA)
    try durableWrite(layout.installer + "/install-identity.json", data: identityBytes, mode: 0o600)
    try durableWrite(layout.threads + "/.root-lock", data: Data(), mode: 0o600)
    try writeInstallerOperation(layout, type: "install", phase: "identity-published", identitySHA: identitySHA, requestSHA: requestSHA)
    let endpointBytes = try randomCredential()
    try endpoint.create(endpointBytes)
    let providerMaterial = String(decoding: try randomCredential(), as: UTF8.self)
    let providerBytes = try canonical(["format": 1, "generation": 1, "material": providerMaterial, "state": "active"], lf: false)
    try provider.create(providerBytes)
    try writeInstallerOperation(layout, type: "install", phase: "credential-published", identitySHA: identitySHA, requestSHA: requestSHA)
    let bundle = layout.kernel + "/bundles/\(args.releaseVersion)"
    let binaries = bundle + "/bin"
    try createDirectory(binaries)
    var rows: [[String: Any]] = []
    let supervisorApp = bundle + "/apps/TekesKernelSupervisor.app"
    var supervisorRows = try copySignedApplication(
        args.supervisor,
        supervisorApp,
        executable: "tekes-supervisor"
    )
    for index in supervisorRows.indices {
        supervisorRows[index]["path"] =
            "apps/TekesKernelSupervisor.app/" + (supervisorRows[index]["path"] as! String)
    }
    rows.append(contentsOf: supervisorRows)
    for (source, name) in [(args.helper, "tekes-helper"), (args.worker, "tekes-worker")] {
        let destination = binaries + "/\(name)"
        var row = try copyExecutable(source, destination)
        row["path"] = "bin/\(name)"
        rows.append(row)
    }
    let registry = try Data(contentsOf: URL(fileURLWithPath: args.fixtures + "/authority-registry.canonical.json"))
    _ = try decodeCanonical(registry)
    let manifest: [String: Any] = [
        "architectures": ["aarch64"],
        "compatibility": ["authority_registry_sha256": sha256(registry), "reader_profile": "v1", "writer_profile": "v1"],
        "files": rows,
        "format": 1,
        "minimum_os": "15.0",
        "signing": ["requirement": "anchor apple generic and identifier com.tekes.kernel.supervisor", "team_id": BuildIdentity.teamID],
        "version": args.releaseVersion,
    ]
    let manifestBytes = try canonical(manifest)
    try durableWrite(bundle + "/manifest.canonical.json", data: manifestBytes, mode: 0o600)
    let selectorDirectory = layout.kernel + "/selector"
    try createDirectory(selectorDirectory + "/bin")
    _ = try copyExecutable(args.selector, selectorDirectory + "/bin/tekes-selector")
    try createDirectory(selectorDirectory + "/operations")
    try createDirectory(selectorDirectory + "/observations")
    try writeInstallerOperation(layout, type: "install", phase: "bundles-published", identitySHA: identitySHA, requestSHA: requestSHA)
    let selection = try canonical(["format": 1, "generation": 1, "selection": ["manifest_sha256": sha256(manifestBytes), "version": args.releaseVersion]])
    try durableWrite(selectorDirectory + "/current.json", data: selection, mode: 0o600)
    try durableWrite(selectorDirectory + "/previous.json", data: try canonical(["format": 1, "generation": 1]), mode: 0o600)
    guard symlink("../bundles/\(args.releaseVersion)", selectorDirectory + "/active") == 0 else {
        throw Failure(code: "io", detail: "active symlink")
    }
    try syncDirectory(selectorDirectory)
    try writeInstallerOperation(layout, type: "install", phase: "selection-published", identitySHA: identitySHA, requestSHA: requestSHA)
    let template = try String(contentsOfFile: args.fixtures + "/com.tekes.kernel.supervisor.plist", encoding: .utf8)
    let selector = selectorDirectory + "/bin/tekes-selector"
    let rendered = template
        .replacingOccurrences(of: "@SELECTOR@", with: xmlEscaped(selector))
        .replacingOccurrences(of: "@INSTALL_ROOT@", with: xmlEscaped(layout.kernel))
        .replacingOccurrences(of: "@STORAGE_ROOT@", with: xmlEscaped(layout.threads))
        .replacingOccurrences(of: "@STDOUT@", with: xmlEscaped(layout.logs + "/stdout.log"))
        .replacingOccurrences(of: "@STDERR@", with: xmlEscaped(layout.logs + "/stderr.log"))
    let unresolved = ["@SELECTOR@", "@INSTALL_ROOT@", "@STORAGE_ROOT@", "@STDOUT@", "@STDERR@"]
    guard !unresolved.contains(where: rendered.contains), !rendered.contains("threads/threads") else {
        throw Failure(code: "invalid-plist", detail: "unresolved token or nested threads")
    }
    try durableWrite(layout.plist, data: Data(rendered.utf8), mode: 0o644)
    try writeInstallerOperation(layout, type: "install", phase: "plist-published", identitySHA: identitySHA, requestSHA: requestSHA)
    let started = try run("/bin/launchctl", ["bootstrap", domain, layout.plist])
    guard started.status == 0 else { throw Failure(code: "launchctl", detail: "bootstrap") }
    try writeInstallerOperation(layout, type: "install", phase: "service-started", identitySHA: identitySHA, requestSHA: requestSHA)
    try writeInstallerOperation(layout, type: "install", phase: "closed", identitySHA: identitySHA, requestSHA: requestSHA, response: ["format": 1, "operation": "install", "service": "running"])
    return (providerMaterial, identitySHA)
}

private func invokeClient(
    _ args: Arguments,
    _ layout: Layout,
    operation: String,
    phase: String,
    sessionID: String? = nil
) throws -> [String: Any] {
    var argv = [
        "--protocol", protocolName,
        "--gate", String(args.gate),
        "--operation", operation,
        "--phase", phase,
        "--origin", args.origin,
        "--release-version", args.releaseVersion,
        "--install-root", layout.kernel,
        "--storage-root", layout.threads,
        "--selector", layout.kernel + "/selector/bin/tekes-selector",
    ]
    if let sessionID { argv.append(contentsOf: ["--session-id", sessionID]) }
    let output = try requireSuccess(
        applicationExecutable(args.clientUAT, "tekes-kernel-client-uat"),
        argv
    )
    guard let value = try decodeCanonical(output) as? [String: Any],
          value["format"] as? Int == 1,
          value["gate"] as? Int == args.gate,
          value["phase"] as? String == phase else {
        throw Failure(code: "client-protocol", detail: phase)
    }
    let common = Set(["format", "gate", "phase"])
    let fields: Set<String>
    switch phase {
    case "post-reboot": fields = common.union(["client_driver", "endpoint_ready"])
    case "session": fields = common.union(["archive", "credential_acl", "retained_data", "session_id", "unarchive"])
    case "provider-initial", "provider-rotated": fields = common.union(["models_ready", "provider_advertised"])
    case "provider-revoked": fields = common.union(["provider_advertised", "typed_not_ready"])
    default: throw Failure(code: "client-protocol", detail: "unknown phase")
    }
    guard Set(value.keys) == fields else {
        throw Failure(code: "client-protocol", detail: "unexpected fields for \(phase)")
    }
    return value
}

private func truth(_ value: [String: Any], _ key: String) throws {
    guard value[key] as? Bool == true else { throw Failure(code: "client-proof", detail: key) }
}

private func assertSecretAbsent(_ layout: Layout, material: String) throws {
    let needle = Data(material.utf8)
    for root in [layout.base, layout.logs] where FileManager.default.fileExists(atPath: root) {
        let enumerator = FileManager.default.enumerator(atPath: root)
        while let relative = enumerator?.nextObject() as? String {
            let path = root + "/" + relative
            if let data = try? Data(contentsOf: URL(fileURLWithPath: path)), data.range(of: needle) != nil {
                throw Failure(code: "secret-leak", detail: relative)
            }
        }
    }
}

private func contractEvidence() throws -> Data {
    try canonical([
        "client_argv": ["--protocol", protocolName, "--gate", "72|76", "--operation", "UUID", "--phase", "PHASE", "--origin", "http://127.0.0.1:7347", "--release-version", "1.0.0", "--install-root", "ABSOLUTE", "--storage-root", "ABSOLUTE", "--selector", "ABSOLUTE"],
        "client_phases": ["post-reboot", "session", "provider-initial", "provider-rotated", "provider-revoked"],
        "client_probe": [
            "argv": ["--describe-contract"],
            "result": [
                "format": 1,
                "identifier": "com.tekesapps.TekesUI",
                "protocol": protocolName,
                "release_version": "1.0.0",
            ],
        ],
        "client_phase_argv_suffix": [
            "post-reboot": [],
            "provider-initial": ["--session-id", "UUID"],
            "provider-revoked": ["--session-id", "UUID"],
            "provider-rotated": ["--session-id", "UUID"],
            "session": [],
        ],
        "client_result_fields": [
            "post-reboot": ["format", "gate", "phase", "client_driver", "endpoint_ready"],
            "provider-active": ["format", "gate", "phase", "models_ready", "provider_advertised"],
            "provider-revoked": ["format", "gate", "phase", "provider_advertised", "typed_not_ready"],
            "session": ["format", "gate", "phase", "archive", "credential_acl", "retained_data", "session_id", "unarchive"],
        ],
        "client_rpc_ids": [
            "archive": "uat-<operation>-archive",
            "create": "<operation>",
            "prompt": "uat-<operation>-prompt",
            "unarchive": "uat-<operation>-unarchive",
            "workspace": "uat-<operation>-workspace",
        ],
        "format": 1,
        "operation_argv": [
            "acknowledge": ["--mode", "acknowledge", "--operation", "UUID"],
            "consume": ["--mode", "consume", "--operation", "UUID"],
            "resume": ["--mode", "resume", "--operation", "UUID"],
            "verify": ["--mode", "verify", "--operation", "UUID"],
        ],
        "operation_exits": [
            "acknowledge": 0,
            "consume": 0,
            "describe": 0,
            "prepare": 75,
            "resume": 0,
            "verify": 0,
        ],
        "operation_phases": [
            "prepare": ["preparing", "installing", "crash-proved", "prepared"],
            "resume": ["resume-process-proved", "resume-client-ready", "resume-session-proved", "resume-canary-attested", "resume-provider-initial", "resume-provider-rotated", "resume-provider-revoked", "resumed"],
            "consume": ["consume-prepared", "consume-service-stopped", "consume-credentials-deleted", "consume-plist-removed", "consume-binaries-removed", "consume-resume-agent-removed", "consume-data-verified", "consumed"],
        ],
        "protocol": protocolName,
        "runner_argv": ["--mode", "prepare", "--gate", "72|76", "--fixtures", "ABSOLUTE", "--selector", "ABSOLUTE", "--supervisor", "ABSOLUTE", "--worker", "ABSOLUTE", "--helper", "ABSOLUTE", "--client-uat", "ABSOLUTE", "--origin", "http://127.0.0.1:7347", "--release-version", "1.0.0", "--selector-conformance", "ABSOLUTE"],
        "supervisor_argv": ["--install-root", "ABSOLUTE", "--storage-root", "ABSOLUTE", "--listen", "127.0.0.1:7347", "--selected-version", "1.0.0", "--selector-generation", "1", "--launch-id", "1-1-0123456789abcdef0123456789abcdef", "--manifest-sha256", String(repeating: "0", count: 64), "--bootstrap-status-fd", "3", "--authority-registry-sha256", "f1f084f11ee379fd19ff0b62db653e245a088f7055f2146a5e1adf49decf5469", "--launcher-lifetime-fd", "4"],
        "uat_profile": ["endpoint": "http://127.0.0.1:7348/v1", "model_id": "model-uat", "provider_id": "provider-uat", "workspace_id": "deployment-uat"],
    ])
}

private func operationState(
    _ directory: String,
    phase: String,
    requestSHA: String,
    secret: Data
) throws {
    try authenticatedWrite(
        directory,
        name: "state",
        value: ["format": 1, "phase": phase, "request_sha256": requestSHA],
        secret: secret
    )
}

private func renderResumeAgent(
    label: String,
    runner: String,
    operation: String,
    stdout: String,
    stderr: String
) -> Data {
    Data("""
    <?xml version="1.0" encoding="UTF-8"?>
    <!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
    <plist version="1.0"><dict>
    <key>Label</key><string>\(xmlEscaped(label))</string>
    <key>LimitLoadToSessionType</key><string>Aqua</string>
    <key>ProgramArguments</key><array>
    <string>\(xmlEscaped(runner))</string><string>--mode</string><string>resume</string>
    <string>--operation</string><string>\(xmlEscaped(operation))</string>
    </array>
    <key>RunAtLoad</key><true/>
    <key>KeepAlive</key><dict><key>SuccessfulExit</key><false/></dict>
    <key>ThrottleInterval</key><integer>5</integer>
    <key>StandardOutPath</key><string>\(xmlEscaped(stdout))</string>
    <key>StandardErrorPath</key><string>\(xmlEscaped(stderr))</string>
    </dict></plist>
    """.utf8)
}

private func prepareFingerprint(_ args: Arguments) throws -> String {
    let selfApp = try currentApplication("tekes-production-uat")
    let value: [String: Any] = [
        "client_uat": try inputBinding(args.clientUAT),
        "fixtures": try inputBinding(args.fixtures),
        "format": 1,
        "gate": args.gate,
        "helper": try inputBinding(args.helper),
        "origin": args.origin,
        "release_version": args.releaseVersion,
        "runner_source": try inputBinding(selfApp),
        "selector": try inputBinding(args.selector),
        "selector_conformance": try inputBinding(args.selectorConformance),
        "supervisor": try inputBinding(args.supervisor),
        "worker": try inputBinding(args.worker),
    ]
    return sha256(try canonical(value, lf: false))
}

private func cleanupPreparingInstall(
    _ layout: Layout,
    request: [String: Any],
    endpoint: KeychainItem,
    provider: KeychainItem
) throws {
    try assertOwnedRecoveryLayout(layout, operation: request["operation"] as! String)
    let domain = "gui/\(getuid())"
    let target = "\(domain)/\(serviceLabel)"
    let current = try run("/bin/launchctl", ["print", target])
    if current.status == 0 {
        let stopped = try run("/bin/launchctl", ["bootout", target])
        guard stopped.status == 0 else { throw Failure(code: "launchctl", detail: "prepare recovery bootout") }
    }
    try endpoint.delete()
    try provider.delete()
    let exactFiles = [
        layout.plist,
        layout.config + "/providers.json",
        layout.config + "/settings.json",
        layout.workspaces + "/deployment-uat/workspace.json",
        layout.threads + "/.root-lock",
        request["resume_plist"] as? String,
    ].compactMap { $0 }
    for path in exactFiles where FileManager.default.fileExists(atPath: path) {
        try FileManager.default.removeItem(atPath: path)
        try syncDirectory(URL(fileURLWithPath: path).deletingLastPathComponent().path)
    }
    let runtimeDirectories = [
        layout.kernel,
        layout.uatWorkspace,
        layout.data + "/staging",
        layout.data + "/.create-staging",
        layout.data + "/.rewrite-trash",
        layout.data + "/memory",
        layout.data + "/goals",
        layout.data + "/jobs",
        layout.data + "/tool-state",
        layout.data + "/credential-state",
        layout.data + "/cache",
        layout.data + "/endpoint-management",
    ]
    for path in runtimeDirectories where FileManager.default.fileExists(atPath: path) {
        try FileManager.default.removeItem(atPath: path)
        try syncDirectory(URL(fileURLWithPath: path).deletingLastPathComponent().path)
    }
}

private func prepareResult(_ request: [String: Any]) throws -> Data {
    try canonical([
        "format": 1,
        "gate": request["gate"]!,
        "operation": request["operation"]!,
        "phase": "reboot-required",
        "pre_boot_session": request["pre_boot_session"]!,
        "resume_label": request["resume_label"]!,
    ])
}

private func prepareProduction(_ args: Arguments) throws -> Data {
    let layout = try Layout.currentUser()
    try platformPreflight(layout)
    let endpointGroup = "\(BuildIdentity.teamID).com.tekes.shared.endpoint"
    let providerGroup = "\(BuildIdentity.teamID).com.tekes.kernel.provider-secrets"
    let operationGroup = "\(BuildIdentity.teamID).com.tekes.kernel.production-uat"
    try verifyProductionActors(args, operationGroup: operationGroup)
    let endpoint = KeychainItem(service: endpointService, account: endpointAccount, accessGroup: endpointGroup)
    let provider = KeychainItem(service: providerService, account: providerAccount, accessGroup: providerGroup)
    let fingerprint = try prepareFingerprint(args)
    let preBootSession = try bootSession()
    let currentHost = try hostIdentity()
    let operation = deterministicOperationID(try canonical([
        "format": 1,
        "host_id": currentHost,
        "prepare_sha256": fingerprint,
    ], lf: false))
    let directory = operationDirectory(layout, operation)
    let operationItem = KeychainItem(service: operationService, account: operation, accessGroup: operationGroup)
    let baseAlreadyExists = FileManager.default.fileExists(atPath: layout.base)
        || FileManager.default.fileExists(atPath: layout.data)
    try noSymlinkComponents(layout.base)
    try noSymlinkComponents(layout.data)
    let secret: Data
    if let existing = try operationItem.read() {
        secret = existing
    } else {
        guard !baseAlreadyExists else {
            throw Failure(code: "preexisting-user-data", detail: layout.data)
        }
        secret = try randomCredential()
        try operationItem.create(secret)
    }
    try createDirectory(directory)
    let selfApp = try currentApplication("tekes-production-uat")
    let runnerApp = directory + "/TekesProductionUAT.app"
    _ = try copySignedApplication(
        selfApp,
        runnerApp,
        executable: "tekes-production-uat"
    )
    let runner = try applicationExecutable(runnerApp, "tekes-production-uat")
    let label = "com.tekes.kernel.production-uat.\(operation)"
    let resumePlist = layout.home + "/Library/LaunchAgents/\(label).plist"
    let (_, _, identitySHA) = try installIdentity()
    let inputs: [String: Any] = [
        "client_uat": try inputBinding(args.clientUAT),
        "fixtures": try inputBinding(args.fixtures),
        "helper": try inputBinding(args.helper),
        "runner": try inputBinding(runnerApp),
        "selector": try inputBinding(args.selector),
        "selector_conformance": try inputBinding(args.selectorConformance),
        "supervisor": try inputBinding(args.supervisor),
        "worker": try inputBinding(args.worker),
    ]
    let proposedRequest: [String: Any] = [
        "format": 1,
        "gate": args.gate,
        "host_id": currentHost,
        "inputs": inputs,
        "install_identity_sha256": identitySHA,
        "operation": operation,
        "origin": args.origin,
        "pre_boot_session": preBootSession,
        "prepare_sha256": fingerprint,
        "release_version": args.releaseVersion,
        "resume_label": label,
        "resume_plist": resumePlist,
    ]
    let request: [String: Any]
    if authenticatedExists(directory, name: "request") {
        let existing = try authenticatedRead(directory, name: "request", secret: secret)
        guard existing["prepare_sha256"] as? String == fingerprint,
              existing["host_id"] as? String == currentHost,
              existing["operation"] as? String == operation,
              existing["gate"] as? Int == args.gate else {
            throw Failure(code: "operation-mismatch", detail: "prepare request")
        }
        request = existing
    } else {
        request = proposedRequest
        try authenticatedWrite(directory, name: "request", value: proposedRequest, secret: secret)
    }
    let requestSHA = sha256(try canonical(request))
    var phase = authenticatedExists(directory, name: "state")
        ? (try authenticatedRead(directory, name: "state", secret: secret)["phase"] as? String ?? "")
        : ""
    if phase == "resumed" || phase.hasPrefix("resume-") || phase.hasPrefix("consume-") || phase == "consumed" {
        return try prepareResult(request)
    }
    if baseAlreadyExists { try assertOwnedRecoveryLayout(layout, operation: operation) }
    if phase == "prepared" { return try prepareResult(request) }
    if phase == "crash-proved" {
        if !FileManager.default.fileExists(atPath: resumePlist) {
            try durableWrite(
                resumePlist,
                data: renderResumeAgent(
                    label: label,
                    runner: runner,
                    operation: operation,
                    stdout: directory + "/resume.stdout.log",
                    stderr: directory + "/resume.stderr.log"
                ),
                mode: 0o644
            )
        }
        try operationState(directory, phase: "prepared", requestSHA: requestSHA, secret: secret)
        return try prepareResult(request)
    }
    if phase.isEmpty {
        try operationState(directory, phase: "preparing", requestSHA: requestSHA, secret: secret)
        phase = "preparing"
    }
    guard ["preparing", "installing"].contains(phase) else {
        throw Failure(code: "operation-corrupt", detail: "unexpected prepare phase \(phase)")
    }
    if baseAlreadyExists || phase == "installing" {
        try cleanupPreparingInstall(layout, request: request, endpoint: endpoint, provider: provider)
    }
    try operationState(directory, phase: "installing", requestSHA: requestSHA, secret: secret)
    var ownershipStarted = false
    let (_, installedIdentitySHA) = try install(
        args,
        layout,
        endpoint: endpoint,
        provider: provider,
        ownershipStarted: &ownershipStarted
    )
    guard installedIdentitySHA == identitySHA else {
        throw Failure(code: "operation-mismatch", detail: "install identity")
    }
    try recordSelectorCrash(
        layout,
        directory: directory,
        operation: operation,
        requestSHA: requestSHA,
        secret: secret
    )
    try operationState(directory, phase: "crash-proved", requestSHA: requestSHA, secret: secret)
    try durableWrite(
        resumePlist,
        data: renderResumeAgent(
            label: label,
            runner: runner,
            operation: operation,
            stdout: directory + "/resume.stdout.log",
            stderr: directory + "/resume.stderr.log"
        ),
        mode: 0o644
    )
    try operationState(directory, phase: "prepared", requestSHA: requestSHA, secret: secret)
    return try prepareResult(request)
}

private func operationContext(_ operation: String) throws -> (Layout, String, KeychainItem, Data, [String: Any], String) {
    let layout = try Layout.currentUser()
    let directory = operationDirectory(layout, operation)
    let group = "\(BuildIdentity.teamID).com.tekes.kernel.production-uat"
    let item = KeychainItem(service: operationService, account: operation, accessGroup: group)
    guard let secret = try item.read() else { throw Failure(code: "operation-missing", detail: operation) }
    let request = try authenticatedRead(directory, name: "request", secret: secret)
    let currentHost = try hostIdentity()
    guard Set(request.keys) == Set(["format", "gate", "host_id", "inputs", "install_identity_sha256", "operation", "origin", "pre_boot_session", "prepare_sha256", "release_version", "resume_label", "resume_plist"]),
          request["format"] as? Int == 1,
          request["operation"] as? String == operation,
          request["host_id"] as? String == currentHost else {
        throw Failure(code: "operation-mismatch", detail: operation)
    }
    guard let inputs = request["inputs"] as? [String: Any],
          let runnerBinding = inputs["runner"],
          let runnerApp = try? boundPath(runnerBinding, name: "runner"),
          let currentApp = try? currentApplication("tekes-production-uat"),
          currentApp == runnerApp else {
        throw Failure(code: "operation-mismatch", detail: "runner build")
    }
    return (layout, directory, item, secret, request, sha256(try canonical(request)))
}

private func expect(_ value: [String: Any], _ key: String, _ expected: Bool) throws {
    guard value[key] as? Bool == expected else { throw Failure(code: "client-proof", detail: key) }
}

private func resumeProduction(_ operation: String) throws {
    let (layout, directory, _, secret, request, requestSHA) = try operationContext(operation)
    try withOperationLock(directory) {
        let state = try authenticatedRead(directory, name: "state", secret: secret)
        var phase = state["phase"] as? String ?? ""
        if phase == "resumed" {
            if let plist = request["resume_plist"] as? String { try durableRemove(plist) }
            return
        }
        let currentBoot = try bootSession()
        guard state["request_sha256"] as? String == requestSHA,
              request["pre_boot_session"] as? String != currentBoot else {
            throw Failure(code: "reboot-not-observed", detail: operation)
        }
        if phase == "crash-proved" {
            guard let plist = request["resume_plist"] as? String,
                  FileManager.default.fileExists(atPath: plist) else {
                throw Failure(code: "evidence-unavailable", detail: "resume agent was not published")
            }
            try operationState(directory, phase: "prepared", requestSHA: requestSHA, secret: secret)
            phase = "prepared"
        }
        let attemptRecord = authenticatedExists(directory, name: "resume-attempt")
            ? try authenticatedRead(directory, name: "resume-attempt", secret: secret)
            : ["attempts": 0, "format": 1]
        let attempts = attemptRecord["attempts"] as? Int ?? 0
        guard attempts < 5 else { return }
        try authenticatedWrite(
            directory,
            name: "resume-attempt",
            value: ["attempts": attempts + 1, "format": 1],
            secret: secret
        )
        let args = try arguments(from: request)
        let operationGroup = "\(BuildIdentity.teamID).com.tekes.kernel.production-uat"
        try verifyProductionActors(args, operationGroup: operationGroup)
        let crash = try authenticatedRead(directory, name: "selector-crash", secret: secret)
        guard Set(crash.keys) == Set([
            "after_launch_id", "after_pid", "before_launch_id", "before_pid",
            "before_supervisor_pid", "boot_session", "format", "old_process_tree",
            "old_process_tree_reaped", "operation", "request_sha256",
        ]),
              crash["operation"] as? String == operation,
              crash["request_sha256"] as? String == requestSHA,
              crash["boot_session"] as? String == request["pre_boot_session"] as? String,
              crash["before_pid"] as? Int != crash["after_pid"] as? Int,
              crash["before_launch_id"] as? String != crash["after_launch_id"] as? String,
              crash["old_process_tree_reaped"] as? Bool == true else {
            throw Failure(code: "selector-proof", detail: "crash witness mismatch")
        }
        if phase == "prepared" {
            let processProof = try waitForProcessProof(layout)
            try authenticatedWrite(directory, name: "process-proof", value: processProof, secret: secret)
            try operationState(directory, phase: "resume-process-proved", requestSHA: requestSHA, secret: secret)
            phase = "resume-process-proved"
        }
        if phase == "resume-process-proved" {
            let postReboot = try invokeClient(args, layout, operation: operation, phase: "post-reboot")
            try expect(postReboot, "endpoint_ready", true)
            guard postReboot["client_driver"] as? String == ".tekes" else {
                throw Failure(code: "client-proof", detail: "client_driver")
            }
            try operationState(directory, phase: "resume-client-ready", requestSHA: requestSHA, secret: secret)
            phase = "resume-client-ready"
        }
        let processProof = try authenticatedRead(directory, name: "process-proof", secret: secret)
        var result: [String: Any]
        if args.gate == 72 {
            guard phase == "resume-client-ready" else {
                throw Failure(code: "operation-corrupt", detail: "Gate 72 resume phase \(phase)")
            }
            result = ["embedded_build": args.releaseVersion, "format": 1, "gate": 72, "launchd_restart": true, "nested_threads_absent": true, "no_orphans": true, "root_lock_released": true, "selector_sigkill": true, "target_user_login": true]
        } else {
            if phase == "resume-client-ready" {
                let session = try invokeClient(args, layout, operation: operation, phase: "session")
                for key in ["archive", "credential_acl", "retained_data", "unarchive"] { try truth(session, key) }
                guard let sessionID = session["session_id"] as? String,
                      UUID(uuidString: sessionID) != nil,
                      sessionID == sessionID.lowercased() else {
                    throw Failure(code: "client-proof", detail: "session_id")
                }
                try authenticatedWrite(directory, name: "session", value: session, secret: secret)
                let retained = try retainedDataProof(layout, session: sessionID)
                try authenticatedWrite(directory, name: "retained-data", value: retained, secret: secret)
                try operationState(directory, phase: "resume-session-proved", requestSHA: requestSHA, secret: secret)
                phase = "resume-session-proved"
            }
            let session = try authenticatedRead(directory, name: "session", secret: secret)
            guard let sessionID = session["session_id"] as? String else {
                throw Failure(code: "operation-corrupt", detail: "session record")
            }
            if phase == "resume-session-proved" {
                let runID = try canaryRun(layout, session: sessionID)
                _ = try requireSuccess(layout.kernel + "/selector/bin/tekes-selector", ["--install-root", layout.kernel, "attest-canary", "--version", args.releaseVersion, "--session", sessionID, "--run", runID, "--storage-root", layout.threads])
                try operationState(directory, phase: "resume-canary-attested", requestSHA: requestSHA, secret: secret)
                phase = "resume-canary-attested"
            }
            let runID = try canaryRun(layout, session: sessionID)
            _ = runID
            let providerGroup = "\(BuildIdentity.teamID).com.tekes.kernel.provider-secrets"
            let provider = KeychainItem(service: providerService, account: providerAccount, accessGroup: providerGroup)
            if phase == "resume-canary-attested" {
                guard let record = try provider.read(),
                      let value = try JSONSerialization.jsonObject(with: record) as? [String: Any],
                      value["format"] as? Int == 1,
                      value["generation"] as? Int == 1,
                      value["state"] as? String == "active",
                      let material = value["material"] as? String else {
                    throw Failure(code: "keychain", detail: "provider initial generation")
                }
                let initial = try invokeClient(args, layout, operation: operation, phase: "provider-initial", sessionID: sessionID)
                try expect(initial, "models_ready", true)
                try expect(initial, "provider_advertised", true)
                try assertSecretAbsent(layout, material: material)
                try verifyProviderGenerationAuthority(layout, generation: 1, record: record)
                try operationState(directory, phase: "resume-provider-initial", requestSHA: requestSHA, secret: secret)
                phase = "resume-provider-initial"
            }
            if phase == "resume-provider-initial" {
                guard let record = try provider.read(),
                      let value = try JSONSerialization.jsonObject(with: record) as? [String: Any],
                      let generation = value["generation"] as? Int,
                      value["state"] as? String == "active" else {
                    throw Failure(code: "keychain", detail: "provider rotation generation")
                }
                let material: String
                if generation == 1 {
                    material = String(decoding: try randomCredential(), as: UTF8.self)
                    try provider.replace(try canonical(["format": 1, "generation": 2, "material": material, "state": "active"], lf: false))
                } else if generation == 2, let existing = value["material"] as? String {
                    material = existing
                } else {
                    throw Failure(code: "keychain", detail: "provider rotation CAS")
                }
                let rotated = try invokeClient(args, layout, operation: operation, phase: "provider-rotated", sessionID: sessionID)
                try expect(rotated, "models_ready", true)
                try expect(rotated, "provider_advertised", true)
                try assertSecretAbsent(layout, material: material)
                guard let rotatedRecord = try provider.read() else {
                    throw Failure(code: "keychain", detail: "provider rotated record")
                }
                try verifyProviderGenerationAuthority(layout, generation: 2, record: rotatedRecord)
                try operationState(directory, phase: "resume-provider-rotated", requestSHA: requestSHA, secret: secret)
                phase = "resume-provider-rotated"
            }
            if phase == "resume-provider-rotated" {
                guard let record = try provider.read(),
                      let value = try JSONSerialization.jsonObject(with: record) as? [String: Any],
                      let generation = value["generation"] as? Int else {
                    throw Failure(code: "keychain", detail: "provider revocation generation")
                }
                if generation == 2, value["state"] as? String == "active" {
                    try provider.replace(try canonical(["format": 1, "generation": 3, "state": "revoked"], lf: false))
                } else if !(generation == 3 && value["state"] as? String == "revoked") {
                    throw Failure(code: "keychain", detail: "provider revocation CAS")
                }
                let revoked = try invokeClient(args, layout, operation: operation, phase: "provider-revoked", sessionID: sessionID)
                try expect(revoked, "provider_advertised", false)
                try expect(revoked, "typed_not_ready", true)
                guard let revokedRecord = try provider.read() else {
                    throw Failure(code: "keychain", detail: "provider revoked record")
                }
                try verifyProviderGenerationAuthority(layout, generation: 3, record: revokedRecord)
                try operationState(directory, phase: "resume-provider-revoked", requestSHA: requestSHA, secret: secret)
                phase = "resume-provider-revoked"
            }
            guard phase == "resume-provider-revoked" else {
                throw Failure(code: "operation-corrupt", detail: "Gate 76 resume phase \(phase)")
            }
            result = ["archive": true, "canary_attested": true, "client_driver": ".tekes", "credential_acl": true, "embedded_build": args.releaseVersion, "format": 1, "gate": 76, "install": true, "nested_threads_absent": true, "origin": args.origin, "provider_secret_acl": true, "provider_secret_redacted": true, "provider_secret_revocation": true, "provider_secret_rotation": true, "reboot_after_target_user_login": true, "retained_data": true, "unarchive": true, "uninstall": false]
        }
        let evidence: [String: Any] = [
            "boot_session": try bootSession(),
            "format": 1,
            "gate": args.gate,
            "host_id": try hostIdentity(),
            "operation": operation,
            "process": processProof,
            "request_sha256": requestSHA,
            "result": result,
        ]
        try authenticatedWrite(directory, name: "evidence", value: evidence, secret: secret)
        try operationState(directory, phase: "resumed", requestSHA: requestSHA, secret: secret)
        if let plist = request["resume_plist"] as? String { try durableRemove(plist) }
    }
}

private func verifiedEvidence(_ operation: String) throws -> (Layout, String, KeychainItem, Data, [String: Any], [String: Any], String) {
    let (layout, directory, item, secret, request, requestSHA) = try operationContext(operation)
    let state = try authenticatedRead(directory, name: "state", secret: secret)
    let acceptedPhases: Set<String> = [
        "resumed", "consume-prepared", "consume-service-stopped",
        "consume-credentials-deleted", "consume-plist-removed",
        "consume-binaries-removed", "consume-resume-agent-removed",
        "consume-data-verified", "consumed",
    ]
    guard Set(state.keys) == Set(["format", "phase", "request_sha256"]),
          state["format"] as? Int == 1,
          (state["phase"] as? String).map(acceptedPhases.contains) == true,
          state["request_sha256"] as? String == requestSHA else {
        throw Failure(code: "evidence-unavailable", detail: operation)
    }
    let evidence = try authenticatedRead(directory, name: "evidence", secret: secret)
    guard Set(evidence.keys) == Set(["boot_session", "format", "gate", "host_id", "operation", "process", "request_sha256", "result"]),
          evidence["format"] as? Int == 1,
          evidence["operation"] as? String == operation,
          evidence["host_id"] as? String == request["host_id"] as? String,
          evidence["request_sha256"] as? String == requestSHA,
          evidence["boot_session"] as? String != request["pre_boot_session"] as? String,
          let result = evidence["result"] as? [String: Any] else {
        throw Failure(code: "evidence-mismatch", detail: operation)
    }
    _ = try arguments(from: request)
    return (layout, directory, item, secret, request, result, requestSHA)
}

private func verifyProduction(_ operation: String) throws -> Data {
    let (_, directory, _, _, request, _, requestSHA) = try verifiedEvidence(operation)
    return try withOperationLock(directory) {
        try canonical([
            "format": 1,
            "gate": request["gate"]!,
            "operation": operation,
            "phase": "verified",
            "request_sha256": requestSHA,
        ])
    }
}

private func consumeProduction(_ operation: String) throws -> Data {
    let (layout, directory, item, secret, request, result, requestSHA) = try verifiedEvidence(operation)
    return try withOperationLock(directory) {
        var phase = try authenticatedRead(directory, name: "state", secret: secret)["phase"] as? String ?? ""
        if phase == "consumed" {
            let consumed = try authenticatedRead(directory, name: "consumed-evidence", secret: secret)
            guard let final = consumed["result"] as? [String: Any] else {
                throw Failure(code: "operation-corrupt", detail: "consumed evidence")
            }
            return try canonical(final)
        }
        let endpoint = KeychainItem(service: endpointService, account: endpointAccount, accessGroup: "\(BuildIdentity.teamID).com.tekes.shared.endpoint")
        let provider = KeychainItem(service: providerService, account: providerAccount, accessGroup: "\(BuildIdentity.teamID).com.tekes.kernel.provider-secrets")
        guard let identitySHA = request["install_identity_sha256"] as? String else {
            throw Failure(code: "operation-corrupt", detail: "install identity")
        }
        let uninstallRequestSHA = sha256(Data("uat-uninstall\u{0}\(identitySHA)".utf8))
        if phase == "resumed" {
            try writeInstallerOperation(layout, type: "uninstall", phase: "prepared", identitySHA: identitySHA, requestSHA: uninstallRequestSHA)
            try operationState(directory, phase: "consume-prepared", requestSHA: requestSHA, secret: secret)
            phase = "consume-prepared"
        }
        if phase == "consume-prepared" {
            let target = "gui/\(getuid())/\(serviceLabel)"
            let current = try run("/bin/launchctl", ["print", target])
            if current.status == 0 {
                let stopped = try run("/bin/launchctl", ["bootout", target])
                guard stopped.status == 0 else { throw Failure(code: "launchctl", detail: "bootout") }
            }
            guard (try run("/bin/launchctl", ["print", target])).status != 0 else {
                throw Failure(code: "launchctl", detail: "service remains loaded")
            }
            try writeInstallerOperation(layout, type: "uninstall", phase: "service-stopped", identitySHA: identitySHA, requestSHA: uninstallRequestSHA)
            try operationState(directory, phase: "consume-service-stopped", requestSHA: requestSHA, secret: secret)
            phase = "consume-service-stopped"
        }
        if phase == "consume-service-stopped" {
            if result["gate"] as? Int == 76 {
                let revokedRecord = try canonical(["format": 1, "generation": 3, "state": "revoked"], lf: false)
                try verifyProviderGenerationAuthority(layout, generation: 3, record: revokedRecord)
            }
            try endpoint.delete()
            try provider.delete()
            try KeychainItem(
                service: legacyEndpointService,
                account: endpointAccount,
                accessGroup: "\(BuildIdentity.teamID).com.tekes.shared.endpoint"
            ).delete()
            try KeychainItem(
                service: legacyProviderService,
                account: providerAccount,
                accessGroup: "\(BuildIdentity.teamID).com.tekes.kernel.provider-secrets"
            ).delete()
            try writeInstallerOperation(layout, type: "uninstall", phase: "credential-deleted", identitySHA: identitySHA, requestSHA: uninstallRequestSHA)
            try operationState(directory, phase: "consume-credentials-deleted", requestSHA: requestSHA, secret: secret)
            phase = "consume-credentials-deleted"
        }
        if phase == "consume-credentials-deleted" {
            try durableRemove(layout.plist)
            try writeInstallerOperation(layout, type: "uninstall", phase: "plist-removed", identitySHA: identitySHA, requestSHA: uninstallRequestSHA)
            try operationState(directory, phase: "consume-plist-removed", requestSHA: requestSHA, secret: secret)
            phase = "consume-plist-removed"
        }
        if phase == "consume-plist-removed" {
            if FileManager.default.fileExists(atPath: layout.kernel) {
                try FileManager.default.removeItem(atPath: layout.kernel)
                try syncDirectory(URL(fileURLWithPath: layout.kernel).deletingLastPathComponent().path)
            }
            try writeInstallerOperation(layout, type: "uninstall", phase: "binaries-removed", identitySHA: identitySHA, requestSHA: uninstallRequestSHA)
            try operationState(directory, phase: "consume-binaries-removed", requestSHA: requestSHA, secret: secret)
            phase = "consume-binaries-removed"
        }
        if phase == "consume-binaries-removed", let label = request["resume_label"] as? String {
            let target = "gui/\(getuid())/\(label)"
            let stopped = try run("/bin/launchctl", ["bootout", target])
            if stopped.status != 0,
               (try run("/bin/launchctl", ["print", target])).status == 0 {
                throw Failure(code: "cleanup", detail: "resume launch agent")
            }
            if let plist = request["resume_plist"] as? String { try durableRemove(plist) }
            try operationState(directory, phase: "consume-resume-agent-removed", requestSHA: requestSHA, secret: secret)
            phase = "consume-resume-agent-removed"
        }
        if phase == "consume-resume-agent-removed" {
            if result["gate"] as? Int == 76 {
                let revokedRecord = try canonical(["format": 1, "generation": 3, "state": "revoked"], lf: false)
                try verifyProviderGenerationAuthority(layout, generation: 3, record: revokedRecord)
                let retained = try authenticatedRead(directory, name: "retained-data", secret: secret)
                guard let sessionID = retained["session_id"] as? String else {
                    throw Failure(code: "operation-corrupt", detail: "retained-data session")
                }
                let current = try retainedDataProof(layout, session: sessionID)
                guard try canonical(retained) == canonical(current) else {
                    throw Failure(code: "cleanup", detail: "authoritative session bytes changed during uninstall")
                }
            }
            try operationState(directory, phase: "consume-data-verified", requestSHA: requestSHA, secret: secret)
            phase = "consume-data-verified"
        }
        guard phase == "consume-data-verified" else {
            throw Failure(code: "operation-corrupt", detail: "consume phase \(phase)")
        }
        var finalResult = result
        if finalResult["gate"] as? Int == 76 { finalResult["uninstall"] = true }
        try authenticatedWrite(
            directory,
            name: "consumed-evidence",
            value: ["format": 1, "operation": operation, "request_sha256": requestSHA, "result": finalResult],
            secret: secret
        )
        try operationState(directory, phase: "consumed", requestSHA: requestSHA, secret: secret)
        _ = item
        return try canonical(finalResult)
    }
}

private func acknowledgeProduction(_ operation: String) throws -> Data {
    let layout = try Layout.currentUser()
    let directory = operationDirectory(layout, operation)
    let item = KeychainItem(
        service: operationService,
        account: operation,
        accessGroup: "\(BuildIdentity.teamID).com.tekes.kernel.production-uat"
    )
    let tombstone = directory + "/acknowledgement.canonical.json"
    if try item.read() == nil {
        guard let data = try? Data(contentsOf: URL(fileURLWithPath: tombstone)),
              let value = try? decodeCanonical(data) as? [String: Any],
              value["operation"] as? String == operation,
              value["phase"] as? String == "acknowledged" else {
            throw Failure(code: "operation-missing", detail: operation)
        }
        return data
    }
    let (_, _, _, secret, _, _, _) = try verifiedEvidence(operation)
    return try withOperationLock(directory) {
        let state = try authenticatedRead(directory, name: "state", secret: secret)
        guard state["phase"] as? String == "consumed" else {
            throw Failure(code: "evidence-unavailable", detail: "consume is not complete")
        }
        let consumed = try authenticatedRead(directory, name: "consumed-evidence", secret: secret)
        let receiptSHA = sha256(try canonical(consumed))
        let receipt = try canonical([
            "format": 1,
            "operation": operation,
            "phase": "acknowledged",
            "receipt_sha256": receiptSHA,
        ])
        try durableWrite(tombstone, data: receipt, mode: 0o600)
        try item.delete()
        return receipt
    }
}

do {
    switch try Invocation.parse(Array(CommandLine.arguments.dropFirst())) {
    case .describe:
        FileHandle.standardOutput.write(try contractEvidence())
    case let .prepare(arguments):
        FileHandle.standardOutput.write(try prepareProduction(arguments))
        exit(75)
    case let .resume(operation):
        try resumeProduction(operation)
    case let .verify(operation):
        FileHandle.standardOutput.write(try verifyProduction(operation))
    case let .consume(operation):
        FileHandle.standardOutput.write(try consumeProduction(operation))
    case let .acknowledge(operation):
        FileHandle.standardOutput.write(try acknowledgeProduction(operation))
    }
} catch let failure as Failure {
    let error = try canonical(["error": ["code": failure.code, "message": failure.detail]])
    FileHandle.standardError.write(error)
    exit(failure.code == "usage" ? 64 : 1)
} catch {
    let failure = try canonical(["error": ["code": "internal", "message": String(describing: error)]])
    FileHandle.standardError.write(failure)
    exit(1)
}
