

use salvo::prelude::Json;
use salvo::Response;
use serde::{Deserialize, Serialize};

pub type AppResult<T> = Result<T, anyhow::Error>;

#[cfg(feature = "anyhow")]
#[async_trait]
impl Writer for ::anyhow::Error {
    async fn write(mut self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        res.render(StatusError::internal_server_error());
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct BaseResp<T> {
    code: Option<u8>,
    data: Option<T>,
    msg: Option<String>,
}


pub fn my_json_msg(res: &mut Response, msg: String) {
    let base_resp: BaseResp<&str>  = BaseResp {
        code: Some(200),
        data: None,
        msg: Some(msg.clone().to_string()),
    };
    res.render(Json(base_resp))
}
// pub fn my_json_data<T>(res: &mut Response, data: Option<T>) {
//     let base_resp: BaseResp<T>  = BaseResp {
//         code: Some(200),
//         data: data,
//         msg: Some("success".to_string())
//     };
//     res.render(Json(base_resp))
// }
