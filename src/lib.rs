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
    pub type Symbol = String;
    pub type Decimal = String;
    pub type Quantity = String;

    pub use super::account::{ApiRestrictions, GetUserAsset};
    pub use super::spot::{
        OrderFill, OrderResponseFULL, OrderSide, OrderStatus, OrderType, SelfTradePreventionMode,
        ServerPing, ServerTime, TimeInForce,
    };
}
