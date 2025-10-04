use serde::Serialize;
use z157::Tree;

#[derive(Serialize)]
struct Address {
    street: String,
    city: String,
    zip: String,
    country: String,
}

#[derive(Serialize)]
struct User {
    name: String,
    age: u32,
    email: String,
    phone: String,
    address: Address,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let user = User {
        name: "Alice Johnson".to_string(),
        age: 30,
        email: "alice@example.com".to_string(),
        phone: "+1-555-0123".to_string(),
        address: Address {
            street: "123 Main St".to_string(),
            city: "Springfield".to_string(),
            zip: "12345".to_string(),
            country: "USA".to_string(),
        },
    };

    println!("=== Original structure ===");
    let json = serde_json::to_string_pretty(&user)?;
    println!("{}", json);

    // Example 1: Include only specific fields
    println!("=== Example 1: Include only name and city ===");
    let fields = "(name,address(city))";
    println!("fields={fields}");
    let tree = Tree::parse(fields)?;
    let filtered = tree.filter(&user);
    let json = serde_json::to_string_pretty(&filtered)?;
    println!("{}", json);

    // Example 2: Exclude sensitive fields
    println!("\n=== Example 2: Exclude email and phone ===");
    let fields = "!(email,phone)";
    println!("fields={fields}");
    let tree = Tree::parse(fields)?;
    let filtered = tree.filter(&user);
    let json = serde_json::to_string_pretty(&filtered)?;
    println!("{}", json);

    // Example 3: Include nested fields
    println!("\n=== Example 3: Include only address fields ===");
    let fields = "(address(street,city,zip))";
    println!("fields={fields}");
    let tree = Tree::parse(fields)?;
    let filtered = tree.filter(&user);
    let json = serde_json::to_string_pretty(&filtered)?;
    println!("{}", json);

    // Example 4: Include nested fields
    println!("\n=== Example 4: Include only address fields (via parent) ===");
    let fields = "(address)";
    println!("fields={fields}");
    let tree = Tree::parse(fields)?;
    let filtered = tree.filter(&user);
    let json = serde_json::to_string_pretty(&filtered)?;
    println!("{}", json);

    Ok(())
}
