use structopt::StructOpt;

mod tweetable_mathematical_art;
use tweetable_mathematical_art::*;

#[derive(Debug, StructOpt)]
struct Args {
    example: String,
}

const DIM: usize = 1024;

fn main() {
    let args = Args::from_args();
    match &args.example[..] {
        "mandelbrot" => {
            Mandelbrot::new(DIM, DIM);
            println!("ok");
        }
        _ => println!("oh no! not support yet!"),
    };
}
