extern crate liquery;
extern crate liquery_file;
extern crate clap;
extern crate walkdir;

mod cli;

use std::collections::HashMap;
use walkdir::WalkDir;
use liquery::{Query, EvalFunc};
use liquery_file::QueryFile;

fn main() {

    let app = cli::build_cli().get_matches();

    let path = app.value_of("path").unwrap();
    let query = app.value_of("query").unwrap();

    let query = match Query::parse(query.to_owned()) {
        Ok(q) => q,
        Err(err) => {
            println!("Error: query failed to parse due to {:?}", err);
            return;
        }
    };

    // TODO: Include some functions
    let func = HashMap::<String, Box<EvalFunc>>::new();

    for entry in WalkDir::new(path) {
        if let Ok(e) = entry {
            if let Ok(metadata) = e.metadata() {
                if metadata.is_file() {
                    let queryable = QueryFile::new(e.path()).unwrap();

                    println!("{} => {}",
                             e.path().to_str().unwrap(),
                             query.eval(&queryable, &func).unwrap());

                }
            }
        }
    }
}
