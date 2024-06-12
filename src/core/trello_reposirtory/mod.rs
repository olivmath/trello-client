mod add_card;
mod edit_card;
mod utils;

use add_card::add_card;

pub struct Trello {}

impl Trello {
    pub async fn add_card(name: &str, label: &str, step: &str) {
        add_card(name, label, step).await;
    }

    pub fn get_card() {}

    pub fn get_all_cards() {}

    pub fn edit_card() {}

    pub fn remove_card() {}
}
