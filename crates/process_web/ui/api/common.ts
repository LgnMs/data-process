import {message} from 'antd'

export interface ResTemplate<T> {
  message: string;
  data: T | null;
  success: boolean;
}
export interface Pagination<T> {
  total: number;
  list: T;
  current: number;
  page_size: number;
}

export type ResJson<T> = ResTemplate<T>;

export type ResJsonWithPagination<T> = ResJson<Pagination<Array<T>>>;

export interface PaginationPayload<T> {
  current: number;
  page_size: number;
  data: T | null;
}

export async function http_get<T>(input: string): Promise<T> {
  const res = await fetch(input);

  let data = await res.json();
  if (!data.success) {
    message.error(data.message);
  }
  return new Promise((resolve) => resolve(data));
}

export async function http_post<T>(
  input: string,
  init: RequestInit
): Promise<T> {
  const headers = new Headers();
  headers.append("Content-Type", "application/json");

  const res = await fetch(input, {
    headers,
    method: "POST",
    ...init,
  });

  let data = await res.json();
  if (!data.success) {
    message.error(data.message);
  }
  return new Promise((resolve) => resolve(data));
}
