extern crate structopt;
use std::{fmt::format, option, path};
use structopt::StructOpt;

extern crate colored;
use colored::*;

extern crate failure;
use failure::ResultExt;
extern crate exitfailure;
use exitfailure::ExitFailure;

#[derive(StructOpt)]
struct Options {
    #[structopt(default_value = "Meow!")]
    /// What doest the cat say?
    message : String,

    #[structopt(short = "d", long = "dead")]
    /// make the cat dead
    dead: bool,

    #[structopt(short = "f", long = "file", parse(from_os_str))]
    /// load the cat picture from a file
    catfile : Option<std::path::PathBuf>,
}

// with "failure" crate 
// fn main() -> Result<(), failure::Error> {
// here we return ExitFailure instead of failure::Error
fn main() -> Result<(), ExitFailure> {
    let options = Options::from_args();
    let message = options.message;
    let eye = if options.dead { "x" } else { "o" };

    if message == "woof" {
        eprintln!("A cat shouldn't bark like a dog");
        ()
    }

    match &options.catfile {
        Some(path) => {
            // let cat_template = std::fs::read_to_string(path)
                // .with_context(|_| format!("Could not read file {:?}", path))?;
            let cat_template = std::fs::read_to_string(path)
                .with_context(|_| format!("Could not read file {:?}", path))?;
            let cat_picture = cat_template.replace("{eye}", eye);
            println!("{}", message.white().on_green());
            println!("{}", &cat_picture);
        },
        None => {
            println!("{}", message.bright_yellow().underline().on_purple());
            println!(" \\");
            println!("  \\");
            println!("     /\\_/\\");
            println!("    ( {eye} {eye} )", eye=eye.red().bold());
            println!("    =( I )=");
        },
    }

    Ok(())
}

// rust error handling
/* fn main() -> Result<(), Box<dyn std::error::Error>> { */
    /* let options = Options::from_args(); */
    /* let message = options.message; */
    /* let eye = if options.dead { "x" } else { "o" }; */
    /*  */
    /* if message.to_lowercase() == "woof" { */
    /*     eprintln!("A cat should not bark as a dog."); */
    /*     () */
    /* } */
    /*  */
    /* // print from a template file */
    /* match &options.catfile { */
    /*     Some (path) => { */
    /*         let cat_template = std::fs::read_to_string(path)?; */
    /*          */
    /*         //    .expect(&format!("Could not read file {:?}", path)); */
    /*         // ? equals to the below snippet */
    /*         // let cat_template = match std::fs::read_to_string(path) { */
    /*         //    Ok(content) => content, */
    /*         //    Err(e) => return e, */
    /*         // }; */
    /*         let cat_picture = cat_template.replace("{eye}", eye); */
    /*         println!("{}", &cat_picture).; */
    /*     }, */
    /*     None => { */
    /*         println!("{}", message.bright_yellow().underline().on_purple()); */
    /*         println!(" \\"); */
    /*         println!("  \\"); */
    /*         println!("     /\\_/\\"); */
    /*         println!("    ( {eye} {eye} )", eye=eye.red().bold()); */
    /*         println!("    =( I )=");            */
    /*     } */
    /* } */
    /* Ok(()) */
/* } */
