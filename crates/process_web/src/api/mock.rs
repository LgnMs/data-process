use crate::api::common::{AppError, AppState, PaginationPayload};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde_json::Value;
use std::sync::Arc;
use tokio::time::{sleep, Duration};
use tracing::debug;

pub fn set_routes() -> Router<Arc<AppState>> {
    let routes = Router::new()
        .route("/test_data_1", get(test_data_1))
        .route("/test_data_2", post(test_data_2))
        .route("/test_data_3", get(test_data_3));

    routes
}

pub async fn test_data_1() -> anyhow::Result<Json<Value>, AppError> {
    let data = serde_json::json!({"code":"SUCCESS","data":{"result":[{"pkid":"1a4e8cb2b85ed5035ad9844e49f3b9b5f4d6b172","citycode":"511100","cityname":"乐山市","monitor_time":"2023-03-01 00:00:00","aqi":"65","aqi_state_code":"2","aqi_level":"Ⅱ","aqi_class_name":"良","aqi_state_name":"二级","aqi_color_name":"黄色","avg_o3_degree":"80","avg_pm25_degree":"47","avg_co_degree":"0.6","avg_no2_degree":"31","avg_so2_degree":"8","avg_pm10_degree":"77","main_polls":"PM2.5","updatetime":"2023-03-02 22:24:36","cnt":"15"},{"pkid":"303dc5c6d4afadea6983b1db14f0886f717ee66f","citycode":"511100","cityname":"乐山市","monitor_time":"2023-03-02 00:00:00","aqi":"69","aqi_state_code":"2","aqi_level":"Ⅱ","aqi_class_name":"良","aqi_state_name":"二级","aqi_color_name":"黄色","avg_o3_degree":"58","avg_pm25_degree":"48","avg_co_degree":"0.8","avg_no2_degree":"36","avg_so2_degree":"7","avg_pm10_degree":"87","main_polls":"PM10","updatetime":"2023-03-03 22:25:30","cnt":"15"},{"pkid":"c4f51c56be549f055b885237c68dec6381b82b6b","citycode":"511100","cityname":"乐山市","monitor_time":"2023-03-03 00:00:00","aqi":"59","aqi_state_code":"2","aqi_level":"Ⅱ","aqi_class_name":"良","aqi_state_name":"二级","aqi_color_name":"黄色","avg_o3_degree":"106","avg_pm25_degree":"42","avg_co_degree":"0.7","avg_no2_degree":"29","avg_so2_degree":"6","avg_pm10_degree":"62","main_polls":"PM2.5","updatetime":"2023-03-04 22:24:35","cnt":"15"},{"pkid":"cdcb64c3d087f7307c372bdac1fcb1808f1a035d","citycode":"511100","cityname":"乐山市","monitor_time":"2023-03-04 00:00:00","aqi":"72","aqi_state_code":"2","aqi_level":"Ⅱ","aqi_class_name":"良","aqi_state_name":"二级","aqi_color_name":"黄色","avg_o3_degree":"115","avg_pm25_degree":"52","avg_co_degree":"0.8","avg_no2_degree":"37","avg_so2_degree":"8","avg_pm10_degree":"76","main_polls":"PM2.5","updatetime":"2023-03-05 22:24:57","cnt":"15"},{"pkid":"26b7189abac1a6bfda179b79326955a280035993","citycode":"511100","cityname":"乐山市","monitor_time":"2023-03-05 00:00:00","aqi":"71","aqi_state_code":"2","aqi_level":"Ⅱ","aqi_class_name":"良","aqi_state_name":"二级","aqi_color_name":"黄色","avg_o3_degree":"125","avg_pm25_degree":"50","avg_co_degree":"0.8","avg_no2_degree":"28","avg_so2_degree":"7","avg_pm10_degree":"72","main_polls":"O3","updatetime":"2023-03-06 22:25:03","cnt":"15"},{"pkid":"527198eece4be90f275737a6ad892a8481ded702","citycode":"511100","cityname":"乐山市","monitor_time":"2023-03-06 00:00:00","aqi":"87","aqi_state_code":"2","aqi_level":"Ⅱ","aqi_class_name":"良","aqi_state_name":"二级","aqi_color_name":"黄色","avg_o3_degree":"144","avg_pm25_degree":"57","avg_co_degree":"0.8","avg_no2_degree":"31","avg_so2_degree":"8","avg_pm10_degree":"80","main_polls":"O3","updatetime":"2023-03-07 22:24:32","cnt":"15"},{"pkid":"b022b6fbbd4573bf3c42e0a57d98dd83112f41e1","citycode":"511100","cityname":"乐山市","monitor_time":"2023-03-07 00:00:00","aqi":"83","aqi_state_code":"2","aqi_level":"Ⅱ","aqi_class_name":"良","aqi_state_name":"二级","aqi_color_name":"黄色","avg_o3_degree":"94","avg_pm25_degree":"61","avg_co_degree":"0.8","avg_no2_degree":"24","avg_so2_degree":"5","avg_pm10_degree":"80","main_polls":"PM2.5","updatetime":"2023-03-08 22:25:23","cnt":"15"},{"pkid":"9ae827a173591db2f57c478d4286caaeab7f174d","citycode":"511100","cityname":"乐山市","monitor_time":"2023-03-08 00:00:00","aqi":"69","aqi_state_code":"2","aqi_level":"Ⅱ","aqi_class_name":"良","aqi_state_name":"二级","aqi_color_name":"黄色","avg_o3_degree":"122","avg_pm25_degree":"42","avg_co_degree":"0.9","avg_no2_degree":"25","avg_so2_degree":"6","avg_pm10_degree":"60","main_polls":"O3","updatetime":"2023-03-09 22:24:32","cnt":"15"},{"pkid":"0cf876325fe1923cb55ffe9176167918df23e15b","citycode":"511100","cityname":"乐山市","monitor_time":"2023-03-09 00:00:00","aqi":"54","aqi_state_code":"2","aqi_level":"Ⅱ","aqi_class_name":"良","aqi_state_name":"二级","aqi_color_name":"黄色","avg_o3_degree":"88","avg_pm25_degree":"38","avg_co_degree":"0.8","avg_no2_degree":"39","avg_so2_degree":"10","avg_pm10_degree":"53","main_polls":"PM2.5","updatetime":"2023-03-10 22:24:38","cnt":"15"},{"pkid":"ef44b1a75b62f832e3ac82d8feaf382e2a53b384","citycode":"511100","cityname":"乐山市","monitor_time":"2023-03-10 00:00:00","aqi":"67","aqi_state_code":"2","aqi_level":"Ⅱ","aqi_class_name":"良","aqi_state_name":"二级","aqi_color_name":"黄色","avg_o3_degree":"120","avg_pm25_degree":"39","avg_co_degree":"0.7","avg_no2_degree":"30","avg_so2_degree":"10","avg_pm10_degree":"56","main_polls":"O3","updatetime":"2023-03-11 22:25:10","cnt":"15"},{"pkid":"1eee72fd760602534fd0aedd5709407b4ed6f4ec","citycode":"511100","cityname":"乐山市","monitor_time":"2023-03-11 00:00:00","aqi":"74","aqi_state_code":"2","aqi_level":"Ⅱ","aqi_class_name":"良","aqi_state_name":"二级","aqi_color_name":"黄色","avg_o3_degree":"128","avg_pm25_degree":"41","avg_co_degree":"0.6","avg_no2_degree":"32","avg_so2_degree":"10","avg_pm10_degree":"62","main_polls":"O3","updatetime":"2023-03-12 21:24:25","cnt":"15"},{"pkid":"2f1c0c30f5f6a9507b3c8fa8f0b65832bd01703d","citycode":"511100","cityname":"乐山市","monitor_time":"2023-03-12 00:00:00","aqi":"40","aqi_state_code":"1","aqi_level":"Ⅰ","aqi_class_name":"优","aqi_state_name":"一级","aqi_color_name":"绿色","avg_o3_degree":"80","avg_pm25_degree":"11","avg_co_degree":"0.4","avg_no2_degree":"15","avg_so2_degree":"5","avg_pm10_degree":"34","main_polls":"—","updatetime":"2023-03-13 21:25:01","cnt":"15"},{"pkid":"16b86ee17e6f1a3e0579d1129c3e8ba7d3d5779a","citycode":"511100","cityname":"乐山市","monitor_time":"2023-03-13 00:00:00","aqi":"46","aqi_state_code":"1","aqi_level":"Ⅰ","aqi_class_name":"优","aqi_state_name":"一级","aqi_color_name":"绿色","avg_o3_degree":"92","avg_pm25_degree":"17","avg_co_degree":"0.4","avg_no2_degree":"19","avg_so2_degree":"6","avg_pm10_degree":"36","main_polls":"—","updatetime":"2023-03-14 21:24:18","cnt":"15"},{"pkid":"b7f86149b2b309a3d81d166cda1552c64601d324","citycode":"511100","cityname":"乐山市","monitor_time":"2023-03-14 00:00:00","aqi":"55","aqi_state_code":"2","aqi_level":"Ⅱ","aqi_class_name":"良","aqi_state_name":"二级","aqi_color_name":"黄色","avg_o3_degree":"104","avg_pm25_degree":"37","avg_co_degree":"0.6","avg_no2_degree":"34","avg_so2_degree":"10","avg_pm10_degree":"60","main_polls":"PM10","updatetime":"2023-03-15 21:25:02","cnt":"15"},{"pkid":"dff3eddf3872c07616bebdeb13c1b6429ee31b78","citycode":"511100","cityname":"乐山市","monitor_time":"2023-03-15 00:00:00","aqi":"74","aqi_state_code":"2","aqi_level":"Ⅱ","aqi_class_name":"良","aqi_state_name":"二级","aqi_color_name":"黄色","avg_o3_degree":"101","avg_pm25_degree":"54","avg_co_degree":"0.7","avg_no2_degree":"34","avg_so2_degree":"12","avg_pm10_degree":"84","main_polls":"PM2.5","updatetime":"2023-09-20 03:49:39","cnt":"15"}]},"message":"","traceId":""});
    debug!("mock test_data_1 start sleep 5000ms!");
    sleep(Duration::from_millis(1000)).await;
    debug!("mock test_data_1 start sleep done!");
    Ok(Json(data))
}

pub async fn test_data_2(
    Json(payload): Json<PaginationPayload<bool>>,
) -> anyhow::Result<Json<Value>, AppError> {
    let data1 = serde_json::json!({"code":"SUCCESS","data":{"result":[{"pkid":"1a4e8cb2b85ed5035ad9844e49f3b9b5f4d6b172","citycode":"511100","cityname":"乐山市","monitor_time":"2023-03-01 00:00:00","aqi":"65","aqi_state_code":"2","aqi_level":"Ⅱ","aqi_class_name":"良","aqi_state_name":"二级","aqi_color_name":"黄色","avg_o3_degree":"80","avg_pm25_degree":"47","avg_co_degree":"0.6","avg_no2_degree":"31","avg_so2_degree":"8","avg_pm10_degree":"77","main_polls":"PM2.5","updatetime":"2023-03-02 22:24:36","cnt":"15"}]},"message":"","traceId":""});
    let data2 = serde_json::json!({"code":"SUCCESS","data":{"result":[{"pkid":"303dc5c6d4afadea6983b1db14f0886f717ee66f","citycode":"511100","cityname":"乐山市","monitor_time":"2023-03-02 00:00:00","aqi":"69","aqi_state_code":"2","aqi_level":"Ⅱ","aqi_class_name":"良","aqi_state_name":"二级","aqi_color_name":"黄色","avg_o3_degree":"58","avg_pm25_degree":"48","avg_co_degree":"0.8","avg_no2_degree":"36","avg_so2_degree":"7","avg_pm10_degree":"87","main_polls":"PM10","updatetime":"2023-03-03 22:25:30","cnt":"15"}]},"message":"","traceId":""});
    let data3 = serde_json::json!({"code":"SUCCESS","data":{"result":[{"pkid":"c4f51c56be549f055b885237c68dec6381b82b6b","citycode":"511100","cityname":"乐山市","monitor_time":"2023-03-03 00:00:00","aqi":"59","aqi_state_code":"2","aqi_level":"Ⅱ","aqi_class_name":"良","aqi_state_name":"二级","aqi_color_name":"黄色","avg_o3_degree":"106","avg_pm25_degree":"42","avg_co_degree":"0.7","avg_no2_degree":"29","avg_so2_degree":"6","avg_pm10_degree":"62","main_polls":"PM2.5","updatetime":"2023-03-04 22:24:35","cnt":"15"}]},"message":"","traceId":""});

    debug!("mock test_data_2 start sleep 5000ms!");
    sleep(Duration::from_millis(1000)).await;
    debug!("mock test_data_2 start sleep done!");

    if payload.current == 1 {
        return Ok(Json(data1));
    }
    if payload.current == 2 {
        return Ok(Json(data2));
    }
    if payload.current == 3 {
        return Ok(Json(data3));
    }
    let null = serde_json::json!({"code":"SUCCESS","data":{},"message":"","traceId":""});

    return Ok(Json(null));
}

pub async fn test_data_3() -> anyhow::Result<Json<Value>, AppError> {
    let data = serde_json::json!({
        "code": "00000",
        "message": "OK",
        "success": true,
        "result": {
            "pageIndex": 1,
            "pageSize": 999,
            "totalPage": 1,
            "totalRecord": 1,
            "domains": [{
                "code": "wireless",
                "data": [{
                    "ciId": "000000000002bdd3",
                    "typeName": "AC",
                    "metricList": [{
                        "code": "delayed_ping",
                        "name": "ping时延",
                        "value": "1.5905349999999998",
                        "unit": "ms"
                    }, {
                        "code": "memory_utilization",
                        "name": "内存利用率",
                        "value": "62",
                        "unit": "%"
                    }, {
                        "code": "ping_status",
                        "name": "ping状态",
                        "value": "正常",
                        "unit": ""
                    }, {
                        "code": "online_user_saturation",
                        "name": "在线用户饱和度",
                        "value": "0",
                        "unit": "%"
                    }, {
                        "code": "online_ap_saturation",
                        "name": "在线AP饱和度",
                        "value": "0",
                        "unit": "%"
                    }, {
                        "code": "cpu_utilization",
                        "name": "CPU利用率",
                        "value": "8",
                        "unit": "%"
                    }, {
                        "code": "online_ap_number",
                        "name": "在线AP数",
                        "value": "0",
                        "unit": "个"
                    }, {
                        "code": "online_user_number",
                        "name": "在线用户数",
                        "value": "0",
                        "unit": "个"
                    }, {
                        "code": "software_version_number",
                        "name": "软件版本号",
                        "value": "AC_RGOS 11.1(5)B80P3, Release(04131821)",
                        "unit": ""
                    }, {
                        "code": "snmp_reachable",
                        "name": "snmp状态",
                        "value": "正常",
                        "unit": ""
                    }, {
                        "code": "offline_ap_number",
                        "name": "离线AP数",
                        "value": "19",
                        "unit": "个"
                    }, {
                        "code": "system_up_time",
                        "name": "运行时长",
                        "value": "1586968.46",
                        "unit": "s"
                    }],
                    "components": [{
                        "ciId": "000000000002be17",
                        "typeName": "WLAN",
                        "metricList": [{
                            "code": "online_user_number",
                            "name": "在线用户数",
                            "value": "0",
                            "unit": "个"
                        }]
                    }, {
                        "ciId": "000000000002be0e",
                        "typeName": "网络接口",
                        "metricList": [{
                            "code": "interface_in_errors_percent",
                            "name": "接收错包率",
                            "value": "0",
                            "unit": "%"
                        }, {
                            "code": "administer_status",
                            "name": "管理状态",
                            "value": "启用",
                            "unit": ""
                        }, {
                            "code": "total_flow",
                            "name": "发送接收总流量",
                            "value": "198071",
                            "unit": "Bytes"
                        }, {
                            "code": "operate_status",
                            "name": "操作状态",
                            "value": "正常",
                            "unit": ""
                        }, {
                            "code": "in_drop_packets",
                            "name": "接收丢包数",
                            "value": "0",
                            "unit": "个"
                        }, {
                            "code": "total_errors_packets",
                            "name": "总错包数",
                            "value": "0",
                            "unit": "个"
                        }, {
                            "code": "bandwidth_utilization",
                            "name": "带宽利用率",
                            "value": "0",
                            "unit": "%"
                        }, {
                            "code": "in_packets",
                            "name": "接收包数",
                            "value": "1785",
                            "unit": "个"
                        }, {
                            "code": "in_error_packets",
                            "name": "接收错包数",
                            "value": "0",
                            "unit": "个"
                        }, {
                            "code": "out_drop_packets",
                            "name": "发送丢包数",
                            "value": "0",
                            "unit": "个"
                        }, {
                            "code": "out_error_packets",
                            "name": "发送错包数",
                            "value": "0",
                            "unit": "个"
                        }, {
                            "code": "out_packets",
                            "name": "发送包数",
                            "value": "230",
                            "unit": "个"
                        }, {
                            "code": "in_bandwidth_utilization",
                            "name": "接收带宽利用率",
                            "value": "0",
                            "unit": "%"
                        }, {
                            "code": "out_rate",
                            "name": "发送速率",
                            "value": "1185.47",
                            "unit": "bps"
                        },{
                            "code": "output_bytes_ipv4",
                            "name": "IPv4发送流量",
                            "value": "44307",
                            "unit": "Bytes"
                        }]
                    }]
                }]
            }]
        }
    });

    // result.domains#data#ciId             data#1_ciId
    // result.domains#data#metricList#name  data#1_name
    // result.domains#data#components#ciId  data#2_ciId
    // result.domains#data#components#metricList#name  data#2_name
    debug!("mock test_data_1 start sleep 5000ms!");
    sleep(Duration::from_millis(1000)).await;
    debug!("mock test_data_1 start sleep done!");
    Ok(Json(data))
}
