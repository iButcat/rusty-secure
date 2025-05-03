use mongodb::{bson::doc, Client, options::ClientOptions};

pub async fn init_mongo_client(url: String, database_name: String) -> mongodb::error::Result<Client> {
    let mut client_options = ClientOptions::parse(url).await?;

    client_options.connect_timeout = Some(std::time::Duration::from_secs(5));

    let client = Client::with_options(client_options)?;

    client.database(database_name.as_str()).run_command(doc! {"ping": 1}).await?;

    println!("MongoDB client initialised!");

    Ok(client)
}