---
partial_kind: storage-partial
storage: database-firebase
---

# Database Firebase

Use `database-firebase` when an adapter persists data in Firebase rather than local files.

## Default Fit

- adapters that call Firebase as an external database service
- web or networked adapters
- projects that explicitly name Firebase in the contract

## Default Compatibility Rule

Prefer this slot over `local-files-*` when the adapter depends on Firebase-hosted persistence.
