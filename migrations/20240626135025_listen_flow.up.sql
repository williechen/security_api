-- Add up migration script here
CREATE TABLE listen_flow (
    flow_code varchar not null default '',
    flow_param1 varchar default '',
    flow_param2 varchar default '',
    flow_param3 varchar default '',
    flow_param4 varchar default '',
    flow_param5 varchar default '',
    pid integer not null default 0,
    created_date timestamp not null default now(),
    updated_date timestamp not null default now()
);

CREATE INDEX listen_flow_flow_code_idx ON listen_flow USING btree (flow_code);
CREATE INDEX listen_flow_flow_param1_idx ON listen_flow USING btree (flow_param1);
CREATE INDEX listen_flow_flow_param2_idx ON listen_flow USING btree (flow_param2);
CREATE INDEX listen_flow_flow_param3_idx ON listen_flow USING btree (flow_param3);
CREATE INDEX listen_flow_flow_param4_idx ON listen_flow USING btree (flow_param4);
CREATE INDEX listen_flow_flow_param5_idx ON listen_flow USING btree (flow_param5);
CREATE INDEX listen_flow_pid_idx ON listen_flow USING btree (pid);

COMMENT ON TABLE listen_flow IS '監聽流程表';

COMMENT ON COLUMN listen_flow.flow_code IS '流程代碼';
COMMENT ON COLUMN listen_flow.flow_param1 IS '參數1';
COMMENT ON COLUMN listen_flow.flow_param2 IS '參數2';
COMMENT ON COLUMN listen_flow.flow_param3 IS '參數3';
COMMENT ON COLUMN listen_flow.flow_param4 IS '參數4';
COMMENT ON COLUMN listen_flow.flow_param5 IS '參數5';
COMMENT ON COLUMN listen_flow.pid IS '線程ID';
COMMENT ON COLUMN daily_task.created_date IS '新增日期';
COMMENT ON COLUMN daily_task.updated_date IS '修改日期';