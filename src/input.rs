use near_sdk::serde::{Deserialize, Serialize};

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
