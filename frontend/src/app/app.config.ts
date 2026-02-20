import { Component } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [CommonModule, FormsModule],
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.css']
})
export class AppComponent {

  constructor(private http: HttpClient) {}

  apiAuth = 'http://localhost:8080';
  apiAccount = 'http://localhost:8081';
  apiSaga = 'http://localhost:8085';

  email = '';
  password = '';
  token = localStorage.getItem('token') || '';

  accounts: any[] = [];
  selectedAccount = '';
  amount = 0;
  transferTo = '';

  register() {
    this.http.post<any>(`${this.apiAuth}/register`, {
      email: this.email,
      password: this.password
    }).subscribe(res => {
      alert('Registered');
    });
  }

  login() {
    this.http.post<any>(`${this.apiAuth}/login`, {
      email: this.email,
      password: this.password
    }).subscribe(res => {
      this.token = res.token;
      localStorage.setItem('token', this.token);
      alert('Logged in');
    });
  }

  createAccount() {
    this.http.post<any>(`${this.apiAccount}/accounts`,
      { currency: 'EUR' },
      { headers: { Authorization: 'Bearer ' + this.token } }
    ).subscribe(res => {
      this.accounts.push(res);
    });
  }

  loadAccount(id: string) {
    this.http.get<any>(`${this.apiAccount}/accounts/${id}`, {
      headers: { Authorization: 'Bearer ' + this.token }
    }).subscribe(res => {
      alert(`Balance: ${res.balance}`);
    });
  }

  credit() {
    this.http.post<any>(`${this.apiAccount}/accounts/${this.selectedAccount}/credit`,
      { amount: this.amount },
      { headers: { Authorization: 'Bearer ' + this.token } }
    ).subscribe(() => alert('Credited'));
  }

  transfer() {
    this.http.post<any>(`${this.apiSaga}/transfer`,
      {
        from_account: this.selectedAccount,
        to_account: this.transferTo,
        amount: this.amount
      },
      { headers: { Authorization: 'Bearer ' + this.token } }
    ).subscribe(() => alert('Transfer complete'));
  }
}