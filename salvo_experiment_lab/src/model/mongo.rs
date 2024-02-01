use bson::{doc};
use mongodb::{Client, Collection, IndexModel};
use mongodb::options::IndexOptions;
use once_cell::sync::OnceCell;
use crate::model::user_model::User;

pub const DB_NAME: &str = "labs";
pub const COLL_USERS: &str = "users";
pub const COLL_LABS: &str = "labs";
pub const COLL_EXPERIMENTS: &str = "experiments";

static MONGODB_CLIENT: OnceCell<Client> = OnceCell::new();
#[inline]
pub fn get_mongodb_client() -> &'static Client {
    MONGODB_CLIENT.get().unwrap()
}

#[inline]
pub fn get_coll<T>(coll_name: &str) -> Collection<T> {
    let client = get_mongodb_client();
    
    client.database(DB_NAME).collection::<T>(coll_name)
}

pub async fn mongodb_init() {
    let mongodb_uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://root:root@localhost:27017".into());
    let client = Client::with_uri_str(mongodb_uri).await.expect("failed to connect");
    create_username_index(&client).await;

    MONGODB_CLIENT.set(client).unwrap();

}
async fn create_username_index(client: &Client) {
    let options = IndexOptions::builder().unique(true).build();
    let model = IndexModel::builder()
        .keys(doc! { "username": 1 })
        .options(options)
        .build();
    client
        .database(DB_NAME)
        .collection::<User>(COLL_USERS)
        .create_index(model, None)
        .await
        .expect("creating an index should succeed");
}
