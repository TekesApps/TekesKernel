# Provider runtime fixtures

`cases.canonical.json` is the Slice-7 completion inventory. Every row names its
exact request, response, stream, normalized-result authority, and source; there
is no filename expansion or implied artifact. `negative.canonical.json` is the
closed language-neutral negative-case inventory. The source lock pins the
predecessor revision used for parity; the Kernel implementation may read but
never generate the oracle.

The four predecessor rows and new Google Interactions row name separate pinned
authorities. The raw dialect snapshots are the first Slice-7 fixture increment.
