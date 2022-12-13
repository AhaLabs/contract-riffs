use microjson::{JSONValue, JSONValueType};
use near_sdk::{
    env,
    serde::{Deserialize, Serialize},
    AccountId,
};
pub fn input_as_str() -> String {
    unsafe { String::from_utf8_unchecked(env::input().unwrap()) }
}

/// Can decode `{"account_id": account_id}`, `"account_id"`, or `account_id`
pub fn account_id() -> AccountId {
    let input = input_as_str();
    input.parse().unwrap_or_else(|_| {
        parse_json_or_string(input.as_str(), "account_id")
            .unwrap()
            .parse()
            .unwrap()
    })
}

pub fn parse_json_or_string(input: &str, key: &str) -> Result<String, microjson::JSONParsingError> {
    let object = JSONValue::parse(input)?;
    match object.value_type {
        JSONValueType::String => object.read_string().map(Into::into),
        JSONValueType::Object => object
            .get_key_value(key)
            .and_then(|val| val.read_string().map(ToString::to_string)),
        _ => env::panic_str("cannot parse account_id"),
    }
}

#[allow(dead_code)]
mod p {
    use witgen::witgen;

    #[witgen]
    /// Upload a file to the contract in base64 encoded data-url
    /// @format data-url
    pub type DataUrl = String;
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct DataUrl(String);

impl DataUrl {
    pub fn to_vec(self) -> Vec<u8> {
        self.into()
    }
}

impl From<DataUrl> for Vec<u8> {
    fn from(value: DataUrl) -> Self {
        data_url::DataUrl::process(&value.0)
            .unwrap()
            .decode_to_vec()
            .map(|(bytes, _)| bytes)
            .unwrap()
    }
}
