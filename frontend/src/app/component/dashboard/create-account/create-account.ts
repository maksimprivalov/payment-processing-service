import { Component } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { ApiService } from '../../../service/api.service';

@Component({
  selector: 'app-create-account',
  standalone: true,
  imports: [CommonModule, FormsModule],
  templateUrl: './create-account.html',
  styleUrls: ['./create-account.css']
})
export class CreateAccount {

  currency = 'EUR';
  success = '';
  error = '';
  loading = false;

  constructor(private api: ApiService) {}

create() {
  this.success = '';
  this.error = '';
  this.loading = true;

  this.api.createAccount(this.currency).subscribe({
    next: () => {
      this.success = 'Account created successfully';
      this.loading = false;
    },
    error: () => {
      this.error = 'Failed to create account';
      this.loading = false;
    }
  });
}
}