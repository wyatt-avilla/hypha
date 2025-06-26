use clap::Parser;

/// Simple systemd service monitor
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Space delimited service names to monitor (e.g syncthing.service)
    #[arg(short, long, required = true, value_parser, num_args = 1.., value_delimiter = ' ')]
    pub services: Vec<String>,

    /// Port to run the server on
    #[arg(short, long, default_value_t = 8910)]
    port: u16,
}

fn main() {
    println!("Hello, world!");
}
