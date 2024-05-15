use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::{
    http::client::{Client, ClientResult},
    types::{Price, Symbol},
};

impl Client {
    pub async fn server_ping(&self) -> ClientResult<ServerPing> {
        let mut url = self.base_url()?;
        url.set_path("/api/v3/ping");

        self.build_request_get(url).send().await
    }

    pub async fn server_time(&self) -> ClientResult<ServerTime> {
        let mut url = self.base_url()?;
        url.set_path("/api/v3/time");

        self.build_request_get(url).send().await
    }

    pub async fn exchange_info(&self, symbol: &Symbol) -> ClientResult<ExchangeInfo> {
        let mut url = self.base_url()?;
        url.set_path("/api/v3/exchangeInfo");

        {
            let mut query_pairs = url.query_pairs_mut();
            query_pairs.append_pair("symbol", symbol);
        }

        self.build_request_get(url).send().await
    }

    pub async fn exchange_infos(
        &self,
        symbols: Option<&Vec<Symbol>>,
    ) -> ClientResult<ExchangeInfo> {
        let mut url = self.base_url()?;
        url.set_path("/api/v3/exchangeInfo");

        {
            let mut query_pairs = url.query_pairs_mut();

            if let Some(value) = symbols {
                query_pairs.append_pair("symbols", &serde_json::to_string(value)?);
            }
        }

        self.build_request_get(url).send().await
    }

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

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerTime {
    #[serde(rename = "serverTime")]
    server_time: u128,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerPing {}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExchangeInfo {
    pub timezone: String,

    #[serde(rename = "serverTime")]
    pub server_time: u128,

    #[serde(rename = "rateLimits")]
    pub rate_limits: Vec<RateLimit>,

    #[serde(rename = "exchangeFilters")]
    pub exchange_filters: Vec<JsonValue>, // TODO

    pub symbols: Vec<SymbolInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RateLimit {
    #[serde(rename = "rateLimitType")]
    pub rate_limit_type: String,

    pub interval: String,

    #[serde(rename = "intervalNum")]
    pub interval_num: u8,

    pub limit: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SymbolInfo {
    pub symbol: Symbol,

    pub status: String,

    #[serde(rename = "baseAsset")]
    pub base_asset: Symbol,

    #[serde(rename = "baseAssetPrecision")]
    pub base_asset_precision: u8,

    #[serde(rename = "quoteAsset")]
    pub quote_asset: Symbol,

    #[serde(rename = "quotePrecision")]
    pub quote_precision: u8,

    #[serde(rename = "quoteAssetPrecision")]
    pub quote_asset_precision: u8,

    #[serde(rename = "baseCommissionPrecision")]
    pub base_commission_precision: u8,

    #[serde(rename = "quoteCommissionPrecision")]
    pub quote_commission_precision: u8,

    #[serde(rename = "orderTypes")]
    pub order_types: Vec<String>,

    #[serde(rename = "icebergAllowed")]
    pub iceberg_allowed: bool,

    #[serde(rename = "ocoAllowed")]
    pub oco_allowed: bool,

    #[serde(rename = "otoAllowed")]
    pub oto_allowed: bool,

    #[serde(rename = "quoteOrderQtyMarketAllowed")]
    pub quote_order_qty_market_allowed: bool,

    #[serde(rename = "allowTrailingStop")]
    pub allow_trailing_stop: bool,

    #[serde(rename = "cancelReplaceAllowed")]
    pub cancel_replace_allowed: bool,

    #[serde(rename = "isSpotTradingAllowed")]
    pub is_spot_trading_allowed: bool,

    #[serde(rename = "isMarginTradingAllowed")]
    pub is_margin_trading_allowed: bool,

    pub filters: Vec<Filter>,

    pub permissions: Vec<JsonValue>, // TODO

    #[serde(rename = "permissionSets")]
    pub permission_sets: Vec<Vec<String>>,

    #[serde(rename = "defaultSelfTradePreventionMode")]
    pub default_self_trade_prevention_mode: String,

    #[serde(rename = "allowedSelfTradePreventionModes")]
    pub allowed_self_trade_prevention_modes: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Filter {
    #[serde(rename = "filterType")]
    pub filter_type: String,

    #[serde(rename = "minPrice")]
    pub min_price: Option<String>,

    #[serde(rename = "maxPrice")]
    pub max_price: Option<String>,

    #[serde(rename = "tickSize")]
    pub tick_size: Option<String>,

    #[serde(rename = "minQty")]
    pub min_qty: Option<String>,

    #[serde(rename = "maxQty")]
    pub max_qty: Option<String>,

    #[serde(rename = "stepSize")]
    pub step_size: Option<String>,

    pub limit: Option<u32>,

    #[serde(rename = "minTrailingAboveDelta")]
    pub min_trailing_above_delta: Option<u32>,

    #[serde(rename = "maxTrailingAboveDelta")]
    pub max_trailing_above_delta: Option<u32>,

    #[serde(rename = "minTrailingBelowDelta")]
    pub min_trailing_below_delta: Option<u32>,

    #[serde(rename = "maxTrailingBelowDelta")]
    pub max_trailing_below_delta: Option<u32>,

    #[serde(rename = "bidMultiplierUp")]
    pub bid_multiplier_up: Option<String>,

    #[serde(rename = "bidMultiplierDown")]
    pub bid_multiplier_down: Option<String>,

    #[serde(rename = "askMultiplierUp")]
    pub ask_multiplier_up: Option<String>,

    #[serde(rename = "askMultiplierDown")]
    pub ask_multiplier_down: Option<String>,

    #[serde(rename = "avgPriceMins")]
    pub avg_price_mins: Option<u8>,

    #[serde(rename = "minNotional")]
    pub min_notional: Option<String>,

    #[serde(rename = "applyMinToMarket")]
    pub apply_min_to_market: Option<bool>,

    #[serde(rename = "maxNotional")]
    pub max_notional: Option<String>,

    #[serde(rename = "applyMaxToMarket")]
    pub apply_max_to_market: Option<bool>,

    #[serde(rename = "maxNumOrders")]
    pub max_num_orders: Option<u32>,

    #[serde(rename = "maxNumAlgoOrders")]
    pub max_num_algo_orders: Option<u32>,
}

#[cfg(test)]
mod tests {
    use crate::http::client::tests::client;

    #[tokio::test]
    async fn test_server_time() {
        let client = client();
        client.server_time().await.unwrap();
    }

    #[tokio::test]
    async fn test_server_ping() {
        let client = client();
        client.server_ping().await.unwrap();
    }

    #[tokio::test]
    async fn test_price() {
        let client = client();
        client.price(&"BTCUSDT".into()).await.unwrap();
    }

    #[tokio::test]
    async fn test_prices() {
        let client = client();
        client
            .prices(Some(&vec!["BTCUSDT".into(), "ETHUSDT".into()]))
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_exchange_info() {
        let client = client();
        client.exchange_info(&"BTCUSDT".into()).await.unwrap();
    }

    #[tokio::test]
    async fn test_exchange_infos() {
        let client = client();
        client
            .exchange_infos(Some(&vec!["BTCUSDT".into(), "ETHUSDT".into()]))
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_get_api_restrictions() {
        let client = client();
        client.prices(None).await.unwrap();
    }
}
