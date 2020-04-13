use serde::Deserialize;
// use serde_json::de::from_reader;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use structopt::StructOpt;

const DUDE_CONF_PATH: &'static str = "/Users/meysamsalehi/Documents/projects/personal/rust-dude/dude/dude.conf.json";

#[derive(StructOpt)]
struct Cli {
    collection: String,
    action: String,
}

#[derive(Deserialize)]
struct Server {
    name: String,
    user: String,
    ip: String,
    port: u32,
}

#[derive(Deserialize)]
struct Config {
    servers: Vec<Server>,
}

fn main() {
    println!("Hey Dude,");
    let args = Cli::from_args();

    let config = read_config_from_file(DUDE_CONF_PATH).unwrap();

    if args.collection == "servers" {
        servers(args.action, config);
    }
}


fn servers(action: String, config: Config) {
    
    if action == "list" {
        servers_list(config);
    }

}

fn servers_list(config: Config) {
    let servers = config.servers;
    for server in &servers {
        println!("{} -> {}@{} -p {}", server.name, server.user, server.ip, server.port );
    }

    println!("This is list", );
}

fn read_config_from_file<P: AsRef<Path>>(path: P) -> Result<Config, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let config = serde_json::from_reader(reader)?;

    // Return the `User`.
    Ok(config)
}
