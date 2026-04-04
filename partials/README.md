---
partial_kind: partials-root
pilot: true
---

# Partials

Source material for future compiled tutorials.

Project partials now cover the full curriculum project set.

Setup partials are still in a pilot phase built around this active `.NET/C# xUnit` family of slices:

- `no-storage/command-line/all/no-framework`
- `no-storage/client/all/no-framework`
- `no-storage/graphical/all/no-framework`
- `no-storage/web-service/all/no-framework`
- `database-firebase/command-line/all/no-framework`
- `database-sqlite/web/full-stack/blazor-server`

Compiled tutorial outputs should be generated from the project manifests in this tree, not authored by hand.

Every project manifest now declares at least one active compiled tutorial output on the current `.NET/C# xUnit` core path.

Compatible projects now also declare active `.NET/C# xUnit` adapter outputs on the currently supported surface, storage, and framework slices.

Projects that still require either new setup partials or additional adapter-activation work include:

- some web full-stack projects
- some local-file persistence projects

## Contents

- [Projects](projects/README.md)
- [Setups](setups/README.md)
