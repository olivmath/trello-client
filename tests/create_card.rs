use assert_cmd::Command;

const CREATE_CARD_STR: &str = "Card added to board nearx-board: Name = \"Nova tarefa\", Label = \"curso web1\", Step = \"gravação\"\n";

#[test]
fn add_card_without_name() {
    Command::cargo_bin("trello - client")
        .unwrap()
        .args(["add", "card", "nearx - board"])
        .assert()
        .failure()
        .code(2);
}

#[test]
fn create_board() {
    Command::cargo_bin("trello - client")
        .unwrap()
        .args([
            "add",
            "card",
            "nearx - board",
            " - - name",
            "Nova tarefa",
            " - - label",
            "curso web1",
            " - - step",
            "gravação",
        ])
        .assert()
        .success()
        .stdout(CREATE_CARD_STR);
}
