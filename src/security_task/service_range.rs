#![warn(clippy::all, clippy::pedantic)]

use std::cmp::max;

use tracing::{event, Level};

use super::{dao, model::SecurityTask};
use crate::daily_task::model::DailyTask;

pub async fn update_task_data(task: &DailyTask) -> Result<(), sqlx::Error> {
    event!(target: "security_api", Level::INFO, "call daily_task.task_range");

    let twse_list = dao::find_all_by_twse(&task).await;
    let tpex_list = dao::find_all_by_tpex(&task).await;

    let max_count = max(twse_list.len(), tpex_list.len());

    let mut sort_num = 0;
    for i in 0..max_count {
        if i < twse_list.len() {
            sort_num = sort_num + 1;

            let twse_data = &twse_list[i];
            loop_data_task_data(twse_data, sort_num).await?;
        }
        if i < tpex_list.len() {
            sort_num = sort_num + 1;

            let tpex_data = &tpex_list[i];
            loop_data_task_data(tpex_data, sort_num).await?;
        }
    }

    Ok(())
}

async fn loop_data_task_data(security: &SecurityTask, item_index: i32) -> Result<(), sqlx::Error> {
    if security.sort_no != item_index {
        let mut new_data = security.clone();
        new_data.sort_no = item_index;
        dao::modify(new_data).await?;
    }

    Ok(())
}
