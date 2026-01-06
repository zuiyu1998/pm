import { Layout } from '@/layouts';
import { createHashRouter, redirect } from 'react-router-dom';
import { Plan } from '@/views/plan';
import { Work } from '@/views/plan/work';

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
        children: [
          {
            path: '',
            element: <Plan />,
          },
          {
            path: 'work',
            element: <Work />,
          },
        ],
      },
    ],
  },
]);
