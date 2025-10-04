use serde::Serialize;
use z157::Tree;

#[derive(Serialize)]
struct Data {
    id: u64,
    name: String,
    email: String,
    balance: f64,
}

/// Example handler function that filters data based on field specification
fn handle(data: Data, fields: &str) -> Result<String, Box<dyn std::error::Error + '_>> {
    let tree = Tree::parse(fields)?;
    let filtered = tree.filter(data);
    let json = serde_json::to_string(&filtered)?;
    Ok(json)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = Data {
        id: 123,
        name: "John Doe".to_string(),
        email: "john@example.com".to_string(),
        balance: 1234.56,
    };

    // Simulating different API requests with different field filters
    println!("Full data:");
    println!("{}\n", serde_json::to_string_pretty(&data)?);

    println!("Request: GET /users/123?fields=(name,email)");
    let result = handle(data, "(name,email)")?;
    println!("{}\n", result);

    let data = Data {
        id: 123,
        name: "John Doe".to_string(),
        email: "john@example.com".to_string(),
        balance: 1234.56,
    };

    println!("Request: GET /users/123?fields=!(email)");
    let result = handle(data, "!(email)")?;
    println!("{}\n", result);

    Ok(())
}
