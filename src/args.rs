use std::path::PathBuf;

use structopt::StructOpt;
use url::Url;

///CLI arguments
#[derive(Debug, StructOpt)]
pub struct Args {
    ///Entry point of scraping
    #[structopt(name = "url", parse(try_from_str), help = "Entry point of the scraping")]
    pub origin: Url,

    ///Output directory
    #[structopt(short, long, parse(from_os_str), help = "Output directory")]
    pub output: Option<PathBuf>,

    ///Number of threads/workers
    #[structopt(short, long, default_value = "1", help = "Maximum number of threads to use concurrently")]
    pub jobs: usize,

    ///Max depth of scraping recursion
    #[structopt(short, long, default_value = "5", help = "Maximum recursion depth to reach when visiting")]
    pub depth: usize,

    ///Number of retries when downloading a page fails
    #[structopt(short, long, default_value = "20", help = "Maximum amount of retries on download failure")]
    pub tries: usize,

    ///Show all logs
    #[structopt(short, long, help = "Enable more information regarding the scraping process")]
    pub verbose: bool,
}

impl Args {
    ///Collect args
    pub fn collect() -> Args {
        Args::from_args()
    }
}
