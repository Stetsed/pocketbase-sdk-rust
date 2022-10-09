use std::collections::HashMap;
use std::error::Error;

use serde::{Serialize, Deserialize};

use crate::client::SyncClient;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SuccessResponse {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FailureResponse {
    code: String,
    message: String,
    data: HashMap<String, String>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", untagged)]
enum DeleteResponse {
    SuccessResponse(SuccessResponse),
    FailureResponse(FailureResponse)
}

pub fn record(collection: &str, id: &str, client: &SyncClient) -> Result<(), Box<dyn Error>> {
    let url = format!("/api/collections/{}/records/{}", collection, id);
    match client.delete(url, None) {
        Ok(resp) => {
            match resp.json::<DeleteResponse>() {
                Ok(_) => Ok(()),
                Err(e) => Err(Box::new(e) as Box<dyn Error>)
            }
        },
        Err(err) => Err(err)
    }
}
