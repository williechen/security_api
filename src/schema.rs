// @generated automatically by Diesel CLI.

diesel::table! {
    /// 行事曆
    calendar_data (row_id) {
        /// 序號
        row_id -> Varchar,
        /// 西元年
        ce_year -> Varchar,
        /// 西元月
        ce_month -> Varchar,
        /// 西元日
        ce_day -> Varchar,
        ///
        week_index -> Int4,
        /// 開市:O/休市:S
        date_status -> Varchar,
        /// 任務群組
        group_task -> Varchar,
        /// 新增日期
        created_date -> Timestamp,
        /// 修改日期
        updated_date -> Timestamp,
    }
}

diesel::table! {
    /// 每日執行表
    daily_task (row_id) {
        /// 序號
        row_id -> Varchar,
        /// 開市日期_年
        open_date_year -> Varchar,
        /// 開市日期_月
        open_date_month -> Varchar,
        /// 開市日期_日
        open_date_day -> Varchar,
        /// 工作代碼
        job_code -> Varchar,
        /// 執行狀態：等待:WAIT/開始:OPEN/執行:EXEC/結束:EXIT/停止:STOP
        exec_status -> Varchar,
        /// 新增日期
        created_date -> Timestamp,
        /// 修改日期
        updated_date -> Timestamp,
    }
}

diesel::table! {
    /// 監聽流程表
    listen_flow (row_id) {
        /// 序號
        row_id -> Varchar,
        /// 流程代碼
        flow_code -> Varchar,
        /// 參數1
        flow_param1 -> Nullable<Varchar>,
        /// 參數2
        flow_param2 -> Nullable<Varchar>,
        /// 參數3
        flow_param3 -> Nullable<Varchar>,
        /// 參數4
        flow_param4 -> Nullable<Varchar>,
        /// 參數5
        flow_param5 -> Nullable<Varchar>,
        /// 線程ID
        pid -> Int4,
        /// 線程狀態
        pstatus -> Varchar,
        /// The `created_date` column of the `listen_flow` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        created_date -> Timestamp,
        /// The `updated_date` column of the `listen_flow` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        updated_date -> Timestamp,
    }
}

diesel::table! {
    /// 網頁資料
    response_data (row_id) {
        /// 序號
        row_id -> Varchar,
        /// 開市日期_年
        open_date_year -> Varchar,
        /// 開市日期_月
        open_date_month -> Varchar,
        /// 開市日期_日
        open_date_day -> Varchar,
        /// 執行代碼
        exec_code -> Varchar,
        /// 資料內容
        data_content -> Text,
        /// 新增日期
        created_date -> Timestamp,
        /// 修改日期
        updated_date -> Timestamp,
    }
}

diesel::table! {
    /// 每日收盤價
    security_price (row_id) {
        /// 序號
        row_id -> Varchar,
        /// 開市日期_年
        open_date_year -> Varchar,
        /// 開市日期_月
        open_date_month -> Varchar,
        /// 開市日期_日
        open_date_day -> Varchar,
        /// 證券代碼
        security_code -> Varchar,
        /// 證券名稱
        security_name -> Varchar,
        /// 收盤日期
        price_date -> Varchar,
        /// 收盤價值
        price_close -> Numeric,
        /// 平均價值
        price_avg -> Numeric,
        /// 最高價值
        price_hight -> Numeric,
        /// 平均最高價值
        price_hight_avg -> Numeric,
        /// 最低價值
        price_lowest -> Numeric,
        /// 平均最低價值
        price_lowest_avg -> Numeric,
        /// 新增日期
        created_date -> Timestamp,
        /// 修改日期
        updated_date -> Timestamp,
    }
}

diesel::table! {
    /// 證券任務
    security_task (row_id) {
        /// 序號
        row_id -> Varchar,
        /// 開市日期_年
        open_date_year -> Varchar,
        /// 開市日期_月
        open_date_month -> Varchar,
        /// 開市日期_日
        open_date_day -> Varchar,
        /// 證券代碼
        security_code -> Varchar,
        /// 證券名稱
        security_name -> Varchar,
        /// 市場別
        market_type -> Varchar,
        /// 發行日期
        issue_date -> Varchar,
        /// 種子數
        exec_seed -> Varchar,
        /// 執行次數
        exec_count -> Int4,
        /// 是否啟用
        is_enabled -> Int4,
        /// 排序
        sort_no -> Int4,
        /// 新增日期
        created_date -> Timestamp,
        /// 修改日期
        updated_date -> Timestamp,
    }
}

diesel::table! {
    /// 證券暫存
    security_temp (row_id) {
        /// 序號
        row_id -> Varchar,
        /// 開市日期_年
        open_date_year -> Varchar,
        /// 開市日期_月
        open_date_month -> Varchar,
        /// 開市日期_日
        open_date_day -> Varchar,
        /// 國際代碼
        international_code -> Varchar,
        /// 代碼
        security_code -> Varchar,
        /// 名稱
        security_name -> Varchar,
        /// 市場別
        market_type -> Varchar,
        /// 證券別
        security_type -> Varchar,
        /// 行業別
        industry_type -> Varchar,
        /// 發行日
        issue_date -> Varchar,
        /// cfi_code
        cfi_code -> Varchar,
        /// 備註
        remark -> Varchar,
        /// 新增日期
        created_date -> Timestamp,
        /// 修改日期
        updated_date -> Timestamp,
    }
}

diesel::table! {
    /// 任務表
    task_setting (row_id) {
        /// 序號
        row_id -> Varchar,
        /// 任務群組
        group_code -> Varchar,
        /// 工作代碼
        job_code -> Varchar,
        /// 等待種類：月:DM/日:DD/週:DW/時:TH/分:TM/秒:TS
        wait_type -> Varchar,
        /// 等待數量：12/30/53/24/59/59
        wait_number -> Int4,
        /// 是否啟用
        is_enabled -> Int4,
        /// 排序
        sort_no -> Int4,
        /// 新增日期
        created_date -> Timestamp,
        /// 修改日期
        updated_date -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    calendar_data,
    daily_task,
    listen_flow,
    response_data,
    security_price,
    security_task,
    security_temp,
    task_setting,
);
