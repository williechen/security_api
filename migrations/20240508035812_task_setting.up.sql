-- Add up migration script here
CREATE TABLE task_setting (
    row_id varchar not null default uuid_generate_v4(),
    group_code varchar not null default '',
    job_code varchar not null default '',
    wait_type varchar not null default 'TS',
    wait_number integer not null default 1,
    is_enabled integer not null default 1,
    sort_no integer not null default 0,
    created_date timestamp not null default now(),
    updated_date timestamp not null default now(),
    CONSTRAINT task_setting_key PRIMARY KEY (row_id)
);

CREATE INDEX task_setting_group_code_idx ON task_setting USING btree (group_code);
CREATE INDEX task_setting_job_code_idx ON task_setting USING btree (job_code);
CREATE INDEX task_setting_sort_no_idx ON task_setting USING btree (sort_no);
CREATE INDEX task_setting_is_enabled_idx ON task_setting USING btree (is_enabled);


COMMENT ON TABLE task_setting IS '任務表';

COMMENT ON COLUMN task_setting.row_id IS '序號';
COMMENT ON COLUMN task_setting.group_code IS '任務群組';
COMMENT ON COLUMN task_setting.job_code IS '工作代碼';
COMMENT ON COLUMN task_setting.wait_type IS '等待種類：月:DM/日:DD/週:DW/時:TH/分:TM/秒:TS';
COMMENT ON COLUMN task_setting.wait_number IS '等待數量：12/30/53/24/59/59';
COMMENT ON COLUMN task_setting.is_enabled IS '是否啟用';
COMMENT ON COLUMN task_setting.sort_no IS '排序';
COMMENT ON COLUMN task_setting.created_date IS '新增日期';
COMMENT ON COLUMN task_setting.updated_date IS '修改日期';