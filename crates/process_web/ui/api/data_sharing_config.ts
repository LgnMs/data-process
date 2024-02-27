import {
  http_get,
  http_post,
  PaginationPayload,
  ResJson,
  ResJsonWithPagination,
} from "@/api/common";
import { DataSharingConfigParams } from "@/api/models/DataSharingConfigParams";
import { DataSharingConfig } from "@/api/models/DataSharingConfig";

export const PREFIX = "/data_sharing_config";

export const FIND_BY_ID = `${PREFIX}/find_by_id/`;

export async function find_by_id(
  id: number
): Promise<ResJson<DataSharingConfig>> {
  return http_get(`${FIND_BY_ID}${id}`);
}

export const LIST = `${PREFIX}/list/`;
export async function list(
  payload: PaginationPayload<DataSharingConfigParams>
): Promise<ResJsonWithPagination<DataSharingConfig>> {
  return http_post(`${PREFIX}/list`, {
    body: JSON.stringify(payload),
  });
}

export const ADD = `${PREFIX}/add/`;
export async function add(
  payload: DataSharingConfig
): Promise<ResJson<DataSharingConfig>> {
  console.log(payload);
  return http_post(`${PREFIX}/add`, {
    body: JSON.stringify(payload),
  });
}

export const UPDATE_BY_ID = `${PREFIX}/update_by_id/`;
export async function update_by_id(
  id: number,
  payload: DataSharingConfig
): Promise<ResJson<DataSharingConfig>> {
  return http_post(`${PREFIX}/update_by_id/${id}`, {
    body: JSON.stringify(payload),
  });
}

export const DEL = `${PREFIX}/del/`;
export async function del(id: number): Promise<ResJson<boolean>> {
  return http_get(`${PREFIX}/del/${id}`);
}
