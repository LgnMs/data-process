import { message } from "antd";

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

async function handler_err(res: Response) {
  if (res.status === 400) {
    message.error(`status: 400 ${res.statusText} 没有认证信息或认证信息已过期`);
    return null
  }
  let data = await res.json();
  if (!data.success) {
    message.error(data.message);
  }

  return data
}

export async function http_get<T>(input: string): Promise<T> {
  const headers = new Headers();
  if (sessionStorage.getItem("Authorization")) {
    headers.append("Authorization", sessionStorage.getItem("Authorization")!);
  }

  const res = await fetch(input, { headers });

  const data = await handler_err(res);

  return new Promise((resolve) => resolve(data));
}

export async function http_post<T>(
  input: string,
  init: RequestInit
): Promise<T> {
  const headers = new Headers();
  headers.append("Content-Type", "application/json");
  if (sessionStorage.getItem("Authorization")) {
    headers.append("Authorization", sessionStorage.getItem("Authorization")!);
  }
  const res = await fetch(input, {
    headers,
    method: "POST",
    ...init,
  });

  const data = await handler_err(res);

  return new Promise((resolve) => resolve(data));
}
