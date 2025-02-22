-- Your SQL goes here
CREATE TABLE security_price (
    row_id varchar not null default uuid_generate_v4(),
    open_date_year varchar not null default '',
    open_date_month varchar not null default '',
    open_date_day varchar not null default '',
    security_code varchar not null default '',
    security_name varchar not null default '',
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

CREATE INDEX security_price_open_date_idx ON security_price USING btree (open_date_year, open_date_month, open_date_day);
CREATE INDEX security_price_price_date_idx ON security_price USING btree (security_code, price_date);


COMMENT ON TABLE security_price IS '每日收盤價';

COMMENT ON COLUMN security_price.row_id IS '序號';
COMMENT ON COLUMN security_price.open_date_year IS '開市日期_年';
COMMENT ON COLUMN security_price.open_date_month IS '開市日期_月';
COMMENT ON COLUMN security_price.open_date_day IS '開市日期_日';
COMMENT ON COLUMN security_price.security_code IS '證券代碼';
COMMENT ON COLUMN security_price.security_name IS '證券名稱';
COMMENT ON COLUMN security_price.price_date IS '收盤日期';
COMMENT ON COLUMN security_price.price_close IS '收盤價值';
COMMENT ON COLUMN security_price.price_avg IS '平均價值';
COMMENT ON COLUMN security_price.price_hight IS '最高價值';
COMMENT ON COLUMN security_price.price_hight_avg IS '平均最高價值';
COMMENT ON COLUMN security_price.price_lowest IS '最低價值';
COMMENT ON COLUMN security_price.price_lowest_avg IS '平均最低價值';
COMMENT ON COLUMN security_price.created_date IS '新增日期';
COMMENT ON COLUMN security_price.updated_date IS '修改日期';