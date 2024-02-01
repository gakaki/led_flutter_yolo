


use futures::stream::TryStreamExt;
use mongodb::{bson::doc};

use salvo::prelude::*;


use crate::model::experment_model::ExperimentItem;
use crate::model::mongo::{COLL_EXPERIMENTS, get_coll, get_mongodb_client};
use crate::model::resp::AppResult;
use crate::utils::req::get_request_object_bson_id;
use crate::utils::res::{ my_json_msg};

#[handler]
pub async fn add_experiment(req: &mut Request, res: &mut Response) {
    let _client = get_mongodb_client();
    let coll = get_coll::<ExperimentItem>(COLL_EXPERIMENTS);
    let new_experiment = req.parse_json::<ExperimentItem>().await.unwrap();
    // let row =
    // let row = doc! {
    //     "first_name": new_experiment,
    //     "last_name": new_experiment.last_name,
    //     "username": new_experiment.username,
    //     "email": new_experiment.email,
    // };
    let result = coll.insert_one(new_experiment, None).await;
    match result {
        Ok(id) => res.render(Json(format!("experiment added with ID {:?}", id.inserted_id))),
        Err(e) => res.render(Json(format!("error {e:?}"))),
    }
}

#[handler]
pub  async fn get_experiments(res: &mut Response) -> AppResult<()> {
    let coll = get_coll::<ExperimentItem>(COLL_EXPERIMENTS);
    let mut cursor = coll.find(None, None).await?;
    let mut rows: Vec<ExperimentItem> = Vec::new();
    while let Some(row) = cursor.try_next().await? {
        rows.push(row);
    }
    res.render(Json(rows));
    // my_json_data::<Vec<ExperimentItem>>(res,Some(rows));
    Ok(())
}

#[handler]
pub async fn show_experiment(req: &mut Request, res: &mut Response) {
    let coll = get_coll::<ExperimentItem>(COLL_EXPERIMENTS);
    let bson_id = get_request_object_bson_id(req);

    match coll.find_one(doc! { "_id": &bson_id }, None).await {
        Ok(Some(row)) => res.render(Json(row)),
        Ok(None) => res.render(Json(format!("No row found with _id {bson_id}"))),
        Err(e) => res.render(Json(format!("error {e:?}"))),
    }
}
#[handler]
pub async fn update_experiment_start_and_end(req: &mut Request, res: &mut Response) {
    let coll = get_coll::<ExperimentItem>(COLL_EXPERIMENTS);
    let bson_id = get_request_object_bson_id(req);
    let row = req.parse_json::<ExperimentItem>().await.unwrap();

    let stated_at = row.started_at.unwrap_or("".to_string());
    let ended_at = row.ended_at.unwrap_or("".to_string());

    let filter = doc! { "_id": &bson_id };
    let mut update = doc! {};
    if stated_at.is_empty().eq(&false) {
        update = doc! { "$set": doc!
            {
                "started_at": stated_at.clone(),
            }
        };
    }
    if ended_at.is_empty().eq(&false){
        update = doc! { "$set": doc!
            {
                "ended_at": ended_at.clone(),
            }
        };
    }

    let result = coll.update_one(filter, update.clone(), None).await;
    match result {
        Ok(_id) => {
            my_json_msg(res, format!("experiment update with row {:?}", update.clone()))
        },
        Err(e) => res.render(Json(format!("error {e:?}"))),
    }
}



