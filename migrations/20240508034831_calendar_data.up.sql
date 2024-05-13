-- Add up migration script here
CREATE TABLE calendar_data (
    row_id varchar not null default uuid_generate_v4(),
    ce_year varchar not null default '1962',
    tw_year varchar not null default '051',
    ce_month varchar not null default '01',
    ce_day varchar not null default '01',
    date_status varchar not null default 'W',
    group_task varchar not null default 'ALL',
    created_date timestamp not null default now(),
    updated_date timestamp not null default now(),
    CONSTRAINT calendar_data_key PRIMARY KEY (row_id)
);

CREATE INDEX calendar_data_ce_year_idx ON calendar_data USING btree (ce_year);
CREATE INDEX calendar_data_tw_year_idx ON calendar_data USING btree (tw_year);
CREATE INDEX calendar_data_ce_month_idx ON calendar_data USING btree (ce_month);
CREATE INDEX calendar_data_ce_day_idx ON calendar_data USING btree (ce_day);
CREATE INDEX calendar_data_date_status_idx ON calendar_data USING btree (date_status);
CREATE INDEX calendar_data_group_task_idx ON calendar_data USING btree (group_task);


COMMENT ON TABLE calendar_data IS '行事曆';

COMMENT ON COLUMN calendar_data.row_id IS '序號';
COMMENT ON COLUMN calendar_data.ce_year IS '西元年';
COMMENT ON COLUMN calendar_data.tw_year IS '民國年';
COMMENT ON COLUMN calendar_data.ce_month IS '西元月';
COMMENT ON COLUMN calendar_data.ce_day IS '西元日';
COMMENT ON COLUMN calendar_data.date_status IS '工作:W/假日:H/颱風:T';
COMMENT ON COLUMN calendar_data.group_task IS '任務群組';
COMMENT ON COLUMN calendar_data.created_date IS '新增日期';
COMMENT ON COLUMN calendar_data.updated_date IS '修改日期';