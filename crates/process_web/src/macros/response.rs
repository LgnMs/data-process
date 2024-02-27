#[macro_export]
macro_rules! res_template {
    ($list: expr, $current: expr, $page_size: expr, $total: expr, $msg: expr, $success: expr) => {{
        let data = $crate::api::common::Pagination {
            total: $total,
            list: $list,
            current: $current,
            page_size: $page_size,
        };
        $crate::api::common::ResTemplate {
            message: $msg,
            data: Some(data),
            success: $success
        }
    }};
    ($data: expr, $msg: expr, $success: expr) => {
        $crate::api::common::ResTemplate {
            message: $msg,
            data: $data,
            success: $success
        }
    };
}

#[macro_export]
macro_rules! res_template_ok {
    ($list: expr, $current: expr, $page_size: expr, $total: expr, $msg: expr) => {
        $crate::res_template!($list, $current, $page_size, $total, $msg, true)
    };
    ($list: expr, $current: expr, $page_size: expr, $total: expr) => {
        $crate::res_template!($list, $current, $page_size, $total, "操作成功".to_string(), true)
    };
    ($data: expr, $msg: expr) => {
        $crate::res_template!($data, $msg, true)
    };
    ($data: expr) => {
        $crate::res_template!($data, "操作成功".to_string(), true)
    };
}

#[macro_export]
macro_rules! res_template_err {
    ($list: expr, $current: expr, $page_size: expr, $total: expr, $msg: expr) => {
        $crate::res_template!($list, $current, $page_size, $total, $msg, false)
    };
    ($list: expr, $current: expr, $page_size: expr, $total: expr) => {
        $crate::res_template!($list, $current, $page_size, $total, "操作失败".to_string(), false)
    };
    ($data: expr, $msg: expr) => {
        $crate::res_template!($data, $msg, false)
    };
    ($data: expr) => {
        $crate::res_template!($data, "操作失败".to_string(), false)
    };
}

#[macro_export]
macro_rules! bool_response {
    ($res: expr) => {
        match $res {
            Ok(_) => Ok(::axum::Json($crate::res_template_ok!(Some(true)))),
            Err(err) => Ok(::axum::Json($crate::res_template_err!(
                Some(false),
                err.to_string()
            ))),
        }
    };
}

#[macro_export]
macro_rules! data_response {
    ($res: expr) => {
        match $res {
            Ok(data) => Ok(::axum::Json($crate::res_template_ok!(Some(data)))),
            Err(err) => Ok(::axum::Json($crate::res_template_err!(
                None,
                err.to_string()
            ))),
        }
    };
}

#[macro_export]
macro_rules! pagination_response {
    ($res: expr, $current: expr, $page_size: expr) => {
        match $res {
            Ok((list, total)) => Ok(::axum::Json($crate::res_template_ok!(
                list, $current, $page_size, total
            ))),
            Err(err) => Ok(::axum::Json($crate::res_template_err!(
                vec![],
                $current,
                $page_size,
                0,
                err.to_string()
            ))),
        }
    };
}
