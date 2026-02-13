CREATE TABLE IF NOT EXISTS accounts (
                                        id UUID PRIMARY KEY,
                                        user_id UUID NOT NULL,
                                        balance NUMERIC NOT NULL,
                                        currency TEXT NOT NULL,
                                        created_at TIMESTAMP WITH TIME ZONE NOT NULL
);