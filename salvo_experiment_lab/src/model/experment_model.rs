

use mongodb::{
    bson::doc, bson::oid::ObjectId,
};

use serde::{Deserialize, Serialize, Serializer};

// https://github.com/thedodd/wither/issues/62
pub fn serialize_object_id<S>(object_id: &Option<ObjectId>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
{
    match object_id {
        Some(ref object_id) => serializer.serialize_some(object_id.to_string().as_str()),
        None => serializer.serialize_none()
    }
}

// serialize_with = "bson::serde_helpers::serialize_object_id_as_hex_string",
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExperimentItem {
    #[serde(
    serialize_with = "serialize_object_id",
    rename = "_id",
    skip_serializing_if = "Option::is_none"
    )]
    pub id: Option<ObjectId>,
    pub lab_id: Option<String>,
    pub name: Option<String>,
    pub comment: Option<String>,
    pub created_at: Option<String>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub created_at_bson: Option<BsonDateTime>,
    // #[serde(with = "chrono_datetime_as_bson_datetime")]
    pub started_at: Option<String>,
    pub ended_at: Option<String>,
}

#[cfg(test)]
mod tests {

    use crate::model::experment_model::{ ExperimentItem};
    use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
    use mongodb::bson::{doc, DateTime as BsonDateTime, Document};
    use std::str::FromStr;
    #[test]
    fn test_deserialize() {
        let datetime_str = "2023-12-11 23:50:25";
        let naive_datetime =
            chrono::NaiveDateTime::parse_from_str(datetime_str, "%Y-%m-%d %H:%M:%S").unwrap();
        let datetime: DateTime<Utc> = TimeZone::from_utc_datetime(&Utc, &Default::default());
    }

    #[test]
    fn test_router_detect_path_encoded() {
        let datetime_str = "2023-12-11 23:50:25";
        let parse_from_str = NaiveDateTime::parse_from_str;
        let naive_datetime = parse_from_str(datetime_str, "%Y-%m-%d %H:%M:%S").expect("Failed to parse naive datetime");
        let chrono_datetime: DateTime<Utc> =
            DateTime::from_naive_utc_and_offset(naive_datetime, Utc);

        let bson_datetime: BsonDateTime = BsonDateTime::from_chrono(chrono_datetime);
        // You can now use bson_datetime in your MongoDB queries or documents
        let doc = doc! {"datetime":bson_datetime};
        println!("{:?}", doc);

        let info = ExperimentItem {
            id: None,
            lab_id: "lab_id".to_string(),
            name: "张三".to_string(),
            comment: "我的设备".to_string(),
            created_at: crate::utils::time_utils::current_time(),
            started_at: "".to_string(),
            // created_at_bson:  Option::from(BsonDateTime::from_chrono(chrono_datetime)),
            ended_at: "".to_string(),
        };
        // 序列化实例为JSON字符串
        let serialized = serde_json::to_string(&info).unwrap();
        println!("Serialized: {}", serialized);
        // 反序列化JSON字符串为DeviceInfo实例
        let deserialized: ExperimentItem = serde_json::from_str(&serialized).unwrap();
        println!("Deserialized: {:?}", deserialized);
        assert_eq!(deserialized.name, "张三");
    }
}
