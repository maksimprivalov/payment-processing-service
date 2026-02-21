import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { CommonModule } from '@angular/common';
import { ApiService } from '../../../service/api.service';
import { RouterLink } from '@angular/router';

@Component({
  selector: 'app-register',
  standalone: true,
  imports: [CommonModule, FormsModule, RouterLink],
  templateUrl: './register.html',
  styleUrls: ['./register.css']
})
export class Register {

  email = '';
  password = '';
  loading = false;
  success = '';
  error = '';

  constructor(private api: ApiService) {}

  register() {
    this.error = '';
    this.success = '';
    this.loading = true;

    this.api.register({
      email: this.email,
      password: this.password
    }).subscribe({
      next: () => {
        this.loading = false;
        this.success = 'Account created';
      },
      error: () => {
        this.loading = false;
        this.error = 'Registration failed';
      }
    });
  }
}