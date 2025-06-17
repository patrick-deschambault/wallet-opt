use std::{error::Error, path::Path};

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

#[derive(Debug, Deserialize)]
pub struct RawPortfolio {
    pub holdings: Vec<RawHolding>,
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

    pub fn stock(&self) -> Stock {
        self.stock.clone()
    }

    pub async fn dividend_generated<P>(
        &self,
        provider: &P,
        end_date: &OffsetDateTime,
    ) -> Result<f64, Box<dyn Error>>
    where
        P: MarketDataProvider + Sync + Send,
    {
        let dividends = provider
            .get_dividends_per_share(
                self.stock.ticker(),
                self.stock.date(), // Date of buying the stock
                end_date,          // Use today for now..
            )
            .await?;

        let total: f64 = dividends
            .iter()
            .map(|(_, amount)| amount * self.quantity as f64)
            .sum();

        Ok(total)
    }
}

use std::fs;

pub async fn load_holdings_from_toml<P>(
    path: impl AsRef<Path>,
    provider: &P,
) -> Result<Vec<Holding>, Box<dyn std::error::Error>>
where
    P: MarketDataProvider + Sync + Send,
{
    let content = fs::read_to_string(path)?;
    let raw: RawPortfolio = toml::from_str(&content)?;

    let mut holdings = Vec::new();
    for raw_holding in raw.holdings {
        let datetime = raw_holding
            .date
            .with_time(time::Time::from_hms(16, 0, 0).unwrap()) // market close
            .assume_offset(time::UtcOffset::UTC);

        let stock = Stock::new(
            provider,
            &raw_holding.symbol.clone(),
            raw_holding.price_paid,
            datetime,
        )
        .await?;

        holdings.push(Holding {
            stock,
            quantity: raw_holding.quantity,
        });
    }

    Ok(holdings)
}
