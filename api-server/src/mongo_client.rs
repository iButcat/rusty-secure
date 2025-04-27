use mongodb::{bson::doc, Client, options::ClientOptions};


pub async fn init_mongo_client() -> mongodb::error::Result<Client> {
    // TODO: Replace by env var instead
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;

    client_options.connect_timeout = Some(std::time::Duration::from_secs(5));

    let client = Client::with_options(client_options)?;

    // TODO: env var instead
    client.database("rusty-secure").run_command(doc! {"ping": 1}).await?;

    println!("MongoDB client initialised!");

    Ok(client)
}