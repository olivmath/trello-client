use crate::core::card::Steps;

pub async fn get_all_cards_from_step(step: Steps) {
    println!("get_all_cards_from_step: {:?}", step);
}
pub async fn get_all_cards() {
    println!("get_all_cards");
}
pub async fn get_card(id: String) {
    println!("get_card: {}", id);
}
