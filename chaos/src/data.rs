use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

pub async fn process_data(Json(request): Json<DataRequest>) -> impl IntoResponse {
    // Calculate sums and return response

    let response = DataResponse {
        let mut total_string_len = 0;
        let mut total_int_sum = 0;

        for item in request.data {
            match item {
                DataValue::Str(s) => total_string_len += s.len(),
                DataValue::Int(i) => total_int_sum += i,
            }
        }

        let response = DataResponse {
            string_len: total_string_len,
            int_sum: total_int_sum,
        };

        (StatusCode::OK, Json(response))
    };

    (StatusCode::OK, Json(response))
}

#[derive(Deserialize)]
#[serde(untagged)]
enum DataValue {
    Str(String),
    Int(i32),
}

#[derive(Deserialize)]
pub struct DataRequest {
    // Add any fields here
}

#[derive(Serialize)]
pub struct DataResponse {
    // Add any fields here
}
