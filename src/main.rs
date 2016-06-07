extern crate liquery;
extern crate liquery_file;
extern crate clap;
extern crate walkdir;

use std::collections::HashMap;
use std::env;
use walkdir::WalkDir;
use clap::{App, Arg};
use liquery::{Query, Queryable, EvalFunc};
use liquery_file::QueryFile;

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
        }
    };

    println!("{:?}", query);

    // TODO: Include some functions
    let func = HashMap::<String, Box<EvalFunc>>::new();

    for entry in WalkDir::new(path) {
        if let Ok(e) = entry {
            if let Ok(metadata) = e.metadata() {
                if metadata.is_file() {
                    let path = e.path();
                    let queryable = QueryFile::new(e.path()).unwrap();
                    println!("{} => {}",
                             e.path().to_str().unwrap(),
                             query.eval(&queryable, &func).unwrap());
                }
            }
        }
    }
}
