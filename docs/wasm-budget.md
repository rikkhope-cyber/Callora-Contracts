# Contract WASM Size Budget

Callora contract builds are gated by a per-contract WASM budget of **100 KiB
(`102400` bytes)**.

This is the Callora project budget used by the local size-check script and CI.
It is intentionally below the larger network-level contract WASM allowance so
there is room for emergency patches and dependency or feature growth.

The **64 KB claim previously used in the release-profile comment was incorrect
for the contract WASM size limit** and has been removed. The Callora budget
remains 100 KiB unless it is intentionally changed through the project's
budget configuration.

## Local Check

Run the same check used by CI from the repository root:

```bash
./scripts/check-wasm-size.sh
