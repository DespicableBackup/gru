use clap::{App, Arg, SubCommand};

pub fn get_app<'a>(app_name: &'a str, version: &'a str) -> App<'a, 'a> {
    App::new(app_name)
        .version(version)
        .subcommand(SubCommand::with_name("serve").about("Launch server"))
        .subcommand(SubCommand::with_name("list").about("List known minions"))
        .subcommand(
            SubCommand::with_name("create")
                .about("Add a new minion")
                .arg(Arg::with_name("NAME").required(true).index(1)),
        )
        .subcommand(
            SubCommand::with_name("regenerate")
                .about("Regenerate API token")
                .arg(Arg::with_name("NAME").required(true).index(1)),
        )
        .subcommand(
            SubCommand::with_name("revoke")
                .about("Revoke minion")
                .arg(Arg::with_name("NAME").required(true).index(1)),
        )
        .subcommand(
            SubCommand::with_name("delete")
                .about("Delete minion")
                .arg(Arg::with_name("NAME").required(true).index(1)),
        )
}
