#![warn(clippy::all, clippy::pedantic)]

#[derive(Debug, Clone)]
pub struct ListenFlow {
    pub row_id: String,
    pub flow_code: String,
    pub flow_param1: Option<String>,
    pub flow_param2: Option<String>,
    pub flow_param3: Option<String>,
    pub flow_param4: Option<String>,
    pub flow_param5: Option<String>,
    pub pid: i32,
    pub pstatus: String,
}

impl std::fmt::Display for ListenFlow {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let flow_code = self.flow_code.clone();
        let flow_param1 = self.flow_param1.clone().unwrap_or(String::from(""));
        let flow_param2 = self.flow_param2.clone().unwrap_or(String::from(""));
        let flow_param3 = self.flow_param3.clone().unwrap_or(String::from(""));
        let flow_param4 = self.flow_param4.clone().unwrap_or(String::from(""));
        let flow_param5 = self.flow_param5.clone().unwrap_or(String::from(""));
        let pid = self.pid;

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
