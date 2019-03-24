use clap::{App, Arg, SubCommand};
use rusoto_glacier::{Glacier, GlacierClient, ListVaultsInput};
use rusoto_core::region::Region;

fn main() {
    // example of parsing command line
    let args = App::new("agar")
        .version("0.1.0")
        .author("Maciek Makowski")
        .about("Archives (backs up) encrypted directories in Amazon Glacier vault.")
        // TODO: this can be defined in a separate module responsible for "up"
        .subcommand(SubCommand::with_name("up")
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
                .index(3)))
        .get_matches();

    if let Some(up_args) = args.subcommand_matches("up") {
        println!("dir: {:?}", up_args.value_of("DIRECTORY"));
    }

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
