//use reqwest::Error;
use tokio;
use serde_json::Value;
use std::time::Duration;
use tokio::time;

#[tokio::main]
async fn main() {

    let api_key = "ENTER API KEY";
    let transaction_hash = "0xb5b9b68692fe1421230ef434609ab0d17a7e3885dfd3e4021bff0366469ad988";
    let interval = Duration::from_secs(10);

    let url = format!("https://api.etherscan.io/api?module=proxy&action=eth_getTransactionByHash&txhash={}&apikey={}", transaction_hash, api_key);

    let response = reqwest::get(&url).await?;

    // Ensure the request was successful (status code 200)
    if response.status().is_success() {
        let body = response.text().await?;


        let json: Value = serde_json::from_str(&body).unwrap();

        println!("{}", serde_json::to_string_pretty(&json).unwrap());
    } else {
        println!("Request failed with status code: {}", response.status());
    }
    //
    // get ether balance for address
    let url2 = format!("https://api.etherscan.io/api?module=account&action=balance&address=0x0AFfB0a96FBefAa97dCe488DfD97512346cf3Ab8&tag=latest&apikey={}", api_key);

    let response2 = reqwest::get(&url2).await.unwrap();

    if response2.status().is_success() {
    let body2 = response2.text().await.unwrap();

    let json2: Value = serde_json::from_str(&body2).unwrap();

    println!("{}", serde_json::to_string_pretty(&json2).unwrap());
    } else {
    println!("Request failed with status code: {}", response2.status());
    }

    loop {
        let url = format!("https://api.etherscan.io/api?module=account&action=txlist&address=0xc5eb05dc13ce3d467a40750ae1baf2f834dd2fe2e08f92b3a722bda769a3d5bd&startblock=0&endblock=99999999&page=1&offset=10&sort=asc&apikey={}", api_key);

        let response = reqwest::get(&url).await.unwrap();
        
        if response.status().is_success() {
            let body = response.text().await.unwrap();

            let json: Value = serde_json::from_str(&body).unwrap();

            println!("{}", serde_json::to_string_pretty(&json).unwrap());
        } else {
            println!("Request failed because:{}", response.status());
        } 

        time::sleep(interval).await;
    }

}
