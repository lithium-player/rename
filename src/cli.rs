use clap::{App, Arg};

pub fn build_cli() -> App<'static, 'static> {
    App::new("lirename")
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::with_name("path")
            .help("Path to be iterated over")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("query")
            .help("Query to run")
            .required(true)
            .takes_value(true))
}
