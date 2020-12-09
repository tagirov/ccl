fn main() {
    let mut args = std::env::args().skip(1);

    let print_ops = "Expected operator: [ + ][ - ][ / ][ x ][ % ]";
    
    let left = args.next().expect("Not enough arguments")
        .parse::<f64>().expect("The first argument is not a number");

    let op = args.next().expect(&print_ops);

    let right = args.next().expect("Not enough arguments")
        .parse::<f64>().expect("The third argument is not a number");

    let result = match op.as_str() {
        "+" => left + right,
        "-" => left - right,
        "/" => left / right,
        "x" => left * right,
        "%" => left % right,
         _  => panic!(print_ops)
    };
    println!("{}", result);
}
