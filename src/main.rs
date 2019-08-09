use clap::{App, SubCommand, Arg, Shell};

mod archive;
mod up;

fn main() {
    let args = arg_parser().get_matches();
    if let Some(up_args) = args.subcommand_matches("up") {
        up::run_command(up_args);
    } else if let Some(completion_args) = args.subcommand_matches("generate-shell-completions") {
        let shell = completion_args.value_of("SHELL").unwrap().parse().unwrap();
        arg_parser().gen_completions_to("agar", shell, &mut ::std::io::stdout());
    } else {
        arg_parser()
            .template("\
USAGE:
    {usage}

SUBCOMMANDS:
{subcommands}")
            .print_help()
            .expect("error printing help");
        ::std::process::exit(2);
    }
}

fn arg_parser<'a, 'b>() -> App<'a, 'b> {
    App::new("agar")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Archives (backs up) encrypted directories in Amazon Glacier vault.")
        .subcommand(up::args_spec().display_order(1))
        .subcommand(completions_spec().display_order(1001))
}

fn completions_spec<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("generate-shell-completions")
        .about("Generates shell completions for a number of popular shells")
        .arg(Arg::with_name("SHELL")
            .help("shell variant")
            .possible_values(Shell::variants().as_ref())
            .required(true))
}
