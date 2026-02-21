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

  constructor(private api: ApiService) {}

  create() {
    this.api.createAccount(this.currency).subscribe(() => {
      this.success = 'Account created';
    });
  }
}