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
        write!(
            f,
            r#"{0}, 
            flow_param1: {1:#?}, 
            flow_param1: {2:#?}, 
            flow_param1: {3:#?},
            flow_param1: {4:#?},
            flow_param1: {5:#?},
            pid: {6}
            "#,
            self.flow_code,
            self.flow_param1,
            self.flow_param2,
            self.flow_param3,
            self.flow_param4,
            self.flow_param5,
            self.pid
        )
    }
}
