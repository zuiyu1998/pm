import { Button } from '@heroui/react';
import { Task } from '@/models/task';
import { useNavigate } from 'react-router-dom';

export type TaskData = Task & {};

export type TaskItemProps = {
  data: TaskData;
};

function TaskItem(props: TaskItemProps) {
  const { data } = props;

  return (
    <div className='p-4 border-2 border-gray-300 rounded'>
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

  const navigate = useNavigate();

  function handleNewTask() {
    navigate('/plan/new');
  }

  return (
    <div>
      <Button onClick={handleNewTask}>New Task</Button>
      <div className='p-4'>
        <TaskItem data={data} />
      </div>
    </div>
  );
}
