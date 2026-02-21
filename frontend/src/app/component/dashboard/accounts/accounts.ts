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
  
  currentView: 'list' | 'create' | 'transfer' | 'transactions' = 'list';
  selectedForTransactions = '';

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