---
partial_kind: partials-root
pilot: true
---

# Partials

Source material for future compiled tutorials.

Project partials now cover the full curriculum project set.

Setup partials are still in a pilot phase built around this active stack:

- `.NET`
- `C#`
- `xUnit`
- `no-storage`
- `command-line/all`
- `no-framework`

Compiled tutorial outputs should be generated from the project manifests in this tree, not authored by hand.

Every project manifest now declares at least one active compiled tutorial output on the current `.NET/C# xUnit` core path.

Compatible projects now also declare an active `.NET/C# xUnit/no-storage/command-line/all/no-framework` adapter output.

Projects that still require new setup partials before adapter activation include:

- graphical projects
- web full-stack projects
- service-and-client multi-adapter projects
- projects that require real persistent storage paths

## Contents

- [Projects](projects/README.md)
- [Setups](setups/README.md)
