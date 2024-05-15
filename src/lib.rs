pub(crate) mod http;
pub(crate) mod time;

mod account;
mod market;
mod spot;

pub mod prelude {
    pub use super::http::client::{Client, ClientBuilder, ClientResult};
}

pub mod error {
    pub use super::http::error::{BinanceError, ClientError};
}

pub mod types {
    pub type Price = String;
    pub type Asset = String;
    pub type Symbol = String;
    pub type Decimal = String;
    pub type Quantity = String;
    pub type Commission = Decimal;

    pub use super::account::{
        ApiRestrictions, Balance, CommissionDetails, CommissionRates, DiscountDetails, SpotAccount,
        SpotCommission, UserAsset,
    };
    pub use super::market::{
        ExchangeInfo, Filter, RateLimit, ServerPing, ServerTime, SymbolInfo, SymbolPrice,
    };
    pub use super::spot::{
        OrderFill, OrderResponseFull, OrderSide, OrderStatus, OrderType, SelfTradePreventionMode,
        TimeInForce,
    };
}
