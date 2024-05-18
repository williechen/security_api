-- Add up migration script here
CREATE TABLE security_task (
    row_id varchar not null default uuid_generate_v4(),
    open_date varchar not null default '',
    security_code varchar not null default '',
    security_name varchar not null default '',
    market_type varchar not null default '',
    issue_date varchar not null default '',
    security_date varchar not null default '',
    security_seed varchar not null default '',
    exec_count integer not null default 0,
    is_enabled integer not null default 0,
    sort_no integer not null default 99999,
    created_date timestamp not null default now(),
    updated_date timestamp not null default now(),
    CONSTRAINT security_task_key PRIMARY KEY (row_id)
);

CREATE INDEX security_task_open_date_idx ON security_task USING btree (open_date);
CREATE INDEX security_task_security_code_idx ON security_task USING btree (security_code);
CREATE INDEX security_task_market_type_idx ON security_task USING btree (market_type);
CREATE INDEX security_task_security_date_idx ON security_task USING btree (security_date);
CREATE INDEX security_task_is_enabled_idx ON security_task USING btree (is_enabled);
CREATE INDEX security_task_sort_no_idx ON security_task USING btree (sort_no);
CREATE INDEX security_task_exec_count_idx ON security_task USING btree (exec_count);


COMMENT ON TABLE security_task IS '證券任務';

COMMENT ON COLUMN security_task.row_id IS '序號';
COMMENT ON COLUMN security_task.open_date IS '開市日期';
COMMENT ON COLUMN security_task.security_code IS '證券代碼';
COMMENT ON COLUMN security_task.security_name IS '證券名稱';
COMMENT ON COLUMN security_task.market_type IS '市場別';
COMMENT ON COLUMN security_task.issue_date IS '發行日期';
COMMENT ON COLUMN security_task.security_date IS '開市日期';
COMMENT ON COLUMN security_task.security_seed IS '種子數';
COMMENT ON COLUMN security_task.is_enabled IS '是否啟用';
COMMENT ON COLUMN security_task.sort_no IS '排序';
COMMENT ON COLUMN security_task.exec_count IS '執行次數';
COMMENT ON COLUMN security_task.created_date IS '新增日期';
COMMENT ON COLUMN security_task.updated_date IS '修改日期';