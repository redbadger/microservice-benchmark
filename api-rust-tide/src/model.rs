use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Deserialize, Serialize)]
pub struct AccountTransaction {
    amount: i64,
    currency: String,
    from: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    #[serde(rename = "_id")]
    id: String,
    index: u32,
    guid: uuid::Uuid,
    is_active: bool,
    balance: String,
    transactions: Vec<AccountTransaction>,
}

#[derive(Deserialize, Serialize)]
pub struct CardTransaction {
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
    transactions: Vec<CardTransaction>,
}

#[derive(Deserialize, Serialize)]
pub struct Name {
    first: String,
    last: String,
}

#[derive(Deserialize, Serialize)]
pub struct Address {
    line1: String,
    line2: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Customer {
    #[serde(rename = "_id")]
    id: String,
    index: u32,
    guid: uuid::Uuid,
    is_active: bool,
    balance: String,
    picture: Url,
    name: Name,
    company: String,
    email: String,
    phone: String,
    addresses: Vec<Address>,
    about: String,
    registered: DateTime<Utc>,
    latitude: f64,
    longitude: f64,
    tags: Vec<String>,
    pub accounts: Option<Vec<Account>>,
    pub cards: Option<Vec<Card>>,
}
