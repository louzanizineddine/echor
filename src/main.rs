use clap::{App , Arg};

fn main() {
    // this program is simulate the echo command in linux base systems
   let mathes = App::new("echor")
        .version("0.1.0")
        .author("zineddine LOUZANI")
        .about("rust echo")
        .arg(
            Arg::with_name("text")
                .value_name("text")
                .help("input text")
                .required(true)
                .min_values(1)

        )
        .arg(
            Arg::with_name("omit_newline")
                .short("n")
                .help("Do not print new line")
                .takes_value(false),
        )
        .get_matches();

    println!("{:#?}", mathes)
}       



