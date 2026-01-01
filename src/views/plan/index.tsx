import { Task } from '@/models/task';
import React, { useState } from 'react';
import { getTaskPageList } from '@/apis/task';
import { Card, Checkbox, Button } from '@heroui/react';
import { VscTrash } from 'react-icons/vsc';
export type TaskData = Task & {};

export type TaskItemProps = {
  data: TaskData;
};

function TaskItem(props: TaskItemProps) {
  const { data } = props;

  return (
    <Card className='flex flex-row justify-between'>
      <Checkbox isSelected={data.completed}>
        <Checkbox.Control>
          <Checkbox.Indicator />
        </Checkbox.Control>
      </Checkbox>

      <div className='flex-1'>
        <div className='text-lg'>{data.title}</div>
        <div className='text-xs text-muted'>{data.create_at}</div>
      </div>
      <div>
        <Button isIconOnly variant='danger'>
          <VscTrash />
        </Button>
      </div>
    </Card>
  );
}

export function Plan() {
  const [data, setData] = useState<Array<Task>>([]);

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
      <div className='p-4 gap-4 flex flex-col'>
        {data.map((item) => {
          return <TaskItem data={item} key={item.id} />;
        })}
      </div>
    </div>
  );
}
