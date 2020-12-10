use std::process::exit;

fn main() {
    let mut args = std::env::args().skip(1);

    let print_ops = "Expected operator: [ + ][ - ][ / ][ x ][ % ]";

    let a = args.next()
        .unwrap_or_else(|| {
            eprintln!("Not enough arguments");
            exit(1);
        })
        .parse::<f64>()
        .unwrap_or_else(|_| {
            eprintln!("The first argument is not a number");
            exit(1);
        });
    let op = args.next()
        .unwrap_or_else(|| {
            eprintln!("{}", &print_ops);
            exit(1);
        });
    let b = args.next()
        .unwrap_or_else(|| {
            eprintln!("Not enough arguments");
            exit(1);
        })
        .parse::<f64>()
        .unwrap_or_else(|_| {
            eprintln!("The third argument is not a number");
            exit(1);
        });
    if (a == 0.0 || b == 0.0) && (op == "/" || op == "%") {
            eprintln!("Forbidden: division by zero");
            exit(1);
    }
    let result = match op.as_str() {
        "+" => a + b,
        "-" => a - b,
        "/" => a / b,
        "x" => a * b,
        "%" => a % b,
        _ => {
            eprintln!("{}", print_ops);
            exit(1);
        }
    };
    println!("= {}", result);
}
