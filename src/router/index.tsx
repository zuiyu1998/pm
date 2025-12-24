import { Layout } from '@/layouts';
import { createHashRouter, redirect } from 'react-router-dom';
import { Plan } from '@/views/plan';
import { NewPlan } from '@/views/plan/new';

export const router = createHashRouter([
  {
    path: '/',
    element: <Layout />,
    children: [
      {
        path: '',
        loader: () => redirect('/plan'),
      },
      {
        path: 'plan',
        element: <Plan />,
      },
      {
        path: 'plan/new',
        element: <NewPlan />,
      },
    ],
  },
]);
