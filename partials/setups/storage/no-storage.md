---
partial_kind: storage-partial
storage: no-storage
---

# No Storage

Use `no-storage` when an adapter does not persist application data and does not call a storage service.

## Default Fit

- one-shot command-line adapters
- read-only API clients
- deterministic file-transform tools that do not maintain app state

## Avoid Adding Unneeded Storage

Choose this slot when storage would be accidental complexity rather than part of the project contract.
