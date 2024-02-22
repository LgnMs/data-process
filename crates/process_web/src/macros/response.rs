#[macro_export]
macro_rules! res_template {
    ($list: expr, $current: expr, $page_size: expr, $total: expr, $msg: expr) => {{
        let data = Pagination {
            total: $total,
            list: $list,
            current: $current,
            page_size: $page_size,
        };
        ResTemplate {
            message: $msg,
            data: Some(data),
        }
    }};
    ($data: expr, $msg: expr) => {
        ResTemplate {
            message: $msg,
            data: $data,
        }
    };
}

#[macro_export]
macro_rules! res_template_ok {
    ($list: expr, $current: expr, $page_size: expr, $total: expr, $msg: expr) => {
        $crate::res_template!($list, $current, $page_size, $total, $msg)
    };
    ($list: expr, $current: expr, $page_size: expr, $total: expr) => {
        $crate::res_template!($list, $current, $page_size, $total, "操作成功".to_string())
    };
    ($data: expr, $msg: expr) => {
        $crate::res_template!($data, $msg)
    };
    ($data: expr) => {
        $crate::res_template!($data, "操作成功".to_string())
    };
}

#[macro_export]
macro_rules! res_template_err {
    ($list: expr, $current: expr, $page_size: expr, $total: expr, $msg: expr) => {
        $crate::res_template!($list, $current, $page_size, $total, $msg)
    };
    ($list: expr, $current: expr, $page_size: expr, $total: expr) => {
        $crate::res_template!($list, $current, $page_size, $total, "操作失败".to_string())
    };
    ($data: expr, $msg: expr) => {
        $crate::res_template!($data, $msg)
    };
    ($data: expr) => {
        $crate::res_template!($data, "操作失败".to_string())
    };
}

#[macro_export]
macro_rules! bool_response {
    ($res: expr) => {
        match $res {
            Ok(_) => Ok(Json($crate::res_template_ok!(Some(true)))),
            Err(err) => Ok(Json($crate::res_template_err!(
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
            Ok(data) => Ok(Json($crate::res_template_ok!(Some(data)))),
            Err(err) => Ok(Json($crate::res_template_err!(None, err.to_string()))),
        }
    };
}

#[macro_export]
macro_rules! pagination_response {
    ($res: expr, $current: expr, $page_size: expr) => {
        match $res {
            Ok((list, total)) => Ok(Json($crate::res_template_ok!(
                list, $current, $page_size, total
            ))),
            Err(err) => Ok(Json($crate::res_template_err!(
                vec![],
                $current,
                $page_size,
                0,
                err.to_string()
            ))),
        }
    };
}
