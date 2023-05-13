// This file is generated by atrium-codegen. DO NOT EDIT.
#![doc = "Definitions for the `app.bsky.actor.getPreferences` namespace."]
#[doc = "`app.bsky.actor.getPreferences`"]
#[doc = "Get private preferences attached to the account."]
#[async_trait::async_trait]
pub trait GetPreferences: crate::xrpc::XrpcClient {
    async fn get_preferences(
        &self,
        params: Parameters,
    ) -> Result<Output, Box<dyn std::error::Error>> {
        let body = crate::xrpc::XrpcClient::send::<Error>(
            self,
            http::Method::GET,
            "app.bsky.actor.getPreferences",
            Some(serde_urlencoded::to_string(&params)?),
            None,
            None,
        )
        .await?;
        serde_json::from_slice(&body).map_err(|e| e.into())
    }
}
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Parameters {}
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Output {
    pub preferences: crate::app::bsky::actor::defs::Preferences,
}
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "error", content = "message")]
pub enum Error {}
