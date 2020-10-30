fn main() {
    let args = std::env::args;

    let first = args()
        .nth(1)
        .expect("Not enough arguments")
        .parse::<f64>()
        .expect("The first argument must be a number");

    let op = args()
        .nth(2)
        .expect("The second argument must be an operator:  [+] [-] [/] [x] [%]");

    let second = args()
        .nth(3)
        .expect("Not enough arguments")
        .parse::<f64>()
        .expect("The third argument must be a number");

    match op.as_str() {
        "+" => println!("{}", first + second),
        "-" => println!("{}", first - second),
        "/" => println!("{}", first / second),
        "x" => println!("{}", first * second),
        "%" => println!("{}", first % second),
        _ => println!("Wrong operator. Available: [+] [-] [/] [x] [%]"),
    }
}
