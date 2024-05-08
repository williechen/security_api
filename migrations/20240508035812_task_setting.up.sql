-- Add up migration script here
CREATE TABLE task_setting (
    row_id varchar not null default uuid_generate_v4(),
    group_code varchar not null default '',
    task_code varchar not null default '',
    wait_type varchar not null default 'S',
    wait_number integer not null default 1,
    wait_last_step integer not null default 1,
    sort_no integer not null default 0,
    is_enabled integer not null default 1,
    created_date timestamp not null default now(),
    updated_date timestamp not null default now(),
    CONSTRAINT task_setting_key PRIMARY KEY (row_id)
);

CREATE INDEX task_setting_group_code_idx ON task_setting USING btree (group_code);
CREATE INDEX task_setting_task_code_idx ON task_setting USING btree (task_code);
CREATE INDEX task_setting_wait_last_step_idx ON task_setting USING btree (wait_last_step);
CREATE INDEX task_setting_sort_no_idx ON task_setting USING btree (sort_no);
CREATE INDEX task_setting_is_enabled_idx ON task_setting USING btree (is_enabled);


COMMENT ON TABLE task_setting IS 'This is my table.';

COMMENT ON COLUMN task_setting.row_id IS 'Employee ID number';
COMMENT ON COLUMN task_setting.group_code IS 'Employee ID number';
COMMENT ON COLUMN task_setting.task_code IS 'Employee ID number';
COMMENT ON COLUMN task_setting.wait_type IS 'Employee ID number';
COMMENT ON COLUMN task_setting.wait_number IS 'Employee ID number';
COMMENT ON COLUMN task_setting.wait_last_step IS 'Employee ID number';
COMMENT ON COLUMN task_setting.sort_no IS 'Employee ID number';
COMMENT ON COLUMN task_setting.is_enabled IS 'Employee ID number';
COMMENT ON COLUMN task_setting.created_date IS 'Employee ID number';
COMMENT ON COLUMN task_setting.updated_date IS 'Employee ID number';