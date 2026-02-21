import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { CommonModule } from '@angular/common';
import { ApiService } from '../../../service/api.service';
import { Router, RouterLink } from '@angular/router';
import { AuthService } from '../../../service/auth.service';

@Component({
  selector: 'app-login',
  standalone: true,
  imports: [CommonModule, FormsModule, RouterLink],
  templateUrl: './login.html',
  styleUrls: ['./login.css']
})
export class Login {

  email = '';
  password = '';
  loading = false;
  error = '';

  constructor(
  private api: ApiService,
  private auth: AuthService,
  private router: Router
) {}

  login() {
    this.error = '';
    this.loading = true;

    this.api.login({
      email: this.email,
      password: this.password
    }).subscribe({
      next: (res: any) => {
        this.auth.login(res.token);
        this.router.navigate(['/accounts']);
        this.loading = false;
        // alert('Login successful');
      },
      error: () => {
        this.loading = false;
        this.error = 'Invalid credentials';
      }
    });
  }
}