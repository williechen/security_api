-- Your SQL goes here
CREATE TABLE response_data (
    row_id varchar not null default uuid_generate_v4(),
    open_date_year varchar not null default '',
    open_date_month varchar not null default '',
    open_date_day varchar not null default '',
    exec_code varchar not null default '',
    data_content text not null default '',
    created_date timestamp not null default now(),
    updated_date timestamp not null default now(),
    CONSTRAINT response_data_key PRIMARY KEY (row_id)
);

CREATE INDEX response_data_exec_code_idx ON response_data USING btree (exec_code);
CREATE INDEX response_data_open_date_idx ON response_data USING btree (open_date_year, open_date_month, open_date_day);

COMMENT ON TABLE response_data IS '網頁資料';

COMMENT ON COLUMN response_data.row_id IS '序號';
COMMENT ON COLUMN response_data.open_date_year IS '開市日期_年';
COMMENT ON COLUMN response_data.open_date_month IS '開市日期_月';
COMMENT ON COLUMN response_data.open_date_day IS '開市日期_日';
COMMENT ON COLUMN response_data.exec_code IS '執行代碼';
COMMENT ON COLUMN response_data.data_content IS '資料內容';
COMMENT ON COLUMN response_data.created_date IS '新增日期';
COMMENT ON COLUMN response_data.updated_date IS '修改日期';