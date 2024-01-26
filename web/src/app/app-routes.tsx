import { lazy } from 'react';

import { Navigate, useRoutes } from 'react-router-dom';

const AccountListFeature = lazy(() => import('./account/account-list-feature'));
const AccountDetailFeature = lazy(
  () => import('./account/account-detail-feature')
);
const ClusterFeature = lazy(() => import('./cluster/cluster-feature'));

const DashboardFeature = lazy(() => import('./dashboard/dashboard-feature'));

export function AppRoutes() {
  return useRoutes([
    { index: true, element: <Navigate to={'/dashboard'} replace={true} /> },
    { path: '/account/', element: <AccountListFeature /> },
    { path: '/account/:address', element: <AccountDetailFeature /> },
    { path: '/clusters', element: <ClusterFeature /> },

    { path: '/dashboard', element: <DashboardFeature /> },
    { path: '*', element: <Navigate to={'/dashboard'} replace={true} /> },
  ]);
}
