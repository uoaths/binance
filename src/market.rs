use serde::{Deserialize, Serialize};

use crate::{
    http::client::{Client, ClientResult},
    types::{Price, Symbol},
};

impl Client {
    pub async fn price(&self, symbol: &Symbol) -> ClientResult<SymbolPrice> {
        let mut url = self.base_url()?;
        url.set_path("/api/v3/ticker/price");

        {
            let mut query_pairs = url.query_pairs_mut();
            query_pairs.append_pair("symbol", symbol);
        }

        self.build_request_get(url).send().await
    }

    pub async fn prices(&self, symbols: Option<&Vec<Symbol>>) -> ClientResult<Vec<SymbolPrice>> {
        let mut url = self.base_url()?;
        url.set_path("/api/v3/ticker/price");

        {
            let mut query_pairs = url.query_pairs_mut();

            if let Some(value) = symbols {
                query_pairs.append_pair("symbols", &serde_json::to_string(value)?);
            }
        }

        self.build_request_get(url).send().await
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SymbolPrice {
    symbol: Symbol,
    price: Price,
}

#[cfg(test)]
mod tests {
    use crate::http::client::tests::client;

    #[tokio::test]
    async fn test_price() {
        let client = client();
        client.price(&"BTCUSDT".to_string()).await.unwrap();
    }

    #[tokio::test]
    async fn test_get_api_restrictions() {
        let client = client();
        client.prices(None).await.unwrap();
    }
}
