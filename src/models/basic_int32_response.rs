/*
 * Imgur API
 *
 * Imgur's API exposes the entire Imgur infrastructure via a standardized programmatic interface. Using Imgur's API, you can do just about anything you can do on imgur.com, while using your programming language of choice.
 *
 * The version of the OpenAPI document: 0.2.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BasicInt32Response {
    #[serde(rename = "data")]
    pub data: i32,
    #[serde(rename = "status")]
    pub status: i32,
    #[serde(rename = "success")]
    pub success: bool,
}

impl BasicInt32Response {
    pub fn new(data: i32, status: i32, success: bool) -> BasicInt32Response {
        BasicInt32Response {
            data,
            status,
            success,
        }
    }
}


