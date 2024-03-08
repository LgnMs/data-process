import { SharingRequestLog } from "@/api/models/SharingRequestLog";
import {
  http_get,
  http_post,
  PaginationPayload,
  ResJson,
  ResJsonWithPagination,
} from "@/api/common";
import { SharingRequestLogParams } from "@/api/models/SharingRequestLogParams";

export const PREFIX = "/api/sharing_request_log";

export const FIND_BY_ID = `${PREFIX}/find_by_id/`;

export async function find_by_id(
  id: number
): Promise<ResJson<SharingRequestLog>> {
  return http_get(`${FIND_BY_ID}${id}`);
}

export const LIST = `${PREFIX}/list/`;
export async function list(
  payload: PaginationPayload<SharingRequestLogParams>
): Promise<ResJsonWithPagination<SharingRequestLog>> {
  return http_post(`${PREFIX}/list`, {
    body: JSON.stringify(payload),
  });
}

export const ADD = `${PREFIX}/add/`;
export async function add(
  payload: SharingRequestLog
): Promise<ResJson<SharingRequestLog>> {
  console.log(payload);
  return http_post(`${PREFIX}/add`, {
    body: JSON.stringify(payload),
  });
}

export const UPDATE_BY_ID = `${PREFIX}/update_by_id/`;
export async function update_by_id(
  id: string,
  payload: SharingRequestLog
): Promise<ResJson<SharingRequestLog>> {
  return http_post(`${PREFIX}/update_by_id/${id}`, {
    body: JSON.stringify(payload),
  });
}

export const DEL = `${PREFIX}/del/`;
export async function del(id: string): Promise<ResJson<boolean>> {
  return http_get(`${PREFIX}/del/${id}`);
}
