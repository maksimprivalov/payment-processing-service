import { Component, Input, OnChanges } from '@angular/core';
import { CommonModule } from '@angular/common';
import { ApiService } from '../../../service/api.service';

@Component({
  selector: 'app-transactions',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './transactions.html',
  styleUrls: ['./transactions.css']
})
export class Transactions implements OnChanges {

  @Input() accountId!: string;

  transactions: any[] = [];

  constructor(private api: ApiService) {}

  ngOnChanges() {
    if (!this.accountId) return;

    this.api.getTransactions(this.accountId)
      .subscribe((res: any) => this.transactions = res);
  }
}