mod add_webhook;
mod delete_webhook;
mod get_webhook;
mod update_webhook;

use std::fmt::Display;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Webhook {
    id: String,
    description: String,
    #[serde(rename = "callbackURL")]
    callback_url: String,
    active: bool,
}

impl Display for Webhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "---------\nWebhook: {} \nActive: {:?} \nCallback: {} \n---------",
            self.description, self.active, self.callback_url,
        )
    }
}

impl Webhook {
    pub async fn add_webhook(name: &str, callback: &str) {
        add_webhook::add_webhook(name, callback).await;
    }

    pub async fn get_webhook(id: Option<String>, all: bool) {
        if all {
            get_webhook::get_all_webhook().await;
        } else if let Some(id) = id {
            get_webhook::get_webhook_by_id(&id).await;
        }
    }

    pub(crate) async fn remove_webhook(all: bool, id: Option<String>) {
        if all {
            delete_webhook::delete_all_webhook().await;
        } else if let Some(id) = id {
            delete_webhook::delete_webhook_by_id(&id).await;
        }
    }

    pub(crate) async fn update_webhook_by_id(id: &str, callback: &str, active: Option<bool>) {
        update_webhook::update_webhook_by_id(id, callback, active).await;
    }
}
