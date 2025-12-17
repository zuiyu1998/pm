import { Task, TaskCreate } from '../models/task';
import { invoke } from '@tauri-apps/api/core';

export type Response<T> = {
  data: T;
  code: number;
};

export async function createTask(create: TaskCreate): Promise<Response<Task>> {
  return await invoke('create_task', { create });
}
