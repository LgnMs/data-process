export interface ResTemplate<T> {
  message: string;
  data: T | null;
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

  return res.json();
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
  return res.json();
}
