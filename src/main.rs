use std::process::exit;
use std::time::Duration;

use clap::{Parser, ValueEnum};
use http::uri::{self, Uri};
use serde::Deserialize;
use serde_json;
use ureq::Agent;

/// Simple Nagios Plugin of remote HTTP Executor
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// use HTTPS (https://)
    #[clap(short, long, default_value_t = false, value_parser)]
    ssl: bool,

    /// HTTP hostname
    #[clap(short = 'H', long, default_value_t = String::from("localhost"), value_parser)]
    hostname: String,

    /// HTTP port [default: 80 on http, 443 on https]
    #[clap(short, long, value_parser)]
    port: Option<u16>,

    /// HTTP uri
    #[clap(short, long, default_value_t = String::from("/"), value_parser)]
    uri: String,

    /// Queries (multiple times)
    #[clap(short, long, value_parser)]
    query: Vec<String>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Scheme {
    Http,
    Https,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct CheckResult {
    code: u8,
    description: String,
}

const UNKNOWN: i32 = 3;

fn main() -> ! {
    let args: Args = Args::parse();

    let agent: Agent = ureq::AgentBuilder::new()
        .timeout_read(Duration::from_secs(15))
        .timeout_write(Duration::from_secs(15))
        .build();

    let uri: Uri = build_uri(&args).unwrap();

    match agent.get(&uri.to_string()).call() {
        Ok(res) => match res.into_string() {
            Ok(body) => match serde_json::from_str::<CheckResult>(&body) {
                Ok(result) => {
                    println!("{}", result.description);

                    exit(result.code as i32);
                }
                Err(e) => exit_with_internal_error(e.to_string()),
            },
            Err(e) => exit_with_internal_error(e.to_string()),
        },
        Err(e) => exit_with_internal_error(e.to_string()),
    }
}

fn build_uri(args: &Args) -> Result<Uri, http::Error> {
    let scheme = if args.ssl { "https" } else { "http" };

    let port: u16 = args.port.unwrap_or_else(|| if args.ssl { 443 } else { 80 });

    let query = if args.query.is_empty() {
        String::from("")
    } else {
        format!("?{}", args.query.join("&"))
    };

    uri::Builder::new()
        .scheme(scheme)
        .authority(format!("{}:{}", args.hostname, port))
        .path_and_query(format!("{}{}", args.uri, query))
        .build()
}

fn exit_with_internal_error(msg: String) -> ! {
    println!("failed to get result from the server: {}", msg);

    exit(UNKNOWN)
}
