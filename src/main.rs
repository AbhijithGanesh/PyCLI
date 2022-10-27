use clap::{ArgMatches, Command};

fn main() {
    let cli: ArgMatches = Command::new("Cassey CLI")
        .version("0.1.0")
        .about(format!(
            "{}",
            "This CLI will help you solve all your Python package and utilitarian needs"
        ))
        .after_help(
            "Longer explanation to appear after the options when \
                 displaying the help information from --help or -h",
        )
        .get_matches();
}
