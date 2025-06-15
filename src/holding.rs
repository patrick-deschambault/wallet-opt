use std::error::Error;

use crate::{market_data_provider::MarketDataProvider, stock::Stock};

use serde::Deserialize;
use time::{Date, OffsetDateTime, Time, UtcOffset};

#[derive(Debug, Deserialize)]
pub struct RawHolding {
    pub symbol: String,
    pub quantity: u32,
    pub price_paid: f64,
    pub date: Date,
}

#[derive(Debug)]
pub struct Holding {
    stock: Stock,
    quantity: u32,
}

impl Holding {
    pub fn new(stock: Stock, quantity: u32) -> Self {
        Self { stock, quantity }
    }

    pub async fn from_raw<P: MarketDataProvider + Sync + Send>(
        raw: RawHolding,
        provider: &P,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let naive_datetime = raw.date.with_time(Time::MIDNIGHT);
        let datetime = naive_datetime.assume_offset(UtcOffset::UTC);

        let stock = Stock::new(provider, &raw.symbol, raw.price_paid, datetime).await?;

        Ok(Holding {
            stock,
            quantity: raw.quantity,
        })
    }

    // Méthode pour calculer la valeur initiale de cet holding
    pub fn initial_value(&self) -> f64 {
        self.quantity as f64 * self.stock.price()
    }

    // Tu peux aussi ajouter des méthodes pour calculer la valeur actuelle
    // ou le gain si tu as un prix actuel
    pub async fn value_with_date<P>(
        &self,
        provider: &P,
        date: &OffsetDateTime,
    ) -> Result<f64, Box<dyn Error>>
    where
        P: MarketDataProvider + Sync + Send,
    {
        let current_price = provider
            .get_price_at_date(self.stock.ticker(), date)
            .await?;

        Ok(self.quantity as f64 * current_price)
    }
}
