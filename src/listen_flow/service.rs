#![warn(clippy::all, clippy::pedantic)]

use super::{
    dao,
    model::{ListenFlow, NewListenFlow},
};
use chrono::Local;

pub fn read_flow_data(flow_code: &str, flow_param1: &str, flow_param2: &str) -> Vec<ListenFlow> {
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
        created_date: Local::now().naive_local(),
        updated_date: Local::now().naive_local(),
    };

    dao::find_all(listen_flow)
}

pub fn delete_flow_data(flow_code: &str) {
    dao::remove_all(flow_code).unwrap();
}

pub fn insert_flow_data2(pid: i32, flow_code: &str, flow_param1: &str, flow_param2: &str) {
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
        created_date: Local::now().naive_local(),
        updated_date: Local::now().naive_local(),
    };

    let flows = dao::find_all(listen_flow);
    if flows.len() <= 0 {
        let new_listen_flow = NewListenFlow {
            flow_code: flow_code.to_string(),
            flow_param1: Some(flow_param1.to_string()),
            flow_param2: Some(flow_param2.to_string()),
            flow_param3: None,
            flow_param4: None,
            flow_param5: None,
            pid,
            pstatus: "WAIT".to_string(),
            created_date: Local::now().naive_local(),
            updated_date: Local::now().naive_local(),
        };

        dao::create(new_listen_flow).unwrap();
    }
}

pub fn modify_flow_data2(pid: i32, flow_code: &str, flow_param1: &str, flow_param2: &str) {
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
        created_date: Local::now().naive_local(),
        updated_date: Local::now().naive_local(),
    };

    let flows = dao::find_all(listen_flow);
    if flows.len() <= 0 {
        let mut new_flow = flows[0].clone();
        new_flow.pstatus = "EXIT".to_string();
        new_flow.updated_date = Local::now().naive_local();

        dao::modify(new_flow).unwrap();
    }
}
