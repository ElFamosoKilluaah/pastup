extern crate structopt;
extern crate pastebin_rs;

use std::path::PathBuf;
use std::fs::File;

use structopt::StructOpt;
use pastebin_rs::PastebinBuilder;

#[derive(StructOpt, Debug)]
#[structopt(name = "pastup")]
struct PastupCli {

    /// Path to the file that contains the content to paste
    #[structopt(short = "p", long = "path")]
    path: PathBuf,

    /// Sets format
    #[structopt(short = "f", long = "format")]
    format: Option<String>,

    /// Sets the lifetime of the paste
    #[structopt(short = "l", long = "lifetime")]
    lifetime: Option<String>,

    /// Sets the paste's title
    #[structopt(short = "t", long = "title")]
    title: Option<String>,
}

fn main() {
    let opt: PastupCli = PastupCli::from_args();
    let api_key = format!(env!("PASTEBIN_API_KEY");
    
    if let Ok(text) = std::fs::read_to_string(&opt.path) {
        
    }
    
    else {
        eprintln!("Error: the file {:?} doesn\'t contain valid UTF-8.", &opt.path);
    }
    
    
    // let builder = PastebinBuilder::new(api_key, , paste_name: Option<String>, visibility: Option<VisibilityLevel>, format: Option<String>, expire_date: Option<ExpirationDate>)


}
