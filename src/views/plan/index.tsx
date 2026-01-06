import { Task } from '@/models/task';
import React, { useState } from 'react';
import {
  createTask,
  getTaskPageList,
  updateTask,
  deleteTask,
} from '@/apis/task';
import { Card, Checkbox, Button, Input, Spinner } from '@heroui/react';
import { VscTrash, VscHistory, VscDebugContinueSmall } from 'react-icons/vsc';
import { datetime } from '@/utils/datatime';
import { useNavigate } from 'react-router-dom';
export type TaskData = Task & {};

export type TaskItemProps = {
  data: TaskData;
  refresh: () => void;
};

function TaskItem(props: TaskItemProps) {
  const { data, refresh } = props;
  const navigate = useNavigate();

  const [loading, setLoading] = React.useState(false);

  function _gotoWork() {
    navigate('/plan/work');
  }

  async function _handleStateChange() {
    try {
      if (loading) {
        return;
      }
      setLoading(true);
      await updateTask({
        id: data.id,
        title: data.title,
        completed: !data.completed,
      });

      refresh();
    } catch (error) {
    } finally {
      setLoading(false);
    }
  }

  async function _handleDelete() {
    try {
      if (loading) {
        return;
      }
      setLoading(true);
      await deleteTask(data.id);

      refresh();
    } catch (error) {
    } finally {
      setLoading(false);
    }
  }

  return (
    <Card className='flex flex-row justify-between'>
      <Checkbox isSelected={data.completed} onChange={_handleStateChange}>
        <Checkbox.Control>
          <Checkbox.Indicator />
        </Checkbox.Control>
      </Checkbox>

      <div className='flex-1'>
        <div className='text-lg'>{data.title}</div>
        <div className=' text-muted pt-1'>
          <div className='flex flex-row items-center'>
            <VscHistory size={14} />
            <div className='pl-1 text-xs'>{datetime(data.created_at)}</div>
          </div>
        </div>
      </div>
      <div className='flex flex-row gap-3'>
        <Button isIconOnly onClick={_gotoWork}>
          <VscDebugContinueSmall />
        </Button>

        <Button isIconOnly variant='danger' onClick={_handleDelete}>
          <VscTrash />
        </Button>
      </div>
    </Card>
  );
}

export function Plan() {
  const [data, setData] = useState<Array<Task>>([]);
  const [loading, setLoading] = React.useState(false);

  const [title, setTitle] = useState('');

  async function _createTask() {
    try {
      if (loading) {
        return;
      }
      setLoading(true);

      await createTask({
        title: title,
      });

      await _getData();
    } catch (error) {
    } finally {
      setLoading(false);
    }
  }

  async function _getData() {
    const res = await getTaskPageList({
      page_size: 50,
      page: 0,
    });

    if (res.code == 200) {
      setData(res.data.data);
    }
  }

  React.useEffect(() => {
    _getData();
  }, []);

  return (
    <div>
      <Card className='m-4 flex flex-row'>
        <Input
          className='flex-1'
          placeholder='请输入标题'
          value={title}
          onChange={(v) => setTitle(v.target.value)}
        />
        <Button variant='primary' onClick={_createTask}>
          {({ isPending }) => (
            <>
              {isPending ? <Spinner color='current' size='sm' /> : null}
              确认
            </>
          )}
        </Button>
      </Card>

      <div className='p-4 gap-4 flex flex-col'>
        {data.map((item) => {
          return <TaskItem data={item} key={item.id} refresh={_getData} />;
        })}
      </div>
    </div>
  );
}
