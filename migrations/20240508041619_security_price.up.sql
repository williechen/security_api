-- Add up migration script here
CREATE TABLE security_price (
    row_id varchar not null default uuid_generate_v4(),
    stock_code varchar not null default '',
    price_date varchar not null default '',
    price_close numeric not null default 0,
    price_avg numeric not null default 0,
    price_hight numeric not null default 0,
    price_hight_avg numeric not null default 0,
    price_lowest numeric not null default 0,
    price_lowest_avg numeric not null default 0,
    created_date timestamp not null default now(),
    updated_date timestamp not null default now(),
    CONSTRAINT security_price_key PRIMARY KEY (row_id)
);

CREATE INDEX security_price_stock_code_idx ON security_price USING btree (stock_code);
CREATE INDEX security_price_price_date_idx ON security_price USING btree (price_date);
CREATE INDEX security_price_price_close_idx ON security_price USING btree (price_close);


COMMENT ON TABLE security_price IS 'This is my table.';

COMMENT ON COLUMN security_price.row_id IS 'Employee ID number';
COMMENT ON COLUMN security_price.stock_code IS 'Employee ID number';
COMMENT ON COLUMN security_price.price_date IS 'Employee ID number';
COMMENT ON COLUMN security_price.price_close IS 'Employee ID number';
COMMENT ON COLUMN security_price.price_avg IS 'Employee ID number';
COMMENT ON COLUMN security_price.price_hight IS 'Employee ID number';
COMMENT ON COLUMN security_price.price_hight_avg IS 'Employee ID number';
COMMENT ON COLUMN security_price.price_lowest IS 'Employee ID number';
COMMENT ON COLUMN security_price.price_lowest_avg IS 'Employee ID number';
COMMENT ON COLUMN security_price.created_date IS 'Employee ID number';
COMMENT ON COLUMN security_price.updated_date IS 'Employee ID number';