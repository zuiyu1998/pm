import { Outlet } from 'react-router-dom';
import { AppTitleBar } from './AppTitleBar/AppTitleBar';

export function Layout() {
  return (
    <div>
      <AppTitleBar />
      <Outlet />
    </div>
  );
}
