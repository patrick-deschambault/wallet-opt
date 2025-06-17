use std::error::Error;

use time::OffsetDateTime;
use yahoo_finance_api::YahooConnector;

#[async_trait::async_trait]
pub trait MarketDataProvider {
    async fn get_price_at_date(
        &self,
        ticker: &str,
        date: &OffsetDateTime,
    ) -> Result<f64, Box<dyn Error>>;

    async fn is_ticker_valid(&self, ticker: &str) -> Result<bool, Box<dyn Error>>;

    async fn get_dividends_per_share(
        &self,
        ticker: &str,
        start: &OffsetDateTime,
        end: &OffsetDateTime,
    ) -> Result<Vec<(u64, f64)>, Box<dyn std::error::Error>>;
}

#[async_trait::async_trait]
impl MarketDataProvider for YahooConnector {
    async fn get_price_at_date(
        &self,
        ticker: &str,
        date: &OffsetDateTime,
    ) -> Result<f64, Box<dyn Error>> {
        // Plage de temps d'un jour autour de la date donnée
        let start = *date;
        let end = start + time::Duration::days(1);

        let response = self
            .get_quote_history(ticker, start, end)
            .await
            .map_err(|e| format!("Error fetching quote history: {}", e))?;

        let quotes = response
            .quotes()
            .map_err(|e| format!("Failed to parse quotes: {}", e))?;

        let quote = quotes
            .last() // On suppose que le dernier quote du jour est celui de clôture
            .ok_or("No price data available for the given date")?;

        Ok(quote.close)
    }

    async fn is_ticker_valid(&self, ticker: &str) -> Result<bool, Box<dyn Error>> {
        let response = self.get_latest_quotes(ticker, "1d").await;
        Ok(response.is_ok())
    }

    async fn get_dividends_per_share(
        &self,
        ticker: &str,
        start: &OffsetDateTime,
        end: &OffsetDateTime,
    ) -> Result<Vec<(u64, f64)>, Box<dyn std::error::Error>> {
        let response = self.get_quote_history(ticker, *start, *end).await?;
        let dividends = response.dividends()?; // Hypothétique

        let data = dividends
            .into_iter()
            .map(|entry| (entry.date, entry.amount))
            .collect();

        Ok(data)
    }
}
