#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Some((v, l)) = upgrenade::check_upgrenade(Some("0.0.1".parse()?), "http://localhost:8080").await? {
        println!("There's a new version {} available under {}", v, l);
    } else {
        println!("You are running the latest version.");
    }
    Ok(())
}
