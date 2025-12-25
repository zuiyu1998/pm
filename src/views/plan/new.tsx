import { createTask } from '@/apis/task';
import { Button, Form, Input, Label, TextField, Spinner } from '@heroui/react';
import React from 'react';

export function NewPlan() {
  const [loading, setLoading] = React.useState(false);

  const onSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();

    if (loading) {
      return;
    }

    setLoading(true);

    try {
      const formData = new FormData(e.currentTarget);

      let params = {
        title: formData.get('title') as string,
      };

      const res = await createTask(params);

      if (res.code != 200) {
        //todo 
      }
    } catch (_error) {
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className='m-auto w-100 pt-4'>
      <Form onSubmit={onSubmit} className='flex flex-col gap-4'>
        <TextField name='title' className='flex-row items-center'>
          <Label htmlFor='title' isRequired className='w-25'>
            名称
          </Label>
          <Input required  className='w-full' />
        </TextField>

        <TextField className='flex flex-row items-center justify-center'>
          <Button type='submit' isDisabled={loading} isPending={loading}>
            {({ isPending }) => (
              <>
                {isPending ? <Spinner color='current' size='sm' /> : null}
                提交
              </>
            )}
          </Button>
        </TextField>
      </Form>
    </div>
  );
}
