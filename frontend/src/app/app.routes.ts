import { Routes } from '@angular/router';
import { Accounts } from './component/dashboard/accounts/accounts';
import { Register } from './component/auth/register/register';
import { Login } from './component/auth/login/login';


export const routes: Routes = [
  { path: '', component: Login },
  { path: 'register', component: Register },
  { path: 'accounts', component: Accounts },
];