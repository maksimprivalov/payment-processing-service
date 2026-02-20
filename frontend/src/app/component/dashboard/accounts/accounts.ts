import { Component, OnInit } from '@angular/core';
import { ApiService } from '../../../service/api.service';

@Component({
  selector: 'app-accounts',
  templateUrl: './accounts.html'
})
export class Accounts implements OnInit {

  accounts: any[] = [];
  selected?: string;
  amount = 0;
  transactions: any[] = [];

  constructor(private api: ApiService) {}

  ngOnInit() {
    this.load();
  }

  load() {
    this.api.getAccounts().subscribe((res: any) => {
      this.accounts = res;
    });
  }

  credit() {
    this.api.credit(this.selected!, this.amount)
      .subscribe(() => this.load());
  }

  loadTransactions() {
    this.api.getTransactions(this.selected!)
      .subscribe((res: any) => this.transactions = res);
  }
}