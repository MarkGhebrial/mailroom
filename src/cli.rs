use clap::Command;

/// Generate the command line interface via clap
pub fn cli() -> Command {
    Command::new("mailroom")
        .about("An email server.")
        .subcommand_required(false)
        .arg_required_else_help(false)
        .allow_external_subcommands(false)
        .subcommand(Command::new("config").about("View and edit the server configuration."))
}
