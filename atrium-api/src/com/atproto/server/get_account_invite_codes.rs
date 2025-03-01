// This file is generated by atrium-codegen. DO NOT EDIT.
//!Definitions for the `com.atproto.server.getAccountInviteCodes` namespace.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Parameters {
    ///Controls whether any new 'earned' but not 'created' invites should be created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_available: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_used: Option<bool>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Output {
    pub codes: Vec<crate::com::atproto::server::defs::InviteCode>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "error", content = "message")]
pub enum Error {
    DuplicateCreate(Option<String>),
}
