use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk::{caller, query};

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[derive(CandidType, Deserialize)]
struct GreetResponse {
    message: String,
}

async fn call_greet_actor(name: String) -> Result<String, String> {
    let response_bytes = match query(caller(), "greet", (name,)).await {
        Ok(bytes) => bytes,
        Err(err) => return Err(format!("Error: {}", err)),
    };
    let response: GreetResponse = match GreetResponse::try_from(response_bytes.as_slice()) {
        Ok(decoded) => decoded,
        Err(err) => return Err(format!("Error decoding response: {}", err)),
    };
    Ok(response.message)
}

#[tokio::main]
async fn main() {
    // Example usage
    let name = "Alice".to_string();
    match call_greet_actor(name).await {
        Ok(message) => println!("{}", message),
        Err(err) => eprintln!("{}", err),
    }
}
