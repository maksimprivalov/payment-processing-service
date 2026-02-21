import { Component, OnInit } from '@angular/core';
import { CommonModule } from '@angular/common';
import { ApiService } from '../../../service/api.service';
import { AuthService } from '../../../service/auth.service';
import { Router } from '@angular/router';
import { FormsModule } from '@angular/forms';
import { CreateAccount } from '../create-account/create-account';
import { Transactions } from '../transactions/transactions';

@Component({
  selector: 'app-accounts',
  standalone: true,
  imports: [CommonModule, FormsModule, CreateAccount, Transactions],
  templateUrl: './accounts.html',
  styleUrls: ['./accounts.css']
})
export class Accounts implements OnInit {

  accounts: any[] = [];
  selectedAccount: string = '';
  creditAmount: number = 0;
  
  currentView: 'list' | 'create' | 'transfer' | 'transactions' | 'topup' = 'list';
  selectedForTransactions = '';

  // transfer
  fromAccount = '';
  toAccount = '';
  transferAmount = 0;
  transferSuccess = '';
  transferError = '';
  transferLoading = false;

  topupAmount = 0;
  topupAccount = '';
  topupSuccess = '';
  topupError = '';
  topupLoading = false;

  transactions: any[] = [];

  constructor(
    private api: ApiService,
    private auth: AuthService,
    private router: Router
  ) {}

  ngOnInit() {
    this.loadAccounts();

    this.api.accountsUpdated$.subscribe(() => {
      this.loadAccounts();
    });
  }

  goToTopup() {
    this.currentView = 'topup';
  }

  goToCreate() {
    this.currentView = 'create';
  }

  goToTransfer() {
    this.currentView = 'transfer';
  }

  goToList() {
    this.currentView = 'list';
  }

  showTransactions(accountId: string) {
    this.selectedForTransactions = accountId;
    this.currentView = 'transactions';
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
    this.transferSuccess = '';
    this.transferError = '';

    if (!this.fromAccount || !this.toAccount || this.transferAmount <= 0) {
      this.transferError = 'Invalid transfer details';
      return;
    }

    this.transferLoading = true;

    this.api.transfer({
      from_account: this.fromAccount,
      to_account: this.toAccount,
      amount: this.transferAmount
    }).subscribe({
      next: () => {
        this.transferSuccess = 'Successfully transferred';
        this.transferLoading = false;
        this.loadAccounts();
        this.api.notifyAccountsUpdated();
        this.fromAccount = '';
        this.toAccount = '';
        this.transferAmount = 0;
      },
      error: (err: any) => {
        this.transferError = err?.error?.message || 'Transfer failed';
        this.transferLoading = false;
      }
    });
  }

  loadTransactions(accountId: string) {
    this.api.getTransactions(accountId)
      .subscribe((res: any) => this.transactions = res);
  }

  topUp() {
    this.topupSuccess = '';
    this.topupError = '';

    if (!this.topupAccount || this.topupAmount <= 0) {
      this.topupError = 'Invalid amount or account';
      return;
    }

    this.topupLoading = true;

    this.api.credit(this.topupAccount, this.topupAmount)
      .subscribe({
        next: () => {
          this.topupSuccess = 'Balance topped up';
          this.topupLoading = false;
          this.api.notifyAccountsUpdated();
          this.loadAccounts();
        },
        error: () => {
          this.topupError = 'Top up failed';
          this.topupLoading = false;
        }
      });
  }
}