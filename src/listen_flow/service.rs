#![warn(clippy::all, clippy::pedantic)]

use super::{dao, model::ListenFlow};

pub async fn read_flow_data(
    flow_code: &str,
    flow_param1: &str,
    flow_param2: &str,
) -> Vec<ListenFlow> {
    let listen_flow = ListenFlow {
        row_id: String::new(),
        flow_code: flow_code.to_string(),
        flow_param1: Some(flow_param1.to_string()),
        flow_param2: Some(flow_param2.to_string()),
        flow_param3: None,
        flow_param4: None,
        flow_param5: None,
        pid: 0,
        pstatus: String::new(),
    };

    dao::find_all(&listen_flow).await
}

pub async fn delete_flow_data(flow_code: &str) {
    dao::remove_all(flow_code).await.unwrap();
}

pub async fn insert_flow_data2(pid: i32, flow_code: &str, flow_param1: &str, flow_param2: &str) {
    let listen_flow = ListenFlow {
        row_id: String::new(),
        flow_code: flow_code.to_string(),
        flow_param1: Some(flow_param1.to_string()),
        flow_param2: Some(flow_param2.to_string()),
        flow_param3: None,
        flow_param4: None,
        flow_param5: None,
        pid,
        pstatus: String::new(),
    };

    let flows = dao::find_all(&listen_flow).await;
    if flows.len() <= 0 {
        let new_listen_flow = ListenFlow {
            row_id: String::new(),
            flow_code: flow_code.to_string(),
            flow_param1: Some(flow_param1.to_string()),
            flow_param2: Some(flow_param2.to_string()),
            flow_param3: None,
            flow_param4: None,
            flow_param5: None,
            pid,
            pstatus: "WAIT".to_string(),
        };

        dao::create(new_listen_flow).await.unwrap();
    }
}

pub async fn modify_flow_data2(pid: i32, flow_code: &str, flow_param1: &str, flow_param2: &str) {
    let listen_flow = ListenFlow {
        row_id: String::new(),
        flow_code: flow_code.to_string(),
        flow_param1: Some(flow_param1.to_string()),
        flow_param2: Some(flow_param2.to_string()),
        flow_param3: None,
        flow_param4: None,
        flow_param5: None,
        pid,
        pstatus: String::new(),
    };

    let flows = dao::find_all(&listen_flow).await;
    for flow in flows {
        let mut new_flow = flow;
        new_flow.pstatus = "EXIT".to_string();

        dao::modify(new_flow).await.unwrap();
    }
}
