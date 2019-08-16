extern crate getopts;
use getopts::Options;

use std::env;
use std::path::Path;
mod bblmeta;
mod bblrec;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main()  {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("d", "dump", "Dumps headers and exits" );
    opts.optopt("i", "index", "Uses log index", "IDX");
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let dumph = matches.opt_present("d");

    let idx = match matches.opt_str("i") {
        Some(x) => x.parse::<u8>().unwrap(),
        None => 0,
    };

    if matches.free.is_empty() {
        print_usage(&program, opts);
        return;
    } else {
        for source in matches.free.iter() {
            let filename = Path::new(source).file_name().unwrap().to_str().unwrap().to_string();
            let mut b = bblmeta::Bblmeta::new();
            b.getmeta(source).expect("failed to read log");
            let mut n = 1;
            for l in b.l.iter() {
                if idx == 0 || idx == n {
                    println!("Log      : {} / {}", filename, n);
                    println!("Craft    : \"{}\" on {}", l.name, l.ldate);
                    println!("Firmware : {} of {}", l.git, l.gdate);
                    bblrec::log_summary(source, n, dumph, &l.name).unwrap();
                    println!("Disarm   : {}", l.disarm);
                    n += 1;
                    println!();
                }
            }
        }
    }
}