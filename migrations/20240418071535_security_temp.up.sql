-- Add up migration script here
CREATE TABLE security_temp (
    row_id varchar not null default uuid_generate_v4(),
    version_code varchar not null default '',
    international_code varchar not null default '',
    security_code varchar not null default '',
    security_name varchar not null default '',
    market_type varchar not null default '',
    security_type varchar not null default '',
    industry_type varchar not null default '',
    issue_date varchar not null default '',
    cfi_code varchar not null default '',
    remark varchar not null default '', 
    is_enabled integer not null default 0,
    created_date timestamp not null default now(),
    updated_date timestamp not null default now(),
    CONSTRAINT security_temp_key PRIMARY KEY (row_id)
);

CREATE INDEX security_temp_version_code_idx ON security_temp USING btree (version_code);
CREATE INDEX security_temp_security_code_idx ON security_temp USING btree (security_code);
CREATE INDEX security_temp_security_name_idx ON security_temp USING btree (security_name);
CREATE INDEX security_temp_market_type_idx ON security_temp USING btree (market_type);
CREATE INDEX security_temp_security_type_idx ON security_temp USING btree (security_type);

COMMENT ON TABLE security_temp IS 'This is my table.';

COMMENT ON COLUMN security_temp.row_id IS 'Employee ID number';
COMMENT ON COLUMN security_temp.version_code IS 'Employee ID number';
COMMENT ON COLUMN security_temp.international_code IS 'Employee ID number';
COMMENT ON COLUMN security_temp.security_code IS 'Employee ID number';
COMMENT ON COLUMN security_temp.security_name IS 'Employee ID number';
COMMENT ON COLUMN security_temp.market_type IS 'Employee ID number';
COMMENT ON COLUMN security_temp.security_type IS 'Employee ID number';
COMMENT ON COLUMN security_temp.industry_type IS 'Employee ID number';
COMMENT ON COLUMN security_temp.issue_date IS 'Employee ID number';
COMMENT ON COLUMN security_temp.cfi_code IS 'Employee ID number';
COMMENT ON COLUMN security_temp.remark IS 'Employee ID number';
COMMENT ON COLUMN security_temp.is_enabled IS 'Employee ID number';
COMMENT ON COLUMN security_temp.created_date IS 'Employee ID number';
COMMENT ON COLUMN security_temp.updated_date IS 'Employee ID number';