use serde::{Deserialize, Serialize};

use crate::{
    http::client::{Client, ClientResult},
    time::timestamp,
    types::{Asset, Commission, Decimal, Quantity, Symbol},
};

impl Client {
    pub async fn user_asset(
        &self,
        asset: Option<&String>,
        need_btc_valuation: Option<bool>,
        recv_window: Option<u8>,
    ) -> ClientResult<Vec<UserAsset>> {
        let mut url = self.base_url()?;
        url.set_path("/sapi/v3/asset/getUserAsset");

        {
            let mut query_pairs = url.query_pairs_mut();

            if let Some(value) = asset {
                query_pairs.append_pair("asset", value);
            }

            if let Some(value) = need_btc_valuation {
                query_pairs.append_pair("needBtcValuation", &value.to_string());
            }

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

    pub async fn api_restrictions(&self, recv_window: Option<u8>) -> ClientResult<ApiRestrictions> {
        let mut url = self.base_url()?;
        url.set_path("/sapi/v1/account/apiRestrictions");

        {
            let mut query_pairs = url.query_pairs_mut();

            if let Some(value) = recv_window {
                query_pairs.append_pair("recvWindow", &value.to_string());
            }

            query_pairs.append_pair("timestamp", &timestamp().as_millis().to_string());
        }

        self.build_sign_request_get(url)?
            .with_api_key(self.secret.api_key()?)
            .send()
            .await
    }

    pub async fn spot_account(
        &self,
        omit_zero_balances: Option<bool>,
        recv_window: Option<u8>,
    ) -> ClientResult<SpotAccount> {
        let mut url = self.base_url()?;
        url.set_path("/api/v3/account");

        {
            let mut query_pairs = url.query_pairs_mut();

            if let Some(value) = omit_zero_balances {
                query_pairs.append_pair("omitZeroBalances", &value.to_string());
            }

            if let Some(value) = recv_window {
                query_pairs.append_pair("recvWindow", &value.to_string());
            }

            query_pairs.append_pair("timestamp", &timestamp().as_millis().to_string());
        }

        self.build_sign_request_get(url)?
            .with_api_key(self.secret.api_key()?)
            .send()
            .await
    }

    pub async fn spot_commission(&self, symbol: &Symbol) -> ClientResult<SpotCommission> {
        let mut url = self.base_url()?;
        url.set_path("/api/v3/account/commission");

        {
            let mut query_pairs = url.query_pairs_mut();

            query_pairs.append_pair("symbol", symbol);
            query_pairs.append_pair("timestamp", &timestamp().as_millis().to_string());
        }

        self.build_sign_request_get(url)?
            .with_api_key(self.secret.api_key()?)
            .send()
            .await
    }

    pub async fn trade_fee(
        &self,
        symbol: &Symbol,
        recv_window: Option<u8>,
    ) -> ClientResult<Vec<TradeFee>> {
        let mut url = self.base_url()?;
        url.set_path("/sapi/v1/asset/tradeFee");

        {
            let mut query_pairs = url.query_pairs_mut();

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
pub struct TradeFee {
    pub symbol: Symbol,

    #[serde(rename = "makerCommission")]
    pub maker_commission: Commission,

    #[serde(rename = "takerCommission")]
    pub taker_commission: Commission,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAsset {
    pub asset: Asset,
    pub free: Decimal,
    pub locked: Decimal,
    pub freeze: Decimal,
    pub withdrawing: Decimal,
    pub ipoable: Decimal,

    #[serde(rename = "btcValuation")]
    pub btc_valuation: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiRestrictions {
    #[serde(rename = "ipRestrict")]
    pub ip_restrict: bool,

    #[serde(rename = "createTime")]
    pub create_time: u128,

    #[serde(rename = "enableInternalTransfer")]
    pub enable_internal_transfer: bool,

    #[serde(rename = "enableFutures")]
    pub enable_futures: bool,

    #[serde(rename = "enablePortfolioMarginTrading")]
    pub enable_portfolio_margin_trading: bool,

    #[serde(rename = "enableVanillaOptions")]
    pub enable_vanilla_options: bool,

    #[serde(rename = "permitsUniversalTransfer")]
    pub permits_universal_transfer: bool,

    #[serde(rename = "enableReading")]
    pub enable_reading: bool,

    #[serde(rename = "enableSpotAndMarginTrading")]
    pub enable_spot_and_margin_trading: bool,

    #[serde(rename = "enableWithdrawals")]
    pub enable_withdrawals: bool,

    #[serde(rename = "enableMargin")]
    pub enable_margin: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpotAccount {
    #[serde(rename = "makerCommission")]
    pub maker_commission: u32,

    #[serde(rename = "takerCommission")]
    pub taker_commission: u32,

    #[serde(rename = "buyerCommission")]
    pub buyer_commission: u32,

    #[serde(rename = "sellerCommission")]
    pub seller_commission: u32,

    #[serde(rename = "commissionRates")]
    pub commission_rates: CommissionRates,

    #[serde(rename = "canTrade")]
    pub can_trade: bool,

    #[serde(rename = "canWithdraw")]
    pub can_withdraw: bool,

    #[serde(rename = "canDeposit")]
    pub can_deposit: bool,

    pub brokered: bool,

    #[serde(rename = "requireSelfTradePrevention")]
    pub require_self_trade_prevention: bool,

    #[serde(rename = "preventSor")]
    pub prevent_sor: bool,

    #[serde(rename = "updateTime")]
    pub update_time: u64,

    #[serde(rename = "accountType")]
    pub account_type: String,

    pub balances: Vec<Balance>,

    pub permissions: Vec<String>,

    pub uid: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommissionRates {
    pub maker: Commission,
    pub taker: Commission,
    pub buyer: Commission,
    pub seller: Commission,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Balance {
    pub asset: Asset,
    pub free: Quantity,

    #[serde(rename = "locked")]
    pub locked_amount: Quantity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpotCommission {
    pub symbol: Symbol,

    #[serde(rename = "standardCommission")]
    pub standard_commission: CommissionDetails,

    #[serde(rename = "taxCommission")]
    pub tax_commission: CommissionDetails,

    pub discount: DiscountDetails,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommissionDetails {
    pub maker: Commission,
    pub taker: Commission,
    pub buyer: Commission,
    pub seller: Commission,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscountDetails {
    #[serde(rename = "enabledForAccount")]
    pub enabled_for_account: bool,

    #[serde(rename = "enabledForSymbol")]
    pub enabled_for_symbol: bool,

    #[serde(rename = "discountAsset")]
    pub discount_asset: Asset,

    pub discount: Commission,
}

#[cfg(test)]
mod tests {
    use crate::http::client::tests::client_with_key_secret;

    #[tokio::test]
    async fn test_user_asset() {
        let client = client_with_key_secret();
        client.user_asset(None, None, None).await.unwrap();
    }

    #[tokio::test]
    async fn test_api_restrictions() {
        let client = client_with_key_secret();
        client.api_restrictions(None).await.unwrap();
    }

    #[tokio::test]
    async fn test_spot_account() {
        let client = client_with_key_secret();
        client.spot_account(None, None).await.unwrap();
    }

    #[tokio::test]
    async fn test_spot_commission() {
        let client = client_with_key_secret();
        client.spot_commission(&"BTCUSDT".into()).await.unwrap();
    }

    #[tokio::test]
    async fn test_trade_fee() {
        let client = client_with_key_secret();
        client.trade_fee(&"BTCUSDT".into(), None).await.unwrap();
    }
}
