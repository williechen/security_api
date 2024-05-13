-- Add up migration script here
CREATE TABLE daily_task (
    row_id varchar not null default uuid_generate_v4(),
    version_code varchar not null default '',
    job_code varchar not null default '',
    exec_status varchar not null default 'WAIT',
    created_date timestamp not null default now(),
    updated_date timestamp not null default now(),
    CONSTRAINT daily_task_key PRIMARY KEY (row_id)
);

CREATE INDEX daily_task_version_code_idx ON daily_task USING btree (version_code);
CREATE INDEX daily_task_job_code_idx ON daily_task USING btree (job_code);
CREATE INDEX daily_task_exec_status_idx ON daily_task USING btree (exec_status);

COMMENT ON TABLE daily_task IS '每日執行表';

COMMENT ON COLUMN daily_task.row_id IS '序號';
COMMENT ON COLUMN daily_task.version_code IS '版本號';
COMMENT ON COLUMN daily_task.job_code IS '工作代碼';
COMMENT ON COLUMN daily_task.exec_status IS '執行狀態：等待:WAIT/開始:OPEN/執行:EXEC/結束:EXIT/停止:STOP';
COMMENT ON COLUMN daily_task.created_date IS '新增日期';
COMMENT ON COLUMN daily_task.updated_date IS '修改日期';