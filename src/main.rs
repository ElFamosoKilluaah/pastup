use std::path::PathBuf;

use structopt::StructOpt;
use pastebin_rs::{ PastebinBuilder, VisibilityLevel };

#[derive(StructOpt, Debug)]
#[structopt(name = "pastup")]
struct PastupCli {

    /// Path to the file that contains the content to paste
    #[structopt(short = "p", long = "path")]
    path: PathBuf,

    /// Sets format
    #[structopt(short = "f", long = "format")]
    format: Option<String>,

    /// Sets the visibility of the paste
    #[structopt(short = "v", long = "visibility")]
    visibility: Option<String>,

    /// Sets the lifetime of the paste
    #[structopt(short = "l", long = "lifetime")]
    lifetime: Option<String>,

    /// Sets the paste's name
    #[structopt(short = "t", long = "title")]
    name: Option<String>,
}

fn main() {
    let opt: PastupCli = PastupCli::from_args();
    let api_key = format!(env!("PASTEBIN_API_KEY");
    
    let visibility = match opt.visibility {
        Some(v) => parse_visibility_string(v),
        None => None,
    };


    if let Ok(text) = std::fs::read_to_string(&opt.path) {
        // let builder = PastebinBuilder::new(
        //     api_key,
        //     text,
        //     opt.name,


        // )
    }
    
    else {
        eprintln!("Error: the file {:?} doesn\'t contain valid UTF-8.", &opt.path);
    }
}

fn parse_visibility_string(to_parse: String) -> Option<VisibilityLevel> {
    match to_parse {

    }
}