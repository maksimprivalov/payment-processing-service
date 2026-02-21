import { Routes } from '@angular/router';
import { Accounts } from './component/dashboard/accounts/accounts';
import { Register } from './component/auth/register/register';
import { Login } from './component/auth/login/login';
import { guestGuard } from './guard/guest.guard';
import { authGuard } from './guard/auth.guard';


export const routes: Routes = [
  {
    path: '',
    redirectTo: 'login',
    pathMatch: 'full'
  },
  {
    path: 'login',
    component: Login,
    canActivate: [guestGuard]
  },
  {
    path: 'register',
    component: Register,
    canActivate: [guestGuard]
  },
  {
    path: 'accounts',
    component: Accounts,
    canActivate: [authGuard]
  },
  {
    path: '**',
    redirectTo: ''
  }
];