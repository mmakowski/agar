use std::path::PathBuf;
use clap::{App, Arg, ArgMatches, SubCommand};
use rusoto_glacier::{Glacier, GlacierClient, ListVaultsInput};
use rusoto_core::region::Region;

use super::archive;

pub fn args_spec<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("up")
                .about("Uploads an archive to Glacier")
                .arg(Arg::with_name("DIRECTORY")
                    .help("the directory to upload")
                    .required(true)
                    .index(1))
                .arg(Arg::with_name("CATEGORY")
                    .help("the category to index the archive under")
                    .required(true)
                    .index(2))
                .arg(Arg::with_name("PASSWORD_REMINDER")
                    .help("password reminder")
                    .required(true)
                    .index(3))
}

pub fn run_command(args: &ArgMatches) {
    println!("dir: {:?}", args.value_of("DIRECTORY"));
    
    let archive_file = archive::archive(PathBuf::from(args.value_of("DIRECTORY").expect("directory not specified")).as_path(),
                                        PathBuf::from(".").as_path())
                                .expect("archiving failed");

    // TODO: make sure config is initialised in ~/.aws
    let glacier_client = GlacierClient::new(Region::default());
    let input = ListVaultsInput { account_id: String::from("-"), limit: None, marker: None }; // note: default() does not work;
    let response = glacier_client.list_vaults(input);
    match response.sync() {
        Ok(output) => match output.vault_list {
            Some(vec) => vec.iter().for_each(|v| println!("Vault: {:?}", v)),
            None => println!("No vaults present!")
        },
        Err(error) => println!("Error: {:?}", error)
    }
}
