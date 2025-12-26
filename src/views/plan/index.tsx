import { Button } from '@heroui/react';
import { Task } from '@/models/task';
import { useNavigate } from 'react-router-dom';
import React, { useState } from 'react';
import { getTaskPageList } from '@/apis/task';

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
  const [data, setData] = useState<Array<Task>>([]);

  const navigate = useNavigate();

  function handleNewTask() {
    navigate('/plan/new');
  }

  React.useEffect(() => {
    async function _getData() {
      const res = await getTaskPageList({
        page_size: 50,
        page: 0,
      });

      if (res.code == 200) {
        setData(res.data.data);
      }
    }

    _getData();
  }, []);

  return (
    <div>
      <div className='p-4'>
        {data.map((item) => {
          return <TaskItem data={item} key={item.id} />;
        })}
      </div>

      <Button onClick={handleNewTask}>新增</Button>
    </div>
  );
}
