<h1 align="center">ccl</h1>

<p align="center">A minimalistic terminal calculator</p>

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

```bash
cargo install --path .
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
