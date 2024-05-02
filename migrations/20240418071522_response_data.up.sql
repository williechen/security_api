-- Add up migration script here
CREATE TABLE response_data (
    row_id varchar not null default uuid_generate_v4(),
    data_content text not null default '',
    data_code varchar not null,
    read_date varchar not null,
    created_date timestamp not null default now(),
    updated_date timestamp not null default now(),
    CONSTRAINT response_data_key PRIMARY KEY (row_id)
);

CREATE INDEX response_data_data_code_idx ON response_data USING btree (data_code);
CREATE INDEX response_data_read_date_idx ON response_data USING btree (read_date);

COMMENT ON TABLE response_data IS 'This is my table.';

COMMENT ON COLUMN response_data.row_id IS 'Employee ID number';
COMMENT ON COLUMN response_data.data_content IS 'Employee ID number';
COMMENT ON COLUMN response_data.data_code IS 'Employee ID number';
COMMENT ON COLUMN response_data.read_date IS 'Employee ID number';
COMMENT ON COLUMN response_data.created_date IS 'Employee ID number';
COMMENT ON COLUMN response_data.updated_date IS 'Employee ID number';