use clap::{Arg, App};
use rustkeys::big_random;

fn main() {
    let _matches = App::new("BigRandom")
        .version("21.12.28")
        .author("darryl.west@raincitysoftware.com")
        .about("A big, potentially huge random number generator to create base 36 random keys")
        .arg(Arg::with_name("size")
            .short("s")
            .long("size")
            .help("set the number of random u32 words to generate, defaults to 8.")
            .takes_value(true))
        .get_matches();

    let size= 8;

    println!("{}", big_random(size));
}