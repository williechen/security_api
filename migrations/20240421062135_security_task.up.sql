-- Add up migration script here
CREATE TABLE security_task (
    row_id varchar not null default uuid_generate_v4(),
    security_url varchar not null default '',
    market_type varchar not null default '',
    security_code varchar not null default '',
    issue_date varchar not null default '',
    twse_date varchar not null default '',
    tpex_date varchar not null default '',
    security_seed varchar not null default '',
    is_enabled integer not null default 0,
    sort_no integer not null default 99999,
    created_date timestamp not null default now(),
    updated_date timestamp not null default now(),
    CONSTRAINT security_task_key PRIMARY KEY (row_id)
);

CREATE INDEX security_task_security_code_idx ON security_task USING btree (security_code);
CREATE INDEX security_task_twse_date_idx ON security_task USING btree (twse_date);
CREATE INDEX security_task_tpex_date_idx ON security_task USING btree (tpex_date);
CREATE INDEX security_task_is_enabled_idx ON security_task USING btree (is_enabled);
CREATE INDEX security_task_sort_no_idx ON security_task USING btree (sort_no);


COMMENT ON TABLE security_task IS 'This is my table.';

COMMENT ON COLUMN security_task.row_id IS 'Employee ID number';
COMMENT ON COLUMN security_task.security_url IS 'Employee ID number';
COMMENT ON COLUMN security_task.market_type IS 'Employee ID number';
COMMENT ON COLUMN security_task.security_code IS 'Employee ID number';
COMMENT ON COLUMN security_task.issue_date IS 'Employee ID number';
COMMENT ON COLUMN security_task.twse_date IS 'Employee ID number';
COMMENT ON COLUMN security_task.tpex_date IS 'Employee ID number';
COMMENT ON COLUMN security_task.security_seed IS 'Employee ID number';
COMMENT ON COLUMN security_task.is_enabled IS 'Employee ID number';
COMMENT ON COLUMN security_task.sort_no IS 'Employee ID number';
COMMENT ON COLUMN security_task.created_date IS 'Employee ID number';
COMMENT ON COLUMN security_task.updated_date IS 'Employee ID number';