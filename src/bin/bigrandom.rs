use clap::{App, Arg};
use rustkeys::big_random;

fn main() {
    let matches = App::new("BigRandom")
        .version("21.12.28")
        .author("darryl.west@raincitysoftware.com")
        .about("A big, potentially huge random number generator to create base 36 random keys")
        .arg(
            Arg::with_name("size")
                .short("s")
                .long("size")
                .help("set the number of random u32 words to generate, defaults to 8.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("base")
                .short("b")
                .long("base")
                .help("set the radix base between 2 and 36, defaults to 36")
                .takes_value(true),
        )
        .get_matches();

    let sz = matches.value_of("size").unwrap_or("8");
    let size = sz.parse::<usize>().unwrap();

    let rx = matches.value_of("base").unwrap_or("36");
    let radix = rx.parse::<u32>().unwrap();

    println!("{}", big_random(size, radix));
}
