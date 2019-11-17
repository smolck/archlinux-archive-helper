# NOTE
This does no checking (yet) on the validity of the packages installed using the `.sig` versions
of the packages (e.g. using `pango-1.43.0-1-x86_64.pkg.tar.xz.sig`). Thus, if any installed packages have been
maliciously modified, I am not responsible for any damage to your system. Use this helper at your own risk.

# archlinux-archive-helper
A simple helper for installing packages from the [Arch Linux Archive](https://archive.archlinux.com).
Currently this helper only helps with packages from the `packages` section of the Archive, but that may
change in the future. Contributions are welcome!

# Example
I haven't yet provided a way to install this helper (will hopefully do so soon), so the helper must be
run with `cargo run`. In any case, here's an example session:
```bash
cargo run -- pango # Using `--` to provide the helper with args.

# snip

Fetching packages..... # in this case fetching from https://archive.archlinux.com/packages/p/pango/
0     pango-1.43.0-1-x86_64.pkg.tar.xz
1     pango-1:1.42.4-1-x86_64.pkg.tar.xz
2     pango-1:1.43.0-1-x86_64.pkg.tar.xz
3     pango-1:1.43.0-2-x86_64.pkg.tar.xz
4     pango-1:1.44-1-x86_64.pkg.tar.xz
5     pango-1:1.44.1-1-x86_64.pkg.tar.xz
6     pango-1:1.44.2+1+gb50f0ef8-1-x86_64.pkg.tar.xz
7     pango-1:1.44.3-1-x86_64.pkg.tar.xz
8     pango-1:1.44.4-1-x86_64.pkg.tar.xz
9     pango-1:1.44.4-2-x86_64.pkg.tar.xz
10     pango-1:1.44.5-1-x86_64.pkg.tar.xz
11     pango-1:1.44.6+2-1-x86_64.pkg.tar.xz
12     pango-1:1.44.6-1-x86_64.pkg.tar.xz
13     pango-1:1.44.7-1-x86_64.pkg.tar.xz

Package to install: (e.g. 1)
2
Downloading package.....
[sudo] password for smolck: # sudo password goes here
loading packages...
warning: downgrading package pango (1:1.44.7-1 => 1:1.42.4-1)
resolving dependencies...
looking for conflicting packages...

Packages (1) pango-1:1.42.4-1

Total Installed Size:   3.67 MiB
Net Upgrade Size:      -0.69 MiB

:: Proceed with installation? [Y/n] Y

# installs package and exits
```
