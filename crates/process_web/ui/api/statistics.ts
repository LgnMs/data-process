import { http_get, http_post, ResJson } from "@/api/common";
import { CollectTaskInfoDayListReq } from "./models/CollectTaskInfoDayListReq";
import { CollectTaskInfo } from "./models/CollectTaskInfo";
import { CollectTaskInfoRes } from "./models/CollectTaskInfoRes";
import { SharingTaskInfoReq } from "./models/SharingTaskInfoReq";
import { SharingTaskInfoRes } from "./models/SharingTaskInfoRes";
import { SyncTaskInfoReq } from "./models/SyncTaskInfoReq";
import { SyncTaskInfoRes } from "./models/SyncTaskInfoRes";
import { SystemInfo } from "./models/SystemInfo";

export const PREFIX = "/api/statistics";

export const COLLECT_TASK_INFO = `${PREFIX}/collect_task_info`;
export async function collect_task_info(): Promise<ResJson<CollectTaskInfo>> {
  return http_get(`${COLLECT_TASK_INFO}`);
}

export const GET_SYS_INFO = `${PREFIX}/get_sys_info`;
export async function get_sys_info(): Promise<ResJson<SystemInfo>> {
  return http_get(`${GET_SYS_INFO}`);
}

export const COLLECT_TASK_INFO_DAY_LIST = `${PREFIX}/collect_task_info_day_list`;
export async function collect_task_info_day_list(
  payload: CollectTaskInfoDayListReq
): Promise<ResJson<CollectTaskInfoRes>> {
  return http_post(`${COLLECT_TASK_INFO_DAY_LIST}`, {
    body: JSON.stringify(payload),
  });
}

export const SHARING_TASK_INFO = `${PREFIX}/sharing_task_info`;
export async function sharing_task_info(
  payload: SharingTaskInfoReq
): Promise<ResJson<SharingTaskInfoRes>> {
  return http_post(`${SHARING_TASK_INFO}`, {
    body: JSON.stringify(payload),
  });
}

export const SYNC_TASK_INFO = `${PREFIX}/sync_task_info`;
export async function sync_task_info(
  payload: SyncTaskInfoReq
): Promise<ResJson<SyncTaskInfoRes>> {
  return http_post(`${SYNC_TASK_INFO}`, {
    body: JSON.stringify(payload),
  });
}
