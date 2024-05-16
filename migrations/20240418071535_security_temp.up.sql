-- Add up migration script here
CREATE TABLE security_temp (
    row_id varchar not null default uuid_generate_v4(),
    open_date varchar not null default '',
    international_code varchar not null default '',
    security_code varchar not null default '',
    security_name varchar not null default '',
    market_type varchar not null default '',
    security_type varchar not null default '',
    industry_type varchar not null default '',
    issue_date varchar not null default '',
    cfi_code varchar not null default '',
    remark varchar not null default '', 
    created_date timestamp not null default now(),
    updated_date timestamp not null default now(),
    CONSTRAINT security_temp_key PRIMARY KEY (row_id)
);

CREATE INDEX security_temp_open_date_idx ON security_temp USING btree (open_date);
CREATE INDEX security_temp_security_code_idx ON security_temp USING btree (security_code);
CREATE INDEX security_temp_security_name_idx ON security_temp USING btree (security_name);
CREATE INDEX security_temp_market_type_idx ON security_temp USING btree (market_type);
CREATE INDEX security_temp_security_type_idx ON security_temp USING btree (security_type);

COMMENT ON TABLE security_temp IS '證券暫存';

COMMENT ON COLUMN security_temp.row_id IS '序號';
COMMENT ON COLUMN security_temp.open_date IS '開市日期';
COMMENT ON COLUMN security_temp.international_code IS '國際代碼';
COMMENT ON COLUMN security_temp.security_code IS '代碼';
COMMENT ON COLUMN security_temp.security_name IS '名稱';
COMMENT ON COLUMN security_temp.market_type IS '市場別';
COMMENT ON COLUMN security_temp.security_type IS '證券別';
COMMENT ON COLUMN security_temp.industry_type IS '行業別';
COMMENT ON COLUMN security_temp.issue_date IS '發行日';
COMMENT ON COLUMN security_temp.cfi_code IS 'cfi_code';
COMMENT ON COLUMN security_temp.remark IS '備註';
COMMENT ON COLUMN security_temp.created_date IS '新增日期';
COMMENT ON COLUMN security_temp.updated_date IS '修改日期';