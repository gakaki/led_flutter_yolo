use std::str::FromStr;
use bson::oid::ObjectId;
use salvo::Request;

pub fn get_request_object_bson_id(req: &mut Request) -> ObjectId {
    let id = req.param::<String>("id").unwrap();
    
    bson::oid::ObjectId::from_str(&id).unwrap()
}
