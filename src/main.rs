extern crate liquery;
extern crate clap;
extern crate walkdir;

use std::env;
use walkdir::WalkDir;
use clap::{App, Arg};
use liquery::Query;

fn main() {
    let app = App::new("lirename")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Renames based on liquery")
        .arg(Arg::with_name("path")
            .help("Path to be iterated over")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("query")
            .help("Query to run")
            .required(true)
            .takes_value(true))
        .get_matches();

    let path = app.value_of("path").unwrap();
    let query = app.value_of("query").unwrap();

    println!("path:{}, query:{}", path, query);

    let query = match Query::parse(query.to_owned()) {
        Ok(q) => q,
        Err(err) => {
            println!("Error: query failed to parse due to {:?}", err);
            return;
        },
    };

    println!("{:?}", query);

    for entry in WalkDir::new(path) {
        if let Ok(e) = entry {
            if let Ok(metadata) = e.metadata() {
                if metadata.is_file() {
                    println!("{:?}", e.path())
                }
            }
        }
    }
}
