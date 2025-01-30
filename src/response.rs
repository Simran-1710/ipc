use serde::{Serialize, Deserialize};

/// Response enum is used for returning the result of a subscription
#[derive(Debug, Serialize, Deserialize)]
pub enum Response<T = String> {
    Success(T),
    DecodeFailure,
    Timeout,
}

impl Response<String> {
    pub fn extract<T>(&self) -> Response<T>
    where
        T: serde::de::DeserializeOwned,
    {
        match self {
            Response::Success(str) => serde_json::from_str(&str)
                .map_or_else(|_| Response::DecodeFailure, |json| Response::Success(json)),
            Response::DecodeFailure => Response::DecodeFailure,
            Response::Timeout => Response::Timeout,
        }
    }
}
