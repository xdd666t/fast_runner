use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    author = "xdd666",
    version = "1.0",
    about = "A simple command-line application",
    long_about = "This application is a simple example of how to use clap to parse command line arguments."
)]
pub struct RunnerArgs {
    #[arg(short, long, default_value = "rust")]
    pub name: String,

    #[arg(short, long, default_value_t = 1)]
    pub count: u8,
}