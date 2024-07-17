use serde_json::Value;
use log::error;

pub fn process_openai_error_response(error_response: String) {
    // parse error response
    let parsed: Value = match serde_json::from_str(error_response.as_str()) {
        Ok(value) => value,
        Err(e) => {
            error!("Failed to parse error response: {}", e);
            println!("Received an error, but failed to parse the response.");
            return;
        }
    };

    // extract and print error message
    if let Some(error) = parsed.get("error") {
        if let Some(message) = error.get("message").and_then(Value::as_str) {
            println!("Error: {}", message);
        } else {
            println!("Error occurred, but no message provided.");
        }
    } else {
        println!("Received an error, but the response format is unexpected.");
    }
}
