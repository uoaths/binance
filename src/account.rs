use serde::{Deserialize, Serialize};

use crate::{
    http::client::{Client, ClientResult},
    time::timestamp,
    types::{Decimal, Symbol},
};

impl Client {
    pub async fn get_user_asset(
        &self,
        asset: Option<&String>,
        need_btc_valuation: Option<bool>,
        recv_window: Option<u8>,
    ) -> ClientResult<Vec<GetUserAsset>> {
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

    pub async fn get_api_restrictions(
        &self,
        recv_window: Option<u8>,
    ) -> ClientResult<ApiRestrictions> {
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
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserAsset {
    pub asset: Symbol,
    pub free: Decimal,
    pub locked: Decimal,
    pub freeze: Decimal,
    pub withdrawing: Decimal,
    pub ipoable: Decimal,

    #[serde(rename(serialize = "btcValuation", deserialize = "btcValuation"))]
    pub btc_valuation: Decimal,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[cfg(test)]
mod tests {
    use crate::http::client::tests::client_with_key_secret;

    #[tokio::test]
    async fn test_get_user_asset() {
        let client = client_with_key_secret();
        client.get_user_asset(None, None, None).await.unwrap();
    }

    #[tokio::test]
    async fn test_get_api_restrictions() {
        let client = client_with_key_secret();
        client.get_api_restrictions(None).await.unwrap();
    }
}
