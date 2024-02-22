import { SyncLog } from "@/api/models/SyncLog";
import {
  http_get,
  http_post,
  PaginationPayload,
  ResJson,
  ResJsonWithPagination,
} from "@/api/common";
import { SyncLogListParams } from "@/api/models/SyncLogListParams";

export const PREFIX = "/sync_log";

export const FIND_BY_ID = `${PREFIX}/find_by_id/`;

export async function find_by_id(id: number): Promise<ResJson<SyncLog>> {
  return http_get(`${FIND_BY_ID}${id}`);
}

export const LIST = `${PREFIX}/list/`;
export async function list(
  payload: PaginationPayload<SyncLogListParams>
): Promise<ResJsonWithPagination<SyncLog>> {
  return http_post(`${PREFIX}/list`, {
    body: JSON.stringify(payload),
  });
}

export const ADD = `${PREFIX}/add/`;
export async function add(payload: SyncLog): Promise<ResJson<SyncLog>> {
  console.log(payload);
  return http_post(`${PREFIX}/add`, {
    body: JSON.stringify(payload),
  });
}

export const UPDATE_BY_ID = `${PREFIX}/update_by_id/`;
export async function update_by_id(
  id: string,
  payload: SyncLog
): Promise<ResJson<SyncLog>> {
  return http_post(`${PREFIX}/update_by_id/${id}`, {
    body: JSON.stringify(payload),
  });
}

export const DEL = `${PREFIX}/del/`;
export async function del(id: string): Promise<ResJson<boolean>> {
  return http_get(`${PREFIX}/del/${id}`);
}
