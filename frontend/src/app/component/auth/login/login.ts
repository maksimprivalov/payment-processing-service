import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { CommonModule } from '@angular/common';
import { ApiService } from '../../../service/api.service';

@Component({
  selector: 'app-login',
  standalone: true,
  imports: [CommonModule, FormsModule],
  templateUrl: './login.html',
  styleUrls: ['./login.css']
})
export class Login {

  email = '';
  password = '';
  loading = false;
  error = '';

  constructor(private api: ApiService) {}

  login() {
    this.error = '';
    this.loading = true;

    this.api.login({
      email: this.email,
      password: this.password
    }).subscribe({
      next: (res: any) => {
        localStorage.setItem('token', res.token);
        this.loading = false;
        alert('Login successful');
      },
      error: () => {
        this.loading = false;
        this.error = 'Invalid credentials';
      }
    });
  }
}