import { Component, OnInit } from '@angular/core';
import { CommonModule } from '@angular/common';
import { ApiService } from '../../../service/api.service';
import { AuthService } from '../../../service/auth.service';
import { Router } from '@angular/router';
import { FormsModule } from '@angular/forms';

@Component({
  selector: 'app-accounts',
  standalone: true,
  imports: [CommonModule, FormsModule],
  templateUrl: './accounts.html',
  styleUrls: ['./accounts.css']
})
export class Accounts implements OnInit {

  accounts: any[] = [];
  selectedAccount: string = '';
  creditAmount: number = 0;

  // transfer
  fromAccount = '';
  toAccount = '';
  transferAmount = 0;

  transactions: any[] = [];

  constructor(
    private api: ApiService,
    private auth: AuthService,
    private router: Router
  ) {}

  ngOnInit() {
    this.loadAccounts();
  }

  loadAccounts() {
    this.api.getAccounts().subscribe((res: any) => {
      this.accounts = res;
    });
  }

  logout() {
    this.auth.logout();
    this.router.navigate(['/']);
  }

  credit() {
    this.api.credit(this.selectedAccount, this.creditAmount)
      .subscribe(() => this.loadAccounts());
  }

  transfer() {
    this.api.transfer({
      from_account: this.fromAccount,
      to_account: this.toAccount,
      amount: this.transferAmount
    }).subscribe(() => this.loadAccounts());
  }

  loadTransactions(accountId: string) {
    this.api.getTransactions(accountId)
      .subscribe((res: any) => this.transactions = res);
  }
}