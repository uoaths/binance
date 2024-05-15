use ring::hmac;
use serde::Deserialize;
use url::Url;

use super::client::{Client, ClientResult};
use super::error::{BinanceError, ClientError};

impl Client {
    pub fn base_url(&self) -> ClientResult<Url> {
        Ok(Url::parse(&self.base_url)?)
    }

    pub(crate) fn build_request_get(&self, url: Url) -> RequestBuilder {
        let inner = self.inner.get(url);

        RequestBuilder { inner }
    }

    pub(crate) fn build_request_post(&self, url: Url) -> RequestBuilder {
        let inner = self.inner.post(url);

        RequestBuilder { inner }
    }

    pub(crate) fn build_sign_request_get(&self, url: Url) -> ClientResult<RequestBuilder> {
        let url = self.sign_url_query(url)?;
        Ok(self.build_request_get(url))
    }

    pub(crate) fn build_sign_request_post(&self, url: Url) -> ClientResult<RequestBuilder> {
        let url = self.sign_url_query(url)?;
        Ok(self.build_request_post(url))
    }

    fn sign_url_query(&self, mut url: Url) -> ClientResult<Url> {
        let query = match url.query() {
            Some(v) => v.to_string(),
            None => return Err(ClientError::Request("Empty Query".to_string())),
        };

        let secret = self.secret.secret_key()?;
        let key = hmac::Key::new(hmac::HMAC_SHA256, secret.as_bytes());
        let tag = hmac::sign(&key, query.as_bytes());

        let value: String = tag
            .as_ref()
            .iter()
            .map(|byte| format!("{:02x}", byte))
            .collect();

        {
            url.query_pairs_mut().append_pair("signature", &value);
        }

        Ok(url)
    }
}

#[rustfmt::skip]
#[derive(Debug, Default)]
pub(crate) struct Secret {
    api_key:     Option<String>,
    secret_key:  Option<String>
}

impl Secret {
    pub(crate) fn api_key(&self) -> ClientResult<&String> {
        match &self.api_key {
            Some(value) => Ok(value),
            None => Err(ClientError::Authorization("API KEY".into())),
        }
    }

    pub(crate) fn secret_key(&self) -> ClientResult<&String> {
        match &self.secret_key {
            Some(value) => Ok(value),
            None => Err(ClientError::Authorization("SECRET KEY".into())),
        }
    }

    pub fn update_api_key(&mut self, value: String) {
        self.api_key = Some(value)
    }

    pub fn update_secret_key(&mut self, value: String) {
        self.secret_key = Some(value)
    }
}

#[derive(Debug)]
pub(crate) struct RequestBuilder {
    inner: reqwest::RequestBuilder,
}

impl From<reqwest::RequestBuilder> for RequestBuilder {
    fn from(value: reqwest::RequestBuilder) -> Self {
        Self { inner: value }
    }
}

impl RequestBuilder {
    pub(crate) async fn send<T>(self) -> ClientResult<T>
    where
        for<'a> T: Deserialize<'a>,
    {
        let response = self.inner.send().await?;

        if response.status().is_success() {
            return Ok(response.json::<T>().await?);
        }

        match response.json::<BinanceError>().await {
            Ok(v) => Err(ClientError::Binance(v)),
            Err(e) => Err(ClientError::Request(e.to_string())),
        }
    }

    pub(crate) fn with_api_key(self, value: &String) -> Self {
        self.inner.header("X-MBX-APIKEY", value).into()
    }

    // pub(crate) fn with_json_body<T>(self, value: &T) -> Self
    // where
    //     T: Serialize,
    // {
    //     self.inner.json(value).into()
    // }
}
