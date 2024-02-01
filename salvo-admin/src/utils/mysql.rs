use crate::GLOBAL_DB;

// 连接数据库
pub async fn init_db() {
    GLOBAL_DB.link(
        rbdc_mysql::driver::MysqlDriver {},
        "mysql://root:root@localhost/ry-vue",
    ).await.expect("数据库连接失败");
}