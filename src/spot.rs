use serde::{Deserialize, Serialize};

use crate::{
    http::client::{Client, ClientResult},
    time::timestamp,
    types::{Asset, Decimal, Price, Quantity, Symbol},
};

impl Client {
    pub async fn spot_market_order_with_quote(
        &self,
        symbol: &Symbol,
        side: OrderSide,
        quote_quantity: &Quantity,
        recv_window: Option<u8>,
    ) -> ClientResult<OrderResponseFull> {
        let mut url = self.base_url()?;
        url.set_path("/api/v3/order");

        {
            let mut query_pairs = url.query_pairs_mut();

            query_pairs.append_pair("symbol", symbol);
            query_pairs.append_pair("side", side.as_str());
            query_pairs.append_pair("type", "MARKET");
            query_pairs.append_pair("newOrderRespType", "FULL");
            query_pairs.append_pair("quoteOrderQty", quote_quantity);

            if let Some(value) = recv_window {
                query_pairs.append_pair("recvWindow", &value.to_string());
            }

            query_pairs.append_pair("timestamp", &timestamp().as_millis().to_string());
        }

        self.build_sign_request_post(url)?
            .with_api_key(self.secret.api_key()?)
            .send()
            .await
    }

    pub async fn spot_market_order_with_base(
        &self,
        symbol: &Symbol,
        side: OrderSide,
        base_quantity: &Quantity,
        recv_window: Option<u8>,
    ) -> ClientResult<OrderResponseFull> {
        let mut url = self.base_url()?;
        url.set_path("/api/v3/order");

        {
            let mut query_pairs = url.query_pairs_mut();

            query_pairs.append_pair("symbol", symbol);
            query_pairs.append_pair("side", side.as_str());
            query_pairs.append_pair("type", "MARKET");
            query_pairs.append_pair("newOrderRespType", "FULL");
            query_pairs.append_pair("quantity", &base_quantity);

            if let Some(value) = recv_window {
                query_pairs.append_pair("recvWindow", &value.to_string());
            }

            query_pairs.append_pair("timestamp", &timestamp().as_millis().to_string());
        }

        self.build_sign_request_post(url)?
            .with_api_key(self.secret.api_key()?)
            .send()
            .await
    }

    pub async fn spot_order_info(
        &self,
        symbol: &Symbol,
        id: i64,
        recv_window: Option<u8>,
    ) -> ClientResult<OrderInfo> {
        let mut url = self.base_url()?;
        url.set_path("/api/v3/order");

        {
            let mut query_pairs = url.query_pairs_mut();

            if let Some(value) = recv_window {
                query_pairs.append_pair("recvWindow", &value.to_string());
            }

            query_pairs.append_pair("symbol", symbol);
            query_pairs.append_pair("orderId", &id.to_string());
            query_pairs.append_pair("timestamp", &timestamp().as_millis().to_string());
        }

        self.build_sign_request_get(url)?
            .with_api_key(self.secret.api_key()?)
            .send()
            .await
    }

    pub async fn spot_all_order_info(
        &self,
        symbol: &Symbol,
        id: Option<i64>,
        start_time: Option<u128>,
        end_time: Option<u128>,
        limit: Option<u16>,
        recv_window: Option<u8>,
    ) -> ClientResult<Vec<OrderInfo>> {
        let mut url = self.base_url()?;
        url.set_path("/api/v3/allOrders");

        {
            let mut query_pairs = url.query_pairs_mut();

            if let Some(value) = id {
                query_pairs.append_pair("orderId", &value.to_string());
            }

            if let Some(value) = start_time {
                query_pairs.append_pair("startTime", &value.to_string());
            }

            if let Some(value) = end_time {
                query_pairs.append_pair("endTime", &value.to_string());
            }

            if let Some(value) = limit {
                query_pairs.append_pair("limit", &value.to_string());
            }

            if let Some(value) = recv_window {
                query_pairs.append_pair("recvWindow", &value.to_string());
            }

            query_pairs.append_pair("symbol", symbol);
            query_pairs.append_pair("timestamp", &timestamp().as_millis().to_string());
        }

        self.build_sign_request_get(url)?
            .with_api_key(self.secret.api_key()?)
            .send()
            .await
    }

    pub async fn spot_trade(
        &self,
        symbol: &Symbol,
        id: i64,
        recv_window: Option<u8>,
    ) -> ClientResult<Vec<Trade>> {
        self.spot_trades(symbol, Some(id), None, None, None, None, recv_window)
            .await
    }

    pub async fn spot_trades(
        &self,
        symbol: &Symbol,
        id: Option<i64>,
        start_time: Option<u128>,
        end_time: Option<u128>,
        from_id: Option<i64>,
        limit: Option<u16>,
        recv_window: Option<u8>,
    ) -> ClientResult<Vec<Trade>> {
        let mut url = self.base_url()?;
        url.set_path("/api/v3/myTrades");

        {
            let mut query_pairs = url.query_pairs_mut();

            if let Some(value) = id {
                query_pairs.append_pair("orderId", &value.to_string());
            }

            if let Some(value) = start_time {
                query_pairs.append_pair("startTime", &value.to_string());
            }

            if let Some(value) = end_time {
                query_pairs.append_pair("endTime", &value.to_string());
            }

            if let Some(value) = from_id {
                query_pairs.append_pair("fromId", &value.to_string());
            }

            if let Some(value) = limit {
                query_pairs.append_pair("limit", &value.to_string());
            }

            if let Some(value) = recv_window {
                query_pairs.append_pair("recvWindow", &value.to_string());
            }

            query_pairs.append_pair("symbol", symbol);
            query_pairs.append_pair("timestamp", &timestamp().as_millis().to_string());
        }

        self.build_sign_request_get(url)?
            .with_api_key(self.secret.api_key()?)
            .send()
            .await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderInfo {
    pub symbol: Symbol,

    #[serde(rename = "orderId")]
    pub order_id: i64,

    #[serde(rename = "orderListId")]
    pub order_list_id: i64,

    #[serde(rename = "clientOrderId")]
    pub client_order_id: String,

    pub price: Price,

    #[serde(rename = "origQty")]
    pub orig_qty: Quantity,

    #[serde(rename = "executedQty")]
    pub executed_qty: Quantity,

    #[serde(rename = "cummulativeQuoteQty")]
    pub cummulative_quote_qty: Quantity,

    pub status: OrderStatus,

    #[serde(rename = "timeInForce")]
    pub time_in_force: String,

    #[serde(rename = "type")]
    pub order_type: OrderType,

    pub side: OrderSide,

    #[serde(rename = "stopPrice")]
    pub stop_price: Price,

    #[serde(rename = "icebergQty")]
    pub iceberg_qty: String,

    pub time: u128,

    #[serde(rename = "updateTime")]
    pub update_time: u128,

    #[serde(rename = "isWorking")]
    pub is_working: bool,

    #[serde(rename = "workingTime")]
    pub working_time: u128,

    #[serde(rename = "origQuoteOrderQty")]
    pub orig_quote_order_qty: Quantity,

    #[serde(rename = "selfTradePreventionMode")]
    pub self_trade_prevention_mode: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum OrderSide {
    #[serde(rename = "BUY")]
    Buy,

    #[serde(rename = "SELL")]
    Sell,
}

impl OrderSide {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Buy => "BUY",
            Self::Sell => "SELL",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SelfTradePreventionMode {
    None,

    #[serde(rename = "EXPIRE_TAKER")]
    ExpireTaker,

    #[serde(rename = "EXPIRE_MAKER")]
    ExpireMaker,

    #[serde(rename = "EXPIRE_BOTH")]
    ExpireBoth,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum OrderStatus {
    #[serde(rename = "NEW")]
    New,

    #[serde(rename = "PARTIALLY_FILLED")]
    PartiallyFilled,

    #[serde(rename = "FILLED")]
    Filled,

    #[serde(rename = "CANCELED")]
    Canceled,

    #[serde(rename = "PENDING_CANCEL")]
    PendingCancel,

    #[serde(rename = "REJECTED")]
    Rejected,

    #[serde(rename = "EXPIRED")]
    Expired,

    #[serde(rename = "EXPIRED_IN_MATCH")]
    ExpiredInMatch,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TimeInForce {
    #[serde(rename = "GTC")]
    Gtc,

    #[serde(rename = "IOC")]
    Ioc,

    #[serde(rename = "FOK")]
    Fok,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum OrderType {
    #[serde(rename = "LIMIT")]
    Limit,

    #[serde(rename = "MARKET")]
    Market,

    #[serde(rename = "STOP_LOSS")]
    StopLoss,

    #[serde(rename = "STOP_LOSS_LIMIT")]
    StopLossLimit,

    #[serde(rename = "TAKE_PROFIT")]
    TakeProfit,

    #[serde(rename = "TAKE_PROFIT_LIMIT")]
    TakeProfitLimit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderFill {
    pub price: Price,

    pub qty: Quantity,

    pub commission: Quantity,

    #[serde(rename = "commissionAsset")]
    pub commission_asset: Symbol,

    #[serde(rename = "tradeId")]
    pub trade_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderResponseFull {
    pub symbol: Symbol,

    #[serde(rename = "orderId")]
    pub order_id: i64,

    #[serde(rename = "orderListId")]
    pub order_list_id: i64,

    #[serde(rename = "clientOrderId")]
    pub client_order_id: String,

    #[serde(rename = "transactTime")]
    pub transact_time: u128,

    pub price: Price,

    #[serde(rename = "origQty")]
    pub orig_qty: Quantity,

    #[serde(rename = "executedQty")]
    pub executed_qty: Quantity,

    #[serde(rename = "cummulativeQuoteQty")]
    pub cummulative_quote_qty: Quantity,

    pub status: OrderStatus,

    #[serde(rename = "timeInForce")]
    pub time_in_force: TimeInForce,

    #[serde(rename = "type")]
    pub order_type: OrderType,

    pub side: OrderSide,

    #[serde(rename = "workingTime")]
    pub working_time: u128,

    #[serde(rename = "selfTradePreventionMode")]
    pub self_trade_prevention_mode: SelfTradePreventionMode,

    pub fills: Vec<OrderFill>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trade {
    pub symbol: Symbol,
    pub id: i64,
    pub price: Price,
    pub qty: Quantity,
    pub commission: Decimal,
    pub time: u128,

    #[serde(rename = "orderId")]
    pub order_id: u64,

    #[serde(rename = "orderListId")]
    pub order_list_id: i64,

    #[serde(rename = "quoteQty")]
    pub quote_qty: Quantity,

    #[serde(rename = "commissionAsset")]
    pub commission_asset: Asset,

    #[serde(rename = "isBuyer")]
    pub is_buyer: bool,

    #[serde(rename = "isMaker")]
    pub is_maker: bool,

    #[serde(rename = "isBestMatch")]
    pub is_best_match: bool,
}

#[cfg(test)]
mod tests {
    use super::OrderSide;

    use crate::http::client::tests::client_with_test_net_key_secret;

    #[tokio::test]
    async fn test_spot_market_order() {
        let client = client_with_test_net_key_secret();
        client
            .spot_market_order_with_quote(
                &"BTCUSDT".into(),
                OrderSide::Buy,
                &"10.14159".into(),
                None,
            )
            .await
            .unwrap();

        client
            .spot_market_order_with_quote(
                &"BTCUSDT".into(),
                OrderSide::Sell,
                &"10.14159".into(),
                None,
            )
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_spot_market_order_with_base() {
        let client = client_with_test_net_key_secret();
        client
            .spot_market_order_with_base(&"BTCUSDT".into(), OrderSide::Buy, &"0.0001".into(), None)
            .await
            .unwrap();

        client
            .spot_market_order_with_base(&"BTCUSDT".into(), OrderSide::Sell, &"0.0001".into(), None)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_spot_order_info() {
        let client = client_with_test_net_key_secret();
        let order = client
            .spot_market_order_with_quote(
                &"BTCUSDT".into(),
                OrderSide::Buy,
                &"10.14159".into(),
                None,
            )
            .await
            .unwrap();

        client
            .spot_order_info(&order.symbol, order.order_id, None)
            .await
            .unwrap();

        client
            .spot_all_order_info(&order.symbol, None, None, None, None, None)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_spot_trade() {
        let client = client_with_test_net_key_secret();
        let order = client
            .spot_market_order_with_quote(
                &"BTCUSDT".into(),
                OrderSide::Buy,
                &"10.14159".into(),
                None,
            )
            .await
            .unwrap();

        client
            .spot_trade(&"BTCUSDT".into(), order.order_id, None)
            .await
            .unwrap();
        client
            .spot_trades(&"BTCUSDT".into(), None, None, None, None, None, None)
            .await
            .unwrap();
    }
}
