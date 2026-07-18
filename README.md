<h1 align="center">ccl</h1>

<p align="center">A minimalistic terminal calculator</p>

```bash
$ ccl 2 + 2 x 2
6
```

## Features

- Full expressions with operator precedence: `*` `/` `%` before `+` `-`
- Floats, unary minus, no parentheses needed for quoting: `ccl -2.5 x -2`
- Word aliases for every operator — no shell escaping: `ccl 2 x 2` instead of `ccl "2 * 2"`
- Spaces optional: `ccl "2+2*2"`

## Install

```bash
cargo install --path .
```

## Usage

```bash
ccl 2 + 2           # 4
ccl 2 + 2 x 2       # 6  (precedence)
ccl 10 - 3 - 2      # 5  (left-associative)
ccl 1.5 mul 4       # 6
ccl "2+2*2"         # quoted form works too
```

## Operators

| Symbol | Alias  | Operation |
|:------:|:------:|-----------|
| `+`    | `add`  | addition  |
| `-`    | `sub`  | subtraction |
| `*`    | `x`, `mul` | multiplication |
| `/`    | `div`  | division  |
| `%`    | `rem`  | remainder |

## License

[GPL-3.0](LICENSE)
