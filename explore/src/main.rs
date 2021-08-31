/// [httpie](https://github.com/httpie/httpie)
use anyhow::Result;
use clap::{AppSettings, Clap};
use reqwest::Url;

#[derive(Clap, Debug)]
#[clap(version = "1.0", author = "Yu Le <is.yuler@gmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap, Debug)]
enum SubCommand {
    Get(Get),
    Post(Post),
}

#[derive(Clap, Debug)]
struct Get {
    #[clap(parse(try_from_str = parse_url))]
    url: String,
}

#[derive(Clap, Debug)]
struct Post {
    url: String,
    body: Vec<String>,
}

fn main() {
    let opts: Opts = Opts::parse();
    println!("{:?}", opts);
}

fn parse_url(s: &str) -> Result {
    let _url: Url = s.parse()?;
    Ok(s.into())
}
