-- Your SQL goes here
CREATE TABLE listen_flow (
    row_id varchar not null default uuid_generate_v4(),
    flow_code varchar not null default '',
    flow_param1 varchar default '',
    flow_param2 varchar default '',
    flow_param3 varchar default '',
    flow_param4 varchar default '',
    flow_param5 varchar default '',
    pid integer not null default 0,
    pstatus varchar not null default 'WAIT',
    created_date timestamp not null default now(),
    updated_date timestamp not null default now(),
    CONSTRAINT listen_flow_key PRIMARY KEY (row_id)
);

CREATE INDEX listen_flow_flow_code_idx ON listen_flow USING btree (flow_code, flow_param1, flow_param2, flow_param3, flow_param4, flow_param5);
CREATE INDEX listen_flow_pid_idx ON listen_flow USING btree (pid, pstatus);

COMMENT ON TABLE listen_flow IS '監聽流程表';

COMMENT ON COLUMN listen_flow.row_id IS '序號';
COMMENT ON COLUMN listen_flow.flow_code IS '流程代碼';
COMMENT ON COLUMN listen_flow.flow_param1 IS '參數1';
COMMENT ON COLUMN listen_flow.flow_param2 IS '參數2';
COMMENT ON COLUMN listen_flow.flow_param3 IS '參數3';
COMMENT ON COLUMN listen_flow.flow_param4 IS '參數4';
COMMENT ON COLUMN listen_flow.flow_param5 IS '參數5';
COMMENT ON COLUMN listen_flow.pid IS '線程ID';
COMMENT ON COLUMN listen_flow.pstatus IS '線程狀態';
COMMENT ON COLUMN daily_task.created_date IS '新增日期';
COMMENT ON COLUMN daily_task.updated_date IS '修改日期';