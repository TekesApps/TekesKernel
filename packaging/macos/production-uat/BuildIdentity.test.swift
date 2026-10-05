// Portable contract tests compile this identity only. The signed production
// build always generates BuildIdentity.swift from the explicit build inputs.
enum BuildIdentity {
    static let teamID = "TEKESAPP01"
    static let clientRequirement = "anchor apple generic and identifier com.tekes.client"
}
