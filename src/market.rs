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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolPrice {
    pub symbol: Symbol,
    pub price: Price,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerTime {
    #[serde(rename = "serverTime")]
    pub server_time: u128,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerPing {}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimit {
    #[serde(rename = "rateLimitType")]
    pub rate_limit_type: String,

    pub interval: String,

    #[serde(rename = "intervalNum")]
    pub interval_num: u8,

    pub limit: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

    pub filters: Vec<symbol_filter::SymbolFilter>,

    pub permissions: Vec<JsonValue>, // TODO

    #[serde(rename = "permissionSets")]
    pub permission_sets: Vec<Vec<String>>,

    #[serde(rename = "defaultSelfTradePreventionMode")]
    pub default_self_trade_prevention_mode: String,

    #[serde(rename = "allowedSelfTradePreventionModes")]
    pub allowed_self_trade_prevention_modes: Vec<String>,
}

pub mod symbol_filter {
    use serde::{Deserialize, Serialize};

    use crate::types::{Decimal, Price, Quantity};

    // PRICE_FILTER
    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct SymbolPriceFilter {
        #[serde(rename = "minPrice")]
        pub min_price: Price,

        #[serde(rename = "maxPrice")]
        pub max_price: Price,

        #[serde(rename = "tickSize")]
        pub tick_size: Decimal,
    }

    // PERCENT_PRICE
    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct SymbolPercentPriceFilter {
        #[serde(rename = "multiplierUp")]
        pub multiplier_up: Decimal,

        #[serde(rename = "multiplierDown")]
        pub multiplier_down: Decimal,

        #[serde(rename = "avgPriceMins")]
        pub avg_price_mins: u32,
    }

    // PERCENT_PRICE_BY_SIDE
    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct SymbolPercentPriceBySideFilter {
        #[serde(rename = "bidMultiplierUp")]
        pub bid_multiplier_up: Decimal,

        #[serde(rename = "bidMultiplierDown")]
        pub bid_multiplier_down: Decimal,

        #[serde(rename = "askMultiplierUp")]
        pub ask_multiplier_up: Decimal,

        #[serde(rename = "askMultiplierDown")]
        pub ask_multiplier_down: Decimal,

        #[serde(rename = "avgPriceMins")]
        pub avg_price_mins: u32,
    }

    // LOT_SIZE
    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct SymbolLotSizeFilter {
        #[serde(rename = "minQty")]
        pub min_qty: Quantity,

        #[serde(rename = "maxQty")]
        pub max_qty: Quantity,

        #[serde(rename = "stepSize")]
        pub step_size: Decimal,
    }

    // MIN_NOTIONAL
    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct SymbolMinNotionalFilter {
        #[serde(rename = "minNotional")]
        pub min_notional: Quantity,

        #[serde(rename = "applyToMarket")]
        pub apply_to_market: bool,

        #[serde(rename = "avgPriceMins")]
        pub avg_price_mins: u32,
    }

    // NOTIONAL
    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct SymbolNotionalFilter {
        #[serde(rename = "minNotional")]
        pub min_notional: Quantity,

        #[serde(rename = "applyMinToMarket")]
        pub apply_min_to_market: bool,

        #[serde(rename = "maxNotional")]
        pub max_notional: Quantity,

        #[serde(rename = "applyMaxToMarket")]
        pub apply_max_to_market: bool,

        #[serde(rename = "avgPriceMins")]
        pub avg_price_mins: u32,
    }

    // ICEBERG_PARTS
    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct SymbolIcebergPartsfilter {
        pub limit: u32,
    }

    // MARKET_LOT_SIZE
    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct SymbolMarketLotSizeFilter {
        #[serde(rename = "minQty")]
        pub min_qty: Quantity,

        #[serde(rename = "maxQty")]
        pub max_qty: Quantity,

        #[serde(rename = "stepSize")]
        pub step_size: Decimal,
    }

    // MAX_NUM_ORDERS
    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct SymbolMaxNumOrdersFilter {
        #[serde(rename = "maxNumOrders")]
        pub max_num_orders: i64,
    }

    // MAX_NUM_ALGO_ORDERS
    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct SymbolMaxNumAlgoOrdersFilter {
        #[serde(rename = "maxNumAlgoOrders")]
        pub max_num_algo_orders: u32,
    }

    // MAX_NUM_ICEBERG_ORDERS
    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct SymbolMaxNumIcebergOrdersFilter {
        #[serde(rename = "maxNumIcebergOrders")]
        pub max_num_iceberg_orders: u32,
    }

    // MAX_POSITION
    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct SymbolMaxPositionFilter {
        #[serde(rename = "maxPosition")]
        pub max_position: Decimal,
    }

    // TRAILING_DELTA
    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct SymbolTrailingDeltaFilter {
        #[serde(rename = "minTrailingAboveDelta")]
        pub min_trailing_above_delta: i32,

        #[serde(rename = "maxTrailingAboveDelta")]
        pub max_trailing_above_delta: i32,

        #[serde(rename = "minTrailingBelowDelta")]
        pub min_trailing_below_delta: i32,

        #[serde(rename = "maxTrailingBelowDelta")]
        pub max_trailing_below_delta: i32,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(tag = "filterType", rename_all = "camelCase")]
    pub enum SymbolFilter {
        #[serde(rename = "PRICE_FILTER")]
        PriceFilter(SymbolPriceFilter),

        #[serde(rename = "PERCENT_PRICE")]
        PercentPrice(SymbolPercentPriceFilter),

        #[serde(rename = "PERCENT_PRICE_BY_SIDE")]
        PercentPriceBySide(SymbolPercentPriceBySideFilter),

        #[serde(rename = "LOT_SIZE")]
        LotSize(SymbolLotSizeFilter),

        #[serde(rename = "MIN_NOTIONAL")]
        MinNotional(SymbolMinNotionalFilter),

        #[serde(rename = "NOTIONAL")]
        Notional(SymbolNotionalFilter),

        #[serde(rename = "ICEBERG_PARTS")]
        IcebergParts(SymbolIcebergPartsfilter),

        #[serde(rename = "MARKET_LOT_SIZE")]
        MarketLotSize(SymbolMarketLotSizeFilter),

        #[serde(rename = "MAX_NUM_ORDERS")]
        MaxNumOrders(SymbolMaxNumOrdersFilter),

        #[serde(rename = "MAX_NUM_ALGO_ORDERS")]
        MaxNumAlgoOrders(SymbolMaxNumAlgoOrdersFilter),

        #[serde(rename = "MAX_NUM_ICEBERG_ORDERS")]
        MaxNumIcebergOrders(SymbolMaxNumIcebergOrdersFilter),

        #[serde(rename = "MAX_POSITION")]
        MaxPosition(SymbolMaxPositionFilter),

        #[serde(rename = "TRAILING_DELTA")]
        TrailingDelta(SymbolTrailingDeltaFilter),
    }
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

    #[test]
    fn test_symbol_filters_serde() {
        use super::symbol_filter::SymbolFilter;

        let json_data = r#"[{"filterType":"PRICE_FILTER","minPrice":"0.00000100","maxPrice":"100000.00000000","tickSize":"0.00000100"},{"filterType":"PERCENT_PRICE","multiplierUp":"5","multiplierDown":"0.2","avgPriceMins":5},{"filterType":"PERCENT_PRICE_BY_SIDE","bidMultiplierUp":"1.2","bidMultiplierDown":"0.2","askMultiplierUp":"5","askMultiplierDown":"0.8","avgPriceMins":1},{"filterType":"LOT_SIZE","minQty":"0.00100000","maxQty":"100000.00000000","stepSize":"0.00100000"},{"filterType":"MIN_NOTIONAL","minNotional":"0.00100000","applyToMarket":true,"avgPriceMins":5},{"filterType":"NOTIONAL","minNotional":"10.00000000","applyMinToMarket":false,"maxNotional":"10000.00000000","applyMaxToMarket":false,"avgPriceMins":5},{"filterType":"ICEBERG_PARTS","limit":10},{"filterType":"MARKET_LOT_SIZE","minQty":"0.00100000","maxQty":"100000.00000000","stepSize":"0.00100000"},{"filterType":"MAX_NUM_ORDERS","maxNumOrders":25},{"filterType":"MAX_NUM_ALGO_ORDERS","maxNumAlgoOrders":5},{"filterType":"MAX_NUM_ICEBERG_ORDERS","maxNumIcebergOrders":5},{"filterType":"MAX_POSITION","maxPosition":"10.00000000"},{"filterType":"TRAILING_DELTA","minTrailingAboveDelta":10,"maxTrailingAboveDelta":2000,"minTrailingBelowDelta":10,"maxTrailingBelowDelta":2000}]"#;

        let filters: Vec<SymbolFilter> = serde_json::from_str(json_data).unwrap();
        for i in filters.iter() {
            match i {
                SymbolFilter::PriceFilter(v) => {
                    assert_eq!(v.max_price, "100000.00000000");
                    assert_eq!(v.min_price, "0.00000100");
                    assert_eq!(v.tick_size, "0.00000100")
                }
                _ => continue,
            }
        }
        assert_eq!(filters.len(), 13);

        assert_eq!(serde_json::to_string(&filters).unwrap(), json_data);
    }
}
