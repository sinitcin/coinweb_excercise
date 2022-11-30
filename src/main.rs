use solution::Solution;
use statement::ServerName;

mod solution;
mod statement;

#[tokio::main]
async fn main() {
    let repositories = vec![
        ServerName("One".to_string()),
        ServerName("Two".to_string()),
        ServerName("Three".to_string()),
        ServerName("Four".to_string()),
        ServerName("Five".to_string()),
        ServerName("Six".to_string()),
        ServerName("Seven".to_string()),
        ServerName("Eight".to_string()),
        ServerName("Nine".to_string()),
        ServerName("Ten".to_string()),
    ];

    match solution::Solution0::solve(repositories).await {
        None => println!("All downloads failed!"),
        Some(binary) => println!("{binary} downloaded"),
    }
}
