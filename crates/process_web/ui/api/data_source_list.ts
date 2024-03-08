import { DataSourceList } from "@/api/models/DataSourceList";
import {
  http_get,
  http_post,
  PaginationPayload,
  ResJson,
  ResJsonWithPagination,
} from "@/api/common";
import { DataSourceListParams } from "@/api/models/DataSourceListParams";
import { QueryTableColumnsParameters } from "@/api/models/QueryTableColumnsParameters";

export const PREFIX = "/api/data_source_list";

export const FIND_BY_ID = `${PREFIX}/find_by_id/`;

export async function find_by_id(id: number): Promise<ResJson<DataSourceList>> {
  return http_get(`${FIND_BY_ID}${id}`);
}

export const LIST = `${PREFIX}/list/`;
export async function list(
  payload: PaginationPayload<DataSourceListParams>
): Promise<ResJsonWithPagination<DataSourceList>> {
  return http_post(`${PREFIX}/list`, {
    body: JSON.stringify(payload),
  });
}

export const ADD = `${PREFIX}/add/`;
export async function add(
  payload: DataSourceList
): Promise<ResJson<DataSourceList>> {
  console.log(payload);
  return http_post(`${PREFIX}/add`, {
    body: JSON.stringify(payload),
  });
}

export const UPDATE_BY_ID = `${PREFIX}/update_by_id/`;
export async function update_by_id(
  id: number,
  payload: DataSourceList
): Promise<ResJson<DataSourceList>> {
  return http_post(`${PREFIX}/update_by_id/${id}`, {
    body: JSON.stringify(payload),
  });
}

export const DEL = `${PREFIX}/del/`;
export async function del(id: number): Promise<ResJson<boolean>> {
  return http_get(`${PREFIX}/del/${id}`);
}

export const QUERY_TABLE_COLUMNS = `${PREFIX}/query_table_columns/`;
export async function query_table_columns(
  payload: QueryTableColumnsParameters
): Promise<ResJson<Record<string, string>[]>> {
  return http_post(`${PREFIX}/query_table_columns`, {
    body: JSON.stringify(payload),
  });
}
