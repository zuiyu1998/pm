import { Task } from '../models/task';

export type TaskData = Task & {};

export type TaskItemProps = {
  data: TaskData;
};

function TaskItem(props: TaskItemProps) {
  const { data } = props;

  return (
    <div className='p-4 border-2 rounded'>
      <div className='text-lg'>{data.title}</div>
    </div>
  );
}

export function Plan() {
  const data = {
    id: '1',
    title: 'Sample Task',
    completed: false,
    create_at: new Date().toISOString(),
  };

  return (
    <div>
      <div className='p-4'>
        <TaskItem data={data} />
      </div>
    </div>
  );
}
