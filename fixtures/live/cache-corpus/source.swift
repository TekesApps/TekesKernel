private func cacheCorpusPrompts(scenario: String) throws -> [String] {
  switch scenario {
  case "hello":
    return ["hello"]
  case "deepseek-responses-smoke":
    let mainlineConstraint = """
      Smoke workflow constraint: Do not call task or subagent; perform the work in the worker \
      handling this input.
      """
    return [
      """
      \(mainlineConstraint)

      Create deepseek-responses-smoke.txt containing exactly DEEPSEEK_RESPONSES_SMOKE_OK followed by \
      one newline. Verify the artifact before completing. Keep the final answer concise.
      """,
      """
      \(mainlineConstraint)

      Read deepseek-responses-smoke.txt, report its exact nonblank content, and state whether the \
      prior artifact verification passed. Do not modify the file.
      """,
    ]
  case "weighted-ttl":
    return [
      """
      Implement a production-quality weighted TTL + LRU cache targeting Python 3.12+ with no third-party \
      dependencies. Create weighted_ttl_cache.py and test_weighted_ttl_cache.py. The cache must \
      accept max_total_weight and an injected clock callable; provide get(key, default=None), \
      put(key, value, *, weight, ttl), delete(key), __len__(), total_weight, and snapshot(). A hit \
      moves the key to MRU. Expiration occurs when clock() >= expires_at and expired entries behave \
      as missing and are removed lazily. put must reject non-positive weight or ttl and entries \
      heavier than capacity; replacing a key must update value, weight, expiry, and recency \
      atomically. Before LRU eviction, remove all currently expired entries; then evict LRU entries \
      until the new item fits. snapshot must return live entries from MRU to LRU without exposing \
      mutable internal nodes. Use unittest and a deterministic fake clock. Cover normal hits, LRU \
      eviction, weighted eviction, lazy expiry, delete, validation, replacement, length, weight, \
      and snapshot ordering. This host's usable interpreter is named python3.14; run \
      python3.14 -m unittest -v and repair failures. Do the work in files, not just an explanation.
      """,
      """
      Audit the implementation against every original requirement. Run the tests and fix any \
      defect you find. Add explicit deterministic tests for replacement changing total weight, \
      expiry exactly at the deadline, and a get changing the subsequent eviction victim.
      """,
      """
      Add get_or_compute(key, factory, *, ttl, weight). It must return a cached live value without \
      calling factory; on a miss it calls factory exactly once, stores the returned value with the \
      requested ttl and weight, and returns it. If factory raises, propagate the exception without \
      caching anything or disturbing unrelated entries. Add deterministic tests for hit, miss, \
      expired-key recomputation, and exception behavior, then run the full suite and repair failures.
      """,
      """
      Run the complete unittest suite one final time and repair anything failing. Then give a concise \
      final summary containing the files changed, number of tests passing, the average-time \
      complexity of get/put/get_or_compute, and any real remaining limitations. Do not merely claim \
      success: base the summary on the final test output.
      """,
    ]
  case "quota-ledger":
    let mainlineConstraint = """
      Cache-corpus workflow constraint: Do not call task or subagent; perform the work in the first \
      session handling this input.
      """
    return [
      """
      Implement a production-quality, thread-safe sliding-window quota ledger for Python 3.12+ with \
      no third-party dependencies. Create sliding_window_quota.py and test_sliding_window_quota.py. \
      Export SlidingWindowQuotaLedger (SlidingWindowQuota is an acceptable alias), \
      IdempotencyConflictError, and optionally a ConsumeRequest value type. The constructor accepts \
      per-resource positive integer limits, a positive window duration, and an injected clock. \
      consume(subject, amounts, *, idempotency_key) must atomically allow or deny a multi-resource \
      request; remaining(subject) returns every resource's live remaining quota. Expire entries on \
      the exact window boundary. Idempotency keys are global: canonicalize resource mappings so an \
      exact replay is independent of mapping insertion order and returns the original decision \
      without charging again; reuse for another subject or different resource/amount payload raises \
      IdempotencyConflictError. Denied decisions are sticky under replay even after time advances. \
      Defensive-copy caller mappings. Reject invalid subjects, keys, resources, and non-positive or \
      non-integer amounts without mutating state. Use a lock around every compound read/write. Add \
      deterministic unittest coverage with a fake clock, including a replay with reversed mapping \
      insertion order, and run python3.14 -m unittest -v exactly once after implementing. Do not wrap \
      the suite in a repetition or stress loop. Do the work in files, not just an explanation.
      """,
      """
      Audit the ledger against every original requirement and repair any defect. Add consume_many \
      for an ordered batch of ConsumeRequest values or equivalent mappings. The whole batch is one \
      transaction: exact duplicate idempotency entries reuse their decision, but if any new request \
      cannot be allowed, no new request in that batch is charged and every newly evaluated result is \
      denied. Conflicting reuse must raise and leave the ledger unchanged. Add deterministic tests \
      for successful batches with a duplicate, rollback on a later resource failure, and conflict \
      rollback. Run the complete suite exactly once after making the changes; do not repeat it in a \
      loop.
      """,
      """
      Add snapshot() as an immutable, detached view of live usage, remaining quota, and idempotency \
      decisions. No mapping or nested value reachable from the snapshot may mutate ledger state. \
      Audit exact-boundary expiry and sticky denied replays again. Add a 32-thread contention test \
      where 100 one-unit requests race for a quota of 10 and exactly 10 succeed. Run the full suite \
      exactly once after making the changes and repair any failure without weakening assertions. \
      The bounded 32-thread test itself is the required race probe; it must terminate with the final \
      partial worker wave. Do not use a reusable Barrier(32) across all 100 tasks because 100 is not \
      divisible by 32. Do not wrap unittest in any repetition or stress loop.
      """,
      """
      Run python3.14 -m unittest -v exactly once for the final verification and repair anything \
      failing. Do not run the suite in a repetition or stress loop. Then give a concise final \
      summary grounded in the final output: files changed, tests passing, locking and complexity \
      characteristics, and any real remaining limitations. Do not merely claim success.
      """,
    ].map { "\(mainlineConstraint)\n\n\($0)" }
  default:
    throw WorkflowTestError(
      "Unknown TEKES_CACHE_CORPUS_SCENARIO '\(scenario)'; expected hello, deepseek-responses-smoke, weighted-ttl, or quota-ledger"
    )
  }
}

