extern crate clap;
extern crate serde_json;

use std::io;
use std::str::FromStr;

use clap::{Arg, App};

fn main() {
    let matches = App::new("pairmaker")
                      .version(env!("CARGO_PKG_VERSION"))
                      .author("Alexandre Bury <alexandre.bury@gmail.com>")
                      .about("A utility to generate pairs for peer-programming schedules.")
                      .arg(Arg::with_name("NAMES")
                               .help("List of names")
                               .required(true)
                               .multiple(true)
                               .index(1))
                      .arg(Arg::with_name("MIRROR")
                               .help("Add a mirror session after each pair")
                               .short("m")
                               .long("mirror"))
                      .arg(Arg::with_name("N")
                               .help("Number of sessions to print (defaults to number of names \
                                      minus 1)")
                               .short("n")
                               .long("sessions")
                               .takes_value(true))
                      .get_matches();

    let names: Vec<_> = matches.values_of("NAMES").unwrap().collect();

    let n = matches.value_of("N")
                   .map(|s| usize::from_str(s).expect("invalid number"))
                   .unwrap_or(names.len() - 1);

    let mirror = matches.is_present("MIRROR");

    // This iterator will return the pairs of names
    let iter = (0..n).map(|i| {
        let i = i % (names.len() - 1);
        let pairs = make_pairs(names.len(), i + 1);
        let names: Vec<_> = pairs.iter().map(|&(a, b)| (names[a], names[b])).collect();
        names
    });

    // If we want mirrors, duplicate it just before collection
    let schedule: Vec<_> = if mirror {
        iter.flat_map(|pairs| {
                let mirror = pairs.iter().map(|&(a, b)| (b, a)).collect();
                Some(pairs).into_iter().chain(Some(mirror).into_iter())
            })
            .collect()
    } else {
        iter.collect()
    };

    // Prints it to the output in JSON format
    serde_json::to_writer(&mut io::stdout(), &schedule).unwrap();
    // Add a newline at the end
    println!("");
}

/// This makes pairs of elements in [0,n[, rotated by `i`.
///
/// (Each value of `i` will give different pairs.)
fn make_pairs(n: usize, i: usize) -> Vec<(usize, usize)> {
    // First element is always (0,i)
    Some((0, i))
        .into_iter()
        .chain((1..n / 2).map(|k| {
            let a = 1 + (i - 1 + k) % (n - 1);
            let b = 1 + (i - 1 + (n - 1) - k) % (n - 1);
            (a, b)
        }))
        .collect()
}
