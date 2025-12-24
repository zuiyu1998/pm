import { Outlet } from 'react-router-dom';
import './index.css';

export function Layout() {
  return (
    <div className='app'>
      <Outlet />
    </div>
  );
}
