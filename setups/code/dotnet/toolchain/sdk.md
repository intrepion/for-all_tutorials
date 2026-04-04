<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../README.md) / [Setups](../../../README.md) / [Code](../../README.md) / [dotnet](../README.md) / [Toolchain](README.md) / sdk.md
<!-- breadcrumbs:end -->

# .NET SDK

Install and verify the .NET SDK before starting a .NET tutorial run.

Use the SDK as the baseline for restoring templates, building projects, and running tests.

## Install The SDK

If `dotnet --version` fails, install the SDK with the official Microsoft instructions for your operating system:

### macOS

1. Open [Install .NET on macOS](https://learn.microsoft.com/en-us/dotnet/core/install/macos).
2. Open the [Download .NET](https://dotnet.microsoft.com/download/dotnet) page.
3. Choose the latest .NET version and open the SDK download table.
4. Under the macOS installers, choose `Arm64` for Apple Silicon or `x64` for Intel.
5. Open the downloaded installer package and follow the prompts.

### Windows

1. Open [Install .NET on Windows](https://learn.microsoft.com/en-us/dotnet/core/install/windows).
2. Open the [Download .NET](https://dotnet.microsoft.com/download/dotnet) page.
3. Choose the latest .NET version and open the SDK download table.
4. Under the Windows installers, choose your CPU architecture. If you are unsure, choose `x64`.
5. Open the downloaded installer and follow the prompts.

### Linux

For the most common Linux families, start with Microsoft's distro-specific install page:

#### [Arch Linux](https://archlinux.org/)

1. Open [Install .NET on Linux distributions](https://learn.microsoft.com/en-us/dotnet/core/install/linux).
2. Choose `Arch Linux`.
3. Follow the package-manager steps for `dotnet-sdk`.

#### [Debian](https://www.debian.org/)

1. Open [Install .NET on Linux distributions](https://learn.microsoft.com/en-us/dotnet/core/install/linux).
2. Choose `Debian`.
3. Follow the package-manager steps for `dotnet-sdk`.

#### [Fedora](https://fedoraproject.org/)

1. Open [Install .NET on Linux distributions](https://learn.microsoft.com/en-us/dotnet/core/install/linux).
2. Choose `Fedora`.
3. Follow the package-manager steps for `dotnet-sdk`.

#### [openSUSE](https://www.opensuse.org/)

1. Open [Install .NET on Linux distributions](https://learn.microsoft.com/en-us/dotnet/core/install/linux).
2. Choose `openSUSE`.
3. Follow the package-manager steps for `dotnet-sdk`.

#### [NixOS](https://nixos.org/)

1. Open [Install .NET on Linux distributions](https://learn.microsoft.com/en-us/dotnet/core/install/linux).
2. Check whether the current Microsoft page links to a maintained `NixOS` path.
3. If not, use the `nixpkgs` package instructions from the NixOS project site.

#### [Alpine Linux](https://alpinelinux.org/)

1. Open [Install .NET on Linux distributions](https://learn.microsoft.com/en-us/dotnet/core/install/linux).
2. Choose `Alpine`.
3. Follow the package-manager steps for `dotnet-sdk`.

For the other Linux distributions and platform projects you mentioned, start with the project site, then follow its package or container guidance for `.NET`:

- [AerynOS](https://aerynos.com/)
- [Puppy Linux](https://puppylinux-woof-ce.github.io/)
- [Void Linux](https://voidlinux.org/)
- [PCLinuxOS](https://www.pclinuxos.com/)
- [Solus](https://getsol.us/)
- [ZimaOS](https://www.zimaos.com/)
- [KDE Linux](https://kde.org/linux-ready/)
- [Gentoo Linux](https://www.gentoo.org/)
- [Mageia](https://www.mageia.org/)
- [EasyOS](https://easyos.org/)
- [GNOME OS](https://os.gnome.org/)
- [MocaccinoOS](https://www.mocaccino.org/)
- [Slackware Linux](https://www.slackware.com/)
- [Tiny Core Linux](https://www.tinycorelinux.net/)
- [MagOS Linux](http://magos-linux.ru/)
- [KaOS](https://kaosx.us/)
- [Talos Linux](https://www.talos.dev/)
- [ALT Linux](https://www.altlinux.org/)
- [OpenMandriva Lx](https://www.openmandriva.org/)

### BSD

These are not mainstream first-class `.NET` workstation paths, so use the project site and current community guidance:

- [FreeBSD](https://www.freebsd.org/)
- [OpenBSD](https://www.openbsd.org/)

### Android

Use the platform site as your starting point:

- [Android](https://www.android.com/)

### iOS

Use the platform site as your starting point:

- [iOS](https://www.apple.com/os/ios/)

### ChromeOS

Use the platform site as your starting point:

- [ChromeOS](https://www.google.com/chromebook/chrome-os/)

### ReactOS

Use the project site as your starting point:

- [ReactOS](https://reactos.org/)

## Verify The SDK

After installation, open a fresh shell and run:

```bash
dotnet --version
```
