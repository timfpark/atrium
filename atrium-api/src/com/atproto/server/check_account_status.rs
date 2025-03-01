// This file is generated by atrium-codegen. DO NOT EDIT.
//!Definitions for the `com.atproto.server.checkAccountStatus` namespace.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Output {
    pub activated: bool,
    pub expected_blobs: i64,
    pub imported_blobs: i64,
    pub indexed_records: i64,
    pub private_state_values: i64,
    pub repo_blocks: i64,
    pub repo_commit: crate::types::string::Cid,
    pub repo_rev: String,
    pub valid_did: bool,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "error", content = "message")]
pub enum Error {}
