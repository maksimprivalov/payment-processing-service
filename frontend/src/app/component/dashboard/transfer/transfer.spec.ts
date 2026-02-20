import { ComponentFixture, TestBed } from '@angular/core/testing';

import { Transfer } from './transfer';

describe('Transfer', () => {
  let component: Transfer;
  let fixture: ComponentFixture<Transfer>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [Transfer]
    })
    .compileComponents();

    fixture = TestBed.createComponent(Transfer);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
