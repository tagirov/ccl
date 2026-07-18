use clap::Parser;

#[derive(Parser)]
#[command(version, about, allow_negative_numbers = true)]
struct Cli {
    /// Expression, e.g.: 2 + 2 x 2 or "2+2*2"
    #[arg(required = true, num_args = 1..)]
    expr: Vec<String>,
}

#[derive(Copy, Clone)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
}

enum Token {
    Num(f64),
    Op(Op),
}

fn read_number(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<f64, String> {
    let mut s = String::new();
    while let Some(&c) = chars.peek() {
        if c.is_ascii_digit() || c == '.' {
            s.push(c);
            chars.next();
        } else {
            break;
        }
    }
    s.parse().map_err(|_| format!("invalid number: {}", s))
}

fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();
    while let Some(&c) = chars.peek() {
        match c {
            ' ' => {
                chars.next();
            }
            '+' | '*' | 'x' | '/' | '%' => {
                chars.next();
                tokens.push(Token::Op(match c {
                    '+' => Op::Add,
                    '*' | 'x' => Op::Mul,
                    '/' => Op::Div,
                    _ => Op::Rem,
                }));
            }
            '-' => {
                chars.next();
                // unary minus at expression start or right after an operator
                if matches!(tokens.last(), None | Some(Token::Op(_))) {
                    tokens.push(Token::Num(-read_number(&mut chars)?));
                } else {
                    tokens.push(Token::Op(Op::Sub));
                }
            }
            c if c.is_ascii_digit() || c == '.' => {
                tokens.push(Token::Num(read_number(&mut chars)?));
            }
            _ => return Err(format!("unexpected character: {}", c)),
        }
    }
    Ok(tokens)
}

fn eval(tokens: Vec<Token>) -> Result<f64, String> {
    let mut nums = Vec::new();
    let mut ops = Vec::new();
    let mut expect_num = true;
    for token in tokens {
        match (token, expect_num) {
            (Token::Num(n), true) => {
                nums.push(n);
                expect_num = false;
            }
            (Token::Op(op), false) => {
                ops.push(op);
                expect_num = true;
            }
            _ => return Err("malformed expression".into()),
        }
    }
    if expect_num {
        return Err("malformed expression".into());
    }

    // first pass: * / % (higher precedence)
    let mut sum_nums = vec![nums[0]];
    let mut sum_ops = Vec::new();
    for (op, n) in ops.into_iter().zip(nums.into_iter().skip(1)) {
        match op {
            Op::Mul | Op::Div | Op::Rem => {
                if matches!(op, Op::Div | Op::Rem) && n == 0.0 {
                    return Err("Forbidden: division by zero".into());
                }
                let a = sum_nums.pop().unwrap();
                sum_nums.push(match op {
                    Op::Mul => a * n,
                    Op::Div => a / n,
                    _ => a % n,
                });
            }
            Op::Add | Op::Sub => {
                sum_ops.push(op);
                sum_nums.push(n);
            }
        }
    }

    // second pass: + -
    let mut result = sum_nums[0];
    for (op, n) in sum_ops.into_iter().zip(sum_nums.into_iter().skip(1)) {
        result = match op {
            Op::Add => result + n,
            _ => result - n,
        };
    }
    Ok(result)
}

fn main() {
    let cli = Cli::parse();
    match tokenize(&cli.expr.join(" ")).and_then(eval) {
        Ok(v) => println!("{}", v),
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calc(input: &str) -> Result<f64, String> {
        tokenize(input).and_then(eval)
    }

    #[test]
    fn basic_operations() {
        assert_eq!(calc("2 + 3"), Ok(5.0));
        assert_eq!(calc("7 - 10"), Ok(-3.0));
        assert_eq!(calc("6 * 7"), Ok(42.0));
        assert_eq!(calc("15 / 4"), Ok(3.75));
        assert_eq!(calc("10 % 3"), Ok(1.0));
    }

    #[test]
    fn operator_precedence() {
        assert_eq!(calc("2 + 2 * 2"), Ok(6.0));
        assert_eq!(calc("10 - 4 / 2"), Ok(8.0));
        assert_eq!(calc("2 * 3 + 4 * 5"), Ok(26.0));
        assert_eq!(calc("1 + 10 % 3 - 2"), Ok(0.0));
    }

    #[test]
    fn left_associativity() {
        assert_eq!(calc("10 - 3 - 2"), Ok(5.0));
        assert_eq!(calc("100 / 10 / 2"), Ok(5.0));
        assert_eq!(calc("100 % 30 % 7"), Ok(3.0));
    }

    #[test]
    fn no_spaces() {
        assert_eq!(calc("2+2*2"), Ok(6.0));
        assert_eq!(calc("1-2/4"), Ok(0.5));
    }

    #[test]
    fn x_operator() {
        assert_eq!(calc("3 x 4"), Ok(12.0));
        assert_eq!(calc("5x5+5"), Ok(30.0));
    }

    #[test]
    fn floats() {
        assert_eq!(calc("1.5 + 2.25"), Ok(3.75));
        assert_eq!(calc(".5 * 4"), Ok(2.0));
        assert_eq!(calc("0.1 * 10"), Ok(0.1f64 * 10.0));
    }

    #[test]
    fn unary_minus() {
        assert_eq!(calc("-5"), Ok(-5.0));
        assert_eq!(calc("-2 + 3"), Ok(1.0));
        assert_eq!(calc("2 * -3"), Ok(-6.0));
        assert_eq!(calc("2--3"), Ok(5.0));
        assert_eq!(calc("-2*-2"), Ok(4.0));
    }

    #[test]
    fn single_number() {
        assert_eq!(calc("42"), Ok(42.0));
        assert_eq!(calc("-0.5"), Ok(-0.5));
    }

    #[test]
    fn division_by_zero() {
        assert_eq!(calc("1 / 0"), Err("Forbidden: division by zero".into()));
        assert_eq!(calc("1 % 0"), Err("Forbidden: division by zero".into()));
        // zero as dividend is fine
        assert_eq!(calc("0 / 5"), Ok(0.0));
    }

    #[test]
    fn malformed_expressions() {
        assert_eq!(calc(""), Err("malformed expression".into()));
        assert_eq!(calc("2 +"), Err("malformed expression".into()));
        assert_eq!(calc("+ 2"), Err("malformed expression".into()));
        assert_eq!(calc("2 3"), Err("malformed expression".into()));
        assert_eq!(calc("2 + * 3"), Err("malformed expression".into()));
    }

    #[test]
    fn invalid_input() {
        assert_eq!(calc("2 & 3"), Err("unexpected character: &".into()));
        assert_eq!(calc("2 foo 3"), Err("unexpected character: f".into()));
        assert_eq!(calc("1.2.3 + 1"), Err("invalid number: 1.2.3".into()));
        assert_eq!(calc("2 + ."), Err("invalid number: .".into()));
    }
}
