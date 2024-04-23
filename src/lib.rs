mod response_data;
mod security_task;
mod security_temp;

use response_data::{dao as resDao, model::ResponseData};
use security_task::{dao as taskDao, model::SecurityTask};
use security_temp::{dao as tempDao, model::SecurityTemp};
use tracing::{event, Level};

pub async fn get_security_all_code(pool: &sqlx::PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let mut transaction = pool.begin().await?;

    let d = SecurityTemp {
        row_id: None,
        version_code: None,
        international_code: None,
        security_code: None,
        security_name: None,
        market_type: None,
        security_type: None,
        industry_type: None,
        issue_date: None,
        cfi_code: None,
        remark: None,
        is_enabled: None,
        created_date: None,
        updated_date: None,
    };

    match tempDao::read_all(&mut transaction, d).await {
        Ok(rows) => {
            println!("{:?}", rows);
        }
        Err(e) => {
            event!(target: "my_api", Level::DEBUG, "{}" , e);
        }
    }
    Ok(())
}
