use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "maint", version = "1.0", about = "Maintainer utilities for 2")]
pub struct Args {
    #[arg(short = 'g', long, value_name = "REPO/NAME", value_delimiter = ' ', num_args = 1..)]
    pub generate: Vec<String>,

    #[arg(short = 'a', long, value_name = "REPO/NAME=VERS", value_delimiter = ' ', num_args = 1..)]
    pub add: Vec<String>,

    #[arg(short = 'r', long, value_name = "REPO/NAME", value_delimiter = ' ', num_args = 1..)]
    pub revise: Vec<String>,

    #[arg(short = 'u', long, value_name = "REPO/NAME=VERS", value_delimiter = ' ', num_args = 1..)]
    pub update: Vec<String>,

    #[arg(short = 'R', long, value_name = "REPO/NAME", value_delimiter = ' ', num_args = 1..)]
    pub remove: Vec<String>,

    #[arg(short = 'm', long, value_name = "REPO/NAME", value_delimiter = ' ', num_args = 2)]
    pub r#move: Vec<String>,
}

impl Args {
    pub fn init() -> Self {
        Self::parse()
    }
}
