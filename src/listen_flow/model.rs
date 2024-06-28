#![warn(clippy::all, clippy::pedantic)]

#[derive(Debug, Clone)]
pub struct ListenFlow {
    pub flow_code: Option<String>,
    pub flow_param1: Option<String>,
    pub flow_param2: Option<String>,
    pub flow_param3: Option<String>,
    pub flow_param4: Option<String>,
    pub flow_param5: Option<String>,
    pub pid: Option<i32>,
}

impl ListenFlow {
    pub fn new() -> Self {
        ListenFlow {
            flow_code: None,
            flow_param1: None,
            flow_param2: None,
            flow_param3: None,
            flow_param4: None,
            flow_param5: None,
            pid: None,
        }
    }
}

impl std::fmt::Display for ListenFlow {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let flow_code = self.flow_code.clone().unwrap_or(String::from(""));
        let flow_param1 = self.flow_param1.clone().unwrap_or(String::from(""));
        let flow_param2 = self.flow_param2.clone().unwrap_or(String::from(""));
        let flow_param3 = self.flow_param3.clone().unwrap_or(String::from(""));
        let flow_param4 = self.flow_param4.clone().unwrap_or(String::from(""));
        let flow_param5 = self.flow_param5.clone().unwrap_or(String::from(""));
        let pid = self.pid.unwrap();

        write!(
            f,
            r#"{0}, 
            flow_param1: {1}, 
            flow_param1: {2}, 
            flow_param1: {3},
            flow_param1: {4},
            flow_param1: {5},
            pid: {6}
            "#,
            flow_code, flow_param1, flow_param2, flow_param3, flow_param4, flow_param5, pid
        )
    }
}
