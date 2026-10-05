# AppServer flow test port

Source: TekesAppServer `e4df9dc94e60025d0e8f62bcf98c95241181793a`,
`Tests/Core/AppServerFlowVerificationTests.swift`, FlowScenario.make and its
argument builders. Six cases preserve original task intent, seed bytes and
tool arguments. Tokens use fixed case suffixes because each run has an
isolated workspace. The shell root spelling changes from `.` to Kernel's
empty relative path; the command remains `pwd`.

The public contract maps legacy context records to Kernel session journal
facts. This fixture alone does not prove execution or notification ordering.
Offline and remote-model matrices must retain independent evidence.


`run-public-flow.py --live` uses the configured Cloudflare Responses route.
It restores the original live write path `docs/live-smoke.md` and exact bytes
`LIVE_OK`, and uses the original live token prefix. Other prompts and seed
files are unchanged. Live result-body substring checks follow the original
relaxation, while actual successful tool results and event ordering remain
mandatory. The provider credential is never included in the saved config.
