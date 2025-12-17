import { Layout } from '@/layouts';
import { createHashRouter } from 'react-router-dom';

export const router = createHashRouter([
  {
    path: '/',
    element: <Layout />,
  },
]);
