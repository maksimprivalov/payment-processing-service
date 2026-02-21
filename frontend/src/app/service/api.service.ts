import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';

@Injectable({ providedIn: 'root' })
export class ApiService {

  saga = 'http://localhost:8085';
  auth = 'http://localhost:8080';

  constructor(private http: HttpClient) {}

  get token() {
    return localStorage.getItem('token') || '';
  }

  login(data: any) {
    return this.http.post(`${this.auth}/login`, data);
  }

  register(data: any) {
    return this.http.post(`${this.auth}/register`, data);
  }

  getAccounts() {
    return this.http.get(`${this.auth}/accounts`);
  }

  credit(accountId: string, amount: number) {
    return this.http.post(
      `${this.saga}/accounts/${accountId}/credit`,
      { amount }
    );
  }

  transfer(data: any) {
    return this.http.post(
      `${this.saga}/transfer`,
      data
    );
  }

  getTransactions(accountId: string) {
    return this.http.get(
      `${this.saga}/transactions/${accountId}`
    );
  }
}