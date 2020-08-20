use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Transaction {
    amount: i64,
    currency: String,
    merchant: String,
    timestamp: DateTime<Utc>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    #[serde(rename = "_id")]
    id: String,
    is_frozen: bool,
    card_number: String,
    transactions: Vec<Transaction>,
}
