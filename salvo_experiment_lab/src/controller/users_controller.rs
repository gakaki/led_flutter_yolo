use futures::stream::TryStreamExt;
use mongodb::{bson::doc, bson::Document, Collection};

use salvo::prelude::*;

use crate::model::mongo::{COLL_USERS, DB_NAME, get_mongodb_client};
use crate::model::resp::AppResult;
use crate::model::user_model::{User};

#[handler]
pub async fn add_user(req: &mut Request, res: &mut Response) {
    let client = get_mongodb_client();
    let coll_users = client.database(DB_NAME).collection::<Document>(COLL_USERS);
    let new_user = req.parse_json::<User>().await.unwrap();

    let user = doc! {
        "first_name": new_user.first_name,
        "last_name": new_user.last_name,
        "username": new_user.username,
        "email": new_user.email,
    };

    let result = coll_users.insert_one(user, None).await;
    match result {
        Ok(id) => res.render(Json(format!("user added with ID {:?}", id.inserted_id))),
        Err(e) => res.render(Json(format!("error {e:?}"))),
    }
}

#[handler]
pub  async fn get_users(res: &mut Response) -> AppResult<()> {
    let client = get_mongodb_client();
    let coll_users = client.database(DB_NAME).collection::<User>(COLL_USERS);
    let mut cursor = coll_users.find(None, None).await?;
    let mut vec_users: Vec<User> = Vec::new();
    while let Some(user) = cursor.try_next().await? {
        vec_users.push(user);
    }
    res.render(Json(vec_users));
    Ok(())
}

#[handler]
pub async fn get_user(req: &mut Request, res: &mut Response) {
    let client = get_mongodb_client();
    let coll_users: Collection<User> = client.database(DB_NAME).collection(COLL_USERS);

    let username = req.param::<String>("username").unwrap();
    match coll_users.find_one(doc! { "username": &username }, None).await {
        Ok(Some(user)) => res.render(Json(user)),
        Ok(None) => res.render(Json(format!("No user found with username {username}"))),
        Err(e) => res.render(Json(format!("error {e:?}"))),
    }
}
