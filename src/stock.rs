use std::error::Error;

use getset::Getters;
use time::OffsetDateTime;

use crate::market_data_provider::MarketDataProvider;

#[derive(Clone, Debug)]
pub enum StockOrigin {
    UserDefined,
    MarketProvider,
}

#[derive(Clone, Debug, Getters)]
pub struct Stock {
    #[get = "pub"]
    ticker: String,
    #[get = "pub"]
    price: f64,
    #[get = "pub"]
    date: OffsetDateTime,
    #[get = "pub"]
    origin: StockOrigin,
}

impl Stock {
    pub async fn new<P>(
        provider: &P,
        ticker: &str,
        price: f64,
        date: OffsetDateTime,
    ) -> Result<Stock, Box<dyn Error>>
    where
        P: MarketDataProvider + Sync + Send,
    {
        let _ = provider.is_ticker_valid(ticker).await?;

        Ok(Self {
            ticker: ticker.into(),
            price,
            date,
            origin: StockOrigin::UserDefined,
        })
    }

    pub async fn from_market<P>(
        provider: &P,
        ticker: &str,
        date: &OffsetDateTime,
    ) -> Result<Stock, Box<dyn Error>>
    where
        P: MarketDataProvider + Sync + Send,
    {
        let price = provider.get_price_at_date(&ticker, &date).await?;

        Ok(Self {
            ticker: ticker.to_string(),
            price,
            date: date.clone(),
            origin: StockOrigin::MarketProvider,
        })
    }
}

#[async_trait::async_trait]
pub trait StockBuilderExt {
    async fn build<P: MarketDataProvider + Sync + Send>(
        self,
        provider: &P,
    ) -> Result<Stock, Box<dyn std::error::Error>>;
}
