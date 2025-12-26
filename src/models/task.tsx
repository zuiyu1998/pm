/**
 * 任务,记录一个待办事项
 */

export type Task = {
  id: string;
  title: string;
  completed: boolean;
  create_at: string;
  finish_at?: string;
};

export type TaskCreate = {
  title: string;
};

export type TaskPageParams = {
  page: number;
  page_size: number;
};
