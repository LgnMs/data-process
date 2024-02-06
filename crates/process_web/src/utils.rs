use anyhow::anyhow;
use chrono::Local;
use sea_orm::DbErr;
use std::collections::HashMap;
use tokio_cron_scheduler::JobSchedulerError;

/// ```md
/// rust cron lib 格式如下
/// sec   min   hour   day of month   month   day of week   year
/// *     *     *      *              *       *             *
/// react-js-cron 格式如下
///       min   hour   day of month   month   day of week
///       3     4      4              5       3
/// 将前端存储的cron格式进行转化
/// ```
pub fn format_cron(cron: String) -> String {
    format!("0 {} *", cron)
}

/// 根据特定字符串获取当地时间日期，支持加减法计算
/// 例如："now+1d-24h-60m+60s.%Y-%m-%d %H:%M:%S"
///
///  1. now必须指定
///  2. 1d表示1天，1h表示1小时，1m表示1分钟，1s表示1秒
///  3. `.`前面是日期计算字符串，`.`后面是格式化字符串，参考：`<https://docs.rs/chrono/latest/chrono/format/strftime/index.html>`
///
///     `%Y-%m-%d %H:%M:%S`输出"2024-01-31 15:12:48"格式的日期
///
/// ```
///     use crate::process_web::utils::*;
///
///     let time_string = get_datetime_by_string(&r#"now+1d-24h-60m+60s.%Y-%m-%d %H:%M:%S"#.to_string());
///
///     println!("time_string {:?}", time_string);
/// ```
pub fn get_datetime_by_string(value_str: &str) -> anyhow::Result<String> {
    let split_str: Vec<&str> = value_str.split('.').collect();

    if split_str.len() > 0 {
        let date_str = split_str[0];

        if !date_str.contains("now") {
            return Err(anyhow!("请指定now"));
        }

        let mut date = Local::now();
        let mut last_i = 0;
        let mut pre_str = "";
        let mut current_sign = "";

        for i in 0..date_str.len() {
            let char = &date_str[i..i + 1];
            if char == "-" || char == "+" {
                if pre_str == "" {
                    pre_str = &date_str[last_i..i];
                    current_sign = char;
                    last_i = i + 1;
                    continue;
                }

                pre_str = &date_str[last_i..i];
                if current_sign == "-" {
                    date = date - get_date(pre_str)?;
                } else if current_sign == "+" {
                    date = date + get_date(pre_str)?;
                }

                last_i = i + 1;
                current_sign = char;
            }
        }

        pre_str = &date_str[last_i..];
        if current_sign == "-" {
            date = date - get_date(pre_str)?;
        } else if current_sign == "+" {
            date = date + get_date(pre_str)?;
        }

        if split_str.len() > 1 {
            let format_str = split_str[1];
            return Ok(date.naive_local().format(format_str).to_string());
        }
        return Ok(date.naive_local().to_string());
    }

    Err(anyhow!("无法解析字符串"))
}

pub fn get_date(str: &str) -> anyhow::Result<chrono::Duration> {
    let number = str[..str.len() - 1].parse::<i64>()?;
    if str.contains("d") {
        return Ok(chrono::Duration::days(number));
    }
    if str.contains("h") {
        return Ok(chrono::Duration::hours(number));
    }
    if str.contains("m") {
        return Ok(chrono::Duration::minutes(number));
    }
    if str.contains("s") {
        return Ok(chrono::Duration::seconds(number));
    }
    Err(anyhow!("未发现匹配的字符"))
}

/// 查找body字符串中`${xxx}`格式值进行转换
/// 目前只支持日期字符串
pub fn format_body_string(body: Option<&String>) -> Option<String> {
    if body.is_none() {
        return None;
    }

    let mut body_str = body.unwrap().as_str();

    let mut value_map = HashMap::new();

    while let Some(i) = body_str.find("${") {
        body_str = &body_str[i..];

        if let Some(j) = body_str.find("}") {
            let params_str = &body_str[2..j];
            let current_str = &body_str[0..j + 1];
            if params_str.contains("_now") {
                let date = get_datetime_by_string(params_str).unwrap_or("".to_string());
                value_map.insert(date, current_str);
            }
            //Tips 含_loop_counts的需要在循环请求中去获取参数，不在此处做处理

            body_str = &body_str[j..];
        } else {
            break;
        }
    }

    let mut new_str = body.unwrap().clone();

    for (key, value) in value_map {
        new_str = new_str.replace(value, key.as_str());
    }

    Some(new_str)
}

pub fn job_err_to_db_err(err: JobSchedulerError) -> DbErr {
    let s = format!("{}", err);

    DbErr::Custom(s.to_owned())
}
