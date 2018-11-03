extern crate ash_nazg;
extern crate docopt;

#[macro_use] extern crate serde_derive;

use std::io::prelude::*;
use std::fs::File;
use docopt::Docopt;

const USAGE: &'static str = "
FlagConfig Composer.

Usage:
  fcc [options] <file>... [--constraint <constraint>]...
  fcc (-h | --help)

Options:
  -c <constraint> --constraint <constraint>  Constraint for FlagConfig composer.
  -o <output> --output <output>              Output file.
  <file>                                     FlagConfig compose universe.
  -v --verbose                               Make execution verbose.
  -h --help                                  Show this help.
";

#[derive(Debug, Deserialize)]
struct Args {
    arg_file: Vec<String>,
    flag_constraint: Vec<String>,
    flag_output: Option<String>,
    flag_verbose: bool,
}


fn main() -> Result<(), std::io::Error> {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    if args.arg_file.len() > 2 {
        eprintln!("More than two flag generator files not yet supported.");
        eprintln!("Only two first file will be taken into account.");
    } else if args.arg_file.len() < 2 {
        eprintln!("Currently only exactly to files are supprted.");
    } else {
        let fst = File::open(&args.arg_file[0]).unwrap();
        let snd = File::open(&args.arg_file[1]).unwrap();

        let mut constr: Vec<(String, String)> = Vec::new();
        eprintln!("Generating flag config for tags:");
        for sstr in args.flag_constraint.iter() {
            let o: Vec<&str> = sstr.split("=").collect();
            constr.push((o[0].to_string(), o[1].to_string()));
            eprintln!("  >> {}: {}", o[0], o[1]);
        }

        let out = ash_nazg::compose_from_file(&fst, &snd, constr);
        if args.flag_output.is_some() {
            let file_name = args.flag_output.unwrap();
            let mut out_file = File::create(&file_name)?;
            out_file.write(out.as_bytes())?;
            eprintln!("Written to file {}", file_name);
        }

        eprintln!("Printing flag config down to stdout:");
        print!("{}", out);
    };
    Ok(())
}
