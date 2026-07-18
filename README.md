<h1 align="center">ccl</h1>

<p align="center">A minimalistic cross-platform terminal calculator</p>

```bash
$ ccl 2 + 2 x 2
6
```

## Features

- Full expressions with operator precedence: `*` `/` `%` before `+` `-`
- Floats, unary minus, no parentheses needed for quoting: `ccl -2.5 x -2`
- `x` as multiplication — no shell escaping: `ccl 2 x 2` instead of `ccl "2 * 2"`
- Spaces optional: `ccl 5x5+5`

## Install

#### Prebuilt binaries

Grab the latest binary for Linux, MacOS (Intel / Apple Silicon) or Windows
from [Releases](https://github.com/tagirov/ccl/releases).

#### Linux/MacOS/Windows

```bash
cargo install --git https://github.com/tagirov/ccl
```

The binary will be installed to:
- Linux/MacOS: `$HOME/.cargo/bin/ccl`
- Windows: `%USERPROFILE%\.cargo\bin\ccl.exe`

Make sure that these paths are added to your $PATH environment variable to use `ccl` command globally.

#### Nix

Install with a single command (any system with Nix, flakes enabled):

```bash
nix profile add github:tagirov/ccl
```

Run without installing:

```bash
nix run github:tagirov/ccl -- 2 + 2 x 2
```

Or add to your NixOS/Home Manager configuration as a flake input:

```nix
{
  inputs.ccl.url = "github:tagirov/ccl";
}
```

```nix
environment.systemPackages = [ inputs.ccl.packages.${pkgs.system}.default ];
```

#### Manually

```bash
git clone https://github.com/tagirov/ccl && cd ccl
```
```bash
cargo build --release
```

Linux/MacOS

```bash
sudo install -m 755 ./target/release/ccl /usr/local/bin
```

Windows

```bash
copy .\target\release\ccl.exe "%USERPROFILE%\AppData\Local\Microsoft\WindowsApps\"
```

## Usage

```bash
ccl 2 + 2           # 4
ccl 2 + 2 x 2       # 6  (precedence)
ccl 10 - 3 - 2      # 5  (left-associative)
ccl 5x5+5           # 30 (spaces optional)
ccl "2+2*2"         # quoted form works too
```

## Operators

| Symbol | Alias | Operation |
|:------:|:-----:|-----------|
| `+`    |       | addition  |
| `-`    |       | subtraction |
| `*`    | `x`   | multiplication |
| `/`    |       | division  |
| `%`    |       | remainder |

## License

[GPL-3.0](LICENSE)
