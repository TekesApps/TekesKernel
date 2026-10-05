import Foundation
let data = try Data(contentsOf: URL(fileURLWithPath: CommandLine.arguments[1]))
let frames = try JSONSerialization.jsonObject(with: data) as! [[String: Any]]
let decoder = JSONDecoder()
for item in frames {
  let data = try JSONSerialization.data(withJSONObject: item["frame"]!)
  switch item["kind"] as! String {
  case "workspace": try decoder.decode(SessionWorkspaceSyncFrame.self, from: data).validate()
  case "inventory": try decoder.decode(SessionInventorySyncFrame.self, from: data).validate()
  case "control": try decoder.decode(SessionControlSyncFrame.self, from: data).validate()
  case "actionables": try decoder.decode(SessionActionableSyncFrame.self, from: data).validate()
  case "journal": try decoder.decode(SessionJournalFrame.self, from: data).validate()
  default: fatalError("unknown stream")
  }
}
print("CURRENT_CLIENT_DECODE_AND_VALIDATE_OK \(frames.count)")
