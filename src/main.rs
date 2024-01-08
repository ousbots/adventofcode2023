use clap::Parser;

mod day1;
mod day2;
mod day3;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // Problem day to run.
    #[arg(short, long)]
    day: u32,

    // Problem part to run.
    #[arg(short, long)]
    part: u32,

    // Problem input to use.
    #[arg(short, long)]
    input: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let path = format!("data/day{}/{}", args.day, args.input);

    match args.day {
        1 => day1::main(args.part, path)?,
        2 => day2::main(args.part, path)?,
        3 => day3::main(args.part, path)?,
        //4 => day4::main(args.part, &args.input)?,
        _ => panic!("invalid day {}", args.day),
    }

    Ok(())
}
