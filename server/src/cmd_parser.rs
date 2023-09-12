use clap::Parser;

#[derive(Parser, Debug)]
pub struct Arguments {
    /// The pattern to look for
    pub port_flag: String,
    /// The path to the file to read
    pub port: i32,
}

pub fn parse_commands() -> Arguments {
    return Arguments::parse();
}
