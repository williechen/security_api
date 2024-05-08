-- Add up migration script here
CREATE TABLE calendar_data (
    row_id varchar not null default uuid_generate_v4(),
    ce_year integer not null default 0,
    tw_year integer not null default 0,
    ce_month integer not null default 0,
    ce_day integer not null default 0,
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
CREATE INDEX calendar_data_task_type_idx ON calendar_data USING btree (task_type);


COMMENT ON TABLE calendar_data IS 'This is my table.';

COMMENT ON COLUMN calendar_data.row_id IS 'Employee ID number';
COMMENT ON COLUMN calendar_data.ce_year IS 'Employee ID number';
COMMENT ON COLUMN calendar_data.tw_year IS 'Employee ID number';
COMMENT ON COLUMN calendar_data.ce_month IS 'Employee ID number';
COMMENT ON COLUMN calendar_data.ce_day IS 'Employee ID number';
COMMENT ON COLUMN calendar_data.date_status IS 'Employee ID number';
COMMENT ON COLUMN calendar_data.task_type IS 'Employee ID number';
COMMENT ON COLUMN calendar_data.created_date IS 'Employee ID number';
COMMENT ON COLUMN calendar_data.updated_date IS 'Employee ID number';