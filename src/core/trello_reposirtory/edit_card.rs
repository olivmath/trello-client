use crate::core::card::LabelsData;

use super::utils::put;
use serde_json::json;
use std::env::var;

pub(super) async fn edit_card(card_id: &str, label: &str, step: &str) {
    let mut base_url = var("BASE_URL").expect("BASE_URL must be set");
    base_url.push_str("/cards/");
    base_url.push_str(card_id);

    let color = LabelsData::get_color_by_id(label);
    let body = json!({
        "idCardSource": var("TEMPLATE_CARD_ID").expect("TEMPLATE_CARD_ID must be set"),
        "idList": step,
        "idLabels": vec![label],
        "cover": {
            "color": color,
            "size": "full"
        }
    });

    put(&base_url, body, None).await;
}
