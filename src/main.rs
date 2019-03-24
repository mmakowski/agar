use rusoto_glacier::{Glacier, GlacierClient, ListVaultsInput};
use rusoto_core::region::Region;

fn main() {
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
