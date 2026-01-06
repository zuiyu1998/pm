/**
 * 任务,记录一个待办事项
 */

export type Task = {
  id: number;
  title: string;
  completed: boolean;
  created_at: string;
  finish_at?: string;
};

export type TaskCreate = {
  title: string;
};

export type TaskUpdate = {
  id: number;
  title: string;
  completed?: boolean;
};

export type TaskPageParams = {
  page: number;
  page_size: number;
};
