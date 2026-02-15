CREATE TABLE IF NOT EXISTS ledger_entries (
                                              id UUID PRIMARY KEY,
                                              payment_id UUID NOT NULL,
                                              account_id UUID NOT NULL,
                                              entry_type TEXT NOT NULL, -- DEBIT / CREDIT / COMPENSATION
                                              amount NUMERIC NOT NULL,
                                              created_at TIMESTAMP WITH TIME ZONE NOT NULL
);
