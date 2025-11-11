use saphyr::{LoadableYamlNode, Yaml, YamlEmitter};
use age::Encryptor;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>
}

enum Commands {
    Test {
        #[arg(short, long)]
        list: bool,
    },
}

fn main() {
    let cli = Cli::parse();


    let docs = Yaml::load_from_str("[1, 2, 3]").unwrap();
    let doc = &docs[0]; // select the first YAML document
    assert_eq!(doc[0].as_integer().unwrap(), 1); // access elements by index

    let mut out_str = String::new();
    let mut emitter = YamlEmitter::new(&mut out_str);
    emitter.dump(doc).unwrap(); // dump the YAML object to a String
}
