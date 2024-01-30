use clap::Parser;

/// A static bar builder
#[derive(Parser, Debug)]
#[command(author = "klliio", version, about, long_about = None, arg_required_else_help = true)]
struct Args {
    // Current fill progress
    #[arg(short, long)]
    progress: u16,

    // Maximum value
    #[arg(short, long)]
    max: u16,

    // Length of the bar
    #[arg(short, long)]
    length: u16,

    // Character to show filled progress
    #[arg(long, default_value_t = '█')]
    fill_char: char,

    // Character to show empty bar
    #[arg(long, default_value_t = '▓')]
    empty_char: char,
}

fn main() {
    let args = Args::parse();
    let value_percent: u16;

    // using 'as' as it allows for conversion to u16 and cannot panic here
    value_percent = ((100.0 * f32::from(args.progress)) / f32::from(args.max)) as u16;
    let fill_amount: u16 = ((f32::from(args.length) / 100.0) * f32::from(value_percent)) as u16;

    let mut i: u16 = 0;
    let mut bar: String = String::new();
    while i < fill_amount {
        bar.push(args.fill_char);
        i += 1;
    }
    while i < args.length {
        bar.push(args.empty_char);
        i += 1;
    }

    println!("{}", bar);
}
