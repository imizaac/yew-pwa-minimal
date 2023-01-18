use gloo_net::http::Request;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct counter_value {
    value: i64,
}

pub async fn fetch_counter_value() {
    let fetched_value: counter_value = Request::get("/counter-value")
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
}

pub async fn add_one() {
    let res: counter_value = Request::post("/add-one").send().await.json().await.unwrap();
    return res;
}
