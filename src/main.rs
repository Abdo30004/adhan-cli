mod api;
mod cli;

#[tokio::main]
async fn main() -> Result<(), api::AdhanError> {
    let result: Result<(), api::AdhanError> = cli::init().await;

    if let Err(e) = result {
        println!("{}", e);
    }

    Ok(())
}
