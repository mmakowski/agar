use clap::App;

mod archive;
mod up;

fn main() {
    // example of parsing command line
    let args = App::new("agar")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Archives (backs up) encrypted directories in Amazon Glacier vault.")
        .subcommand(up::args_spec())
        .get_matches();

    if let Some(up_args) = args.subcommand_matches("up") {
        up::run_command(up_args);
    }
}
