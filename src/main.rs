use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(version, about)] 
struct Cli {
    a: f64,
    #[arg(value_enum)]
    op: Op,
    b: f64,
}

#[derive(Copy, Clone, ValueEnum)]
enum Op {
    #[clap(alias = "+")]
    Add,
    #[clap(alias = "-")]
    Sub,
    #[clap(alias = "/")]
    Div,
    #[clap(alias = "x", alias = "*")]
    Mul,
    #[clap(alias = "%")]
    Rem,
}

fn main() {
    let cli = Cli::parse();
    let result = match cli.op {
        Op::Add => cli.a + cli.b,
        Op::Sub => cli.a - cli.b,
        Op::Mul => cli.a * cli.b,
        Op::Div => {
            if cli.b == 0.0 {
                eprintln!("Forbidden: division by zero");
                std::process::exit(1);
            }
            cli.a / cli.b
        }
        Op::Rem => {
            if cli.b == 0.0 {
                eprintln!("Forbidden: division by zero");
                std::process::exit(1);
            }
            cli.a % cli.b
        }
    };
    println!("{}", result);
}
