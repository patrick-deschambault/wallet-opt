use derive_builder::Builder;
use serde::{Deserialize, Deserializer};
use time::OffsetDateTime;

#[derive(Default, Builder, Debug)]
#[builder(setter(into))]
pub struct Wallet {
    stocks: Vec<Stock>,
    cash: f64,
}

#[derive(Clone, Default, Debug, Builder)]
pub struct Stock {
    ticker: String,
    quantity: i64,
    time_of_buy: Option<OffsetDateTime>,
}
