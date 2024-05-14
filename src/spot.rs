use serde::{Deserialize, Serialize};

use crate::{
    http::client::{Client, ClientResult}, time::timestamp, types::{Price, Symbol, Quantity}
};

// impl Client {
//     pub async fn market_buying_order(&self, order: &Order) -> ClientResult<SymbolPrice> {
//         let mut url = self.base_url()?;
//         url.set_path("/api/v3/order");

//         {
//             let mut query_pairs = url.query_pairs_mut();
//             query_pairs.append_pair("timestamp", &timestamp().as_millis().to_string());
//         }

//         self.build_request_get(url).send().await
//     }
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct SymbolPrice {
    symbol: Symbol,
    price: Price,
}

#[cfg(test)]
mod tests {
    use crate::http::client::tests::client;
}


#[derive(Debug, Serialize, Deserialize)]
pub enum OrderSide {
    Buy,
    Sell,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum OrderType {
    Limit,
    Market,
    StopLoss,
    StopLossLimit,
    TakeProfit,
    TakeProfitLimit,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TimeInForce {
    GTC,
    IOC,
    FOK,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum NewOrderRespType {
    ACK,
    RESULT,
    FULL,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SelfTradePreventionMode {
    ExpireTaker,
    ExpireMaker,
    ExpireBoth,
    None,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    symbol: String,

    side: OrderSide,

    #[serde(rename = "type")]
    type_: OrderType,

    #[serde(rename = "timeInForce")]
    time_in_force: Option<TimeInForce>,

    quantity: Option<f64>,

    #[serde(rename = "quoteOrderQty")]
    quote_order_qty: Option<Quantity>,

    price: Option<Price>,

    #[serde(rename = "newClientOrderId")]
    new_client_order_id: Option<String>,

    #[serde(rename = "stopPrice")]
    stop_price: Option<f64>,

    #[serde(rename = "trailingDelta")]
    trailing_delta: Option<i64>,

    #[serde(rename = "icebergQty")]
    iceberg_qty: Option<f64>,

    #[serde(rename = "newOrderRespType")]
    new_order_resp_type: Option<NewOrderRespType>,

    #[serde(rename = "selfTradePreventionMode")]
    self_trade_prevention_mode: Option<SelfTradePreventionMode>,

    #[serde(rename = "strategyId")]
    strategy_id: Option<i32>,

    #[serde(rename = "strategyType")]
    strategy_type: Option<i32>,
}


// 枚举定义：订单状态
#[derive(Debug, Serialize, Deserialize)]
pub enum OrderStatus {
    FILLED,
    PARTIALLY_FILLED,
    CANCELED,
    REJECTED,
    EXPIRED,
}


// 定义填充信息的结构体
#[derive(Debug, Serialize, Deserialize)]
pub struct Fill {
    price: String,
    qty: String,
    commission: String,
    #[serde(rename = "commissionAsset")]
    commission_asset: String,
    #[serde(rename = "tradeId")]
    trade_id: i64,
}

// 定义响应结构体
#[derive(Debug, Serialize, Deserialize)]
pub struct OrderResponse {
    symbol: Symbol,

    #[serde(rename = "orderId")]
    order_id: i64,

    #[serde(rename = "orderListId")]
    order_list_id: i64,

    #[serde(rename = "clientOrderId")]
    client_order_id: String,

    #[serde(rename = "transactTime")]
    transact_time: u128,

    price: Price,

    #[serde(rename = "origQty")]
    orig_qty: Quantity,

    #[serde(rename = "executedQty")]
    executed_qty: Quantity,

    #[serde(rename = "cummulativeQuoteQty")]
    cummulative_quote_qty: Quantity,

    status: OrderStatus,

    #[serde(rename = "timeInForce")]
    time_in_force: TimeInForce,

    type_: OrderType,

    side: OrderSide,

    #[serde(rename = "workingTime")]
    working_time: u128,

    #[serde(rename = "selfTradePreventionMode")]
    self_trade_prevention_mode: SelfTradePreventionMode,

    fills: Vec<Fill>,
}
