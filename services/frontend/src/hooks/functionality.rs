// use anyhow::Result;
// use serde_json::*;
// use serde_json::value::Value as Json;

// use super::ROOT_DOMAIN;

// pub async fn dice_prediction() -> Result<()> {
//     let response = reqwest::get(format!("{}/api/login", ROOT_DOMAIN())).await;

//     match response {
//         Ok(_) => Ok(()),
//         Ok(res) => {
//             let json_value = res.json().await?;

//             Ok((
//                 "tes".into(),
//                 json_value["user"]["username"].as_str().map(to_string),
//             ))
//         }
//         Err(e) => Err(e.into()),
//     }
// }