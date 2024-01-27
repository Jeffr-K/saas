use mongodb::{Client, Collection};
use mongodb::bson::{doc, Document};
use mongodb::options::ClientOptions;

#[tokio::main]
async fn main() {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
    let client = Client::with_options(client_options).unwrap();

    create_collection(&client, "meshed-iam", "member").await;
    insert_document(&client, "meshed-iam", "member").await;
    retrieve_document(&client, "meshed-iam", "member").await;
}

async fn create_collection(client: &Client, database_name: &str, collection_name: &str) {
    let db = client.database(database_name);
    db.create_collection(collection_name, None).await.expect("TODO: panic message");
}

async fn insert_document(client: &Client, database_name: &str, collection_name: &str) {
    let database = client.database(database_name);
    let collection = database.collection(collection_name);

    let document = doc! { "name": "jeffrey", "email": "wjdrlrkdl3@gmail.com" };

    collection.insert_one(document, None).await.unwrap();
}

async fn retrieve_document(client: &Client, database_name: &str, collection_name: &str) {
    let database = client.database(database_name);
    let collection: Collection<Document> = database.collection(collection_name);

    let filter = doc! { "name": "jeffrey" };

    let result = collection.find_one(Some(filter), None).await.unwrap();

    match result {
        Some(doc) => println!("{}", doc),
        None => println!("No document found"),
    }
}
