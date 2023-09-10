use structopt::StructOpt;
use strum::VariantNames;
use strum_macros:: { EnumString, EnumVariantNames };

#[derive(Debug, EnumString, EnumVariantNames)]
#[strum(serialize_all = "kebab-case")]
enum Categories {
    Science = 0,
    People = 1,
    Comedy = 2,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "rust argparse demo", about = "usage demo.")]
struct Options {
    /// filename of the video to upload
    #[structopt(short, long)]
    file: std::path::PathBuf,
    /// title of the video
    #[structopt(short, long)]
    title: Option<String>,
    /// category of the video
    #[structopt(short, long, default_value="science", possible_values=&Categories::VARIANTS)]
    category: Categories,
    /// verbose log mode
    #[structopt(short, long)]
    verbose: bool,
    /// names to greet
    #[structopt(short = "n", long = "name")]
    names: Vec<String>,
}

fn main() {
    let opts = Options::from_args();
    println!("{:?}", opts);
}
