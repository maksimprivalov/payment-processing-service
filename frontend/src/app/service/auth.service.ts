import { Injectable } from '@angular/core';

@Injectable({ providedIn: 'root' })
export class AuthService {

  get token(): string | null {
    return localStorage.getItem('token');
  }

  isAuthenticated(): boolean {
    return !!this.token;
  }

  login(token: string) {
    localStorage.setItem('token', token);
  }

  logout() {
    localStorage.removeItem('token');
  }
}