use assert_cmd::Command;

const HELP_STR: &str = "Usage: trello-client <COMMAND>

Commands:
  add     Add resources
  get     Get resources
  move    Move cards
  edit    Edit resources
  remove  Remove resources
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
";

#[test]
fn help() {
    Command::cargo_bin("trello-client")
        .unwrap()
        .args(["help"])
        .assert()
        .success()
        .stdout(HELP_STR);
}
