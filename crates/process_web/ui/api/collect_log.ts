import { CollectLog } from "@/api/models/CollectLog";
import {
  http_get,
  http_post,
  PaginationPayload,
  ResJson,
  ResJsonWithPagination,
} from "@/api/common";

export const PREFIX = "/collect_log";

export const FIND_BY_ID = `${PREFIX}/find_by_id/`;

export async function find_by_id(id: number): Promise<ResJson<CollectLog>> {
  return http_get(`${FIND_BY_ID}${id}`);
}

export const LIST = `${PREFIX}/list/`;
export async function list(
  payload: PaginationPayload<CollectLog>
): Promise<ResJsonWithPagination<CollectLog>> {
  return http_post(`${PREFIX}/list`, {
    body: JSON.stringify(payload),
  });
}

export const ADD = `${PREFIX}/add/`;
export async function add(
  payload: CollectLog
): Promise<ResJson<CollectLog>> {
  console.log(payload);
  return http_post(`${PREFIX}/add`, {
    body: JSON.stringify(payload),
  });
}

export const UPDATE_BY_ID = `${PREFIX}/update_by_id/`;
export async function update_by_id(
  id: string,
  payload: CollectLog
): Promise<ResJson<CollectLog>> {
  return http_post(`${PREFIX}/update_by_id/${id}`, {
    body: JSON.stringify(payload),
  });
}

export const DEL = `${PREFIX}/del/`;
export async function del(id: string): Promise<ResJson<boolean>> {
  return http_get(`${PREFIX}/del/${id}`);
}

export const EXECUTE = `${PREFIX}/execute/`;
export async function execute(id: string): Promise<ResJson<string[]>> {
  return http_get(`${PREFIX}/execute/${id}`);
}
