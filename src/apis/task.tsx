import { Task, TaskCreate, TaskPageParams, TaskUpdate } from '@/models/task';
import { invoke } from '@tauri-apps/api/core';

export type Response<T> = {
  data: T;
  code: number;
  msg?: string;
};

export type PageResponse<T> = {
  has_next: boolean;
  page: number;
  page_sizee: number;
  data: Array<T>;
  total: number;
};

export async function updateTask(update: TaskUpdate): Promise<Response<Task>> {
  return await invoke('update_task', { update });
}

export async function createTask(create: TaskCreate): Promise<Response<Task>> {
  return await invoke('create_task', { create });
}

export async function getTaskPageList(
  params: TaskPageParams
): Promise<Response<PageResponse<Task>>> {
  return await invoke('get_task_page_list', { params });
}
