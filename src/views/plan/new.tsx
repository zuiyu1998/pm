import { Button, Form, Input, Label } from '@heroui/react';
import React from 'react';

export function NewPlan() {
  const [submitted] = React.useState(null);

  const onSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
  };

  return (
    <Form onSubmit={onSubmit}>
      <Label htmlFor='name'>Name</Label>
      <Input
        required
        name='email'
        placeholder='Enter your email'
        type='email'
      />
      <Button type='submit'>Submit</Button>
    </Form>
  );
}
