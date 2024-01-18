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

macro_rules! res_template_ok {
    ($list: expr, $current: expr, $page_size: expr, $total: expr, $msg: expr) => {
        res_template!($list, $current, $page_size, $total, $msg)
    };
    ($list: expr, $current: expr, $page_size: expr, $total: expr) => {
        res_template!($list, $current, $page_size, $total, "操作成功".to_string())
    };
    ($data: expr, $msg: expr) => {
        res_template!($data, $msg)
    };
    ($data: expr) => {
        res_template!($data, "操作成功".to_string())
    };
}
macro_rules! res_template_err {
    ($list: expr, $current: expr, $page_size: expr, $total: expr, $msg: expr) => {
        res_template!($list, $current, $page_size, $total, $msg)
    };
    ($list: expr, $current: expr, $page_size: expr, $total: expr) => {
        res_template!($list, $current, $page_size, $total, "操作失败".to_string())
    };
    ($data: expr, $msg: expr) => {
        res_template!($data, $msg)
    };
    ($data: expr) => {
        res_template!($data, "操作失败".to_string())
    };
}

macro_rules! bool_response {
    ($res: expr) => {
        match $res {
            Ok(_) => Ok(Json(res_template_ok!(Some(true)))),
            Err(err) => Ok(Json(res_template_err!(Some(false), err.to_string()))),
        }
    };
}
macro_rules! data_response {
    ($res: expr) => {
        match $res {
            Ok(data) => Ok(Json(res_template_ok!(Some(data)))),
            Err(err) => Ok(Json(res_template_err!(None, err.to_string()))),
        }
    };
}

macro_rules! pagination_response {
    ($res: expr, $current: expr, $page_size: expr) => {
        match $res {
            Ok((list, total)) => Ok(Json(res_template_ok!(list, $current, $page_size, total))),
            Err(err) => Ok(Json(res_template_err!(
                vec![],
                $current,
                $page_size,
                0,
                err.to_string()
            ))),
        }
    };
}
