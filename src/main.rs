extern crate liquery;
extern crate liquery_file;
extern crate clap;
extern crate walkdir;

mod cli;

use std::collections::HashMap;
use walkdir::WalkDir;
use liquery::{Query, EvalFunc};
use liquery_file::QueryFile;

use std::io::Error as IOError;
use std::path::Path;

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

                    if app.is_present("MOVE") {
                        let _ = move_file(e.path(), query.eval(&queryable, &func).unwrap());
                    } else if app.is_present("COPY") {
                        let _ = copy_file(e.path(), query.eval(&queryable, &func).unwrap());
                    } else {
                        let _ = dry_run_move(e.path(), query.eval(&queryable, &func).unwrap());
                    }
                }
            }
        }
    }
}

fn dry_run_move<F, T>(from: F, to: T) -> Result<(), IOError>
    where F: AsRef<Path>,
          T: AsRef<Path>
{
    println!("{:?} => {:?}", from.as_ref(), to.as_ref());
    Ok(())
}

fn move_file<F, T>(from: F, to: T) -> Result<(), IOError>
    where F: AsRef<Path>,
          T: AsRef<Path>
{
    println!("moving {:?} => {:?}", from.as_ref(), to.as_ref());
    Ok(())
}

fn copy_file<F, T>(from: F, to: T) -> Result<(), IOError>
    where F: AsRef<Path>,
          T: AsRef<Path>
{
    println!("copying {:?} => {:?}", from.as_ref(), to.as_ref());
    Ok(())
}
