CREATE TABLE IF NOT EXISTS payments (
                                        id UUID PRIMARY KEY,
                                        user_id UUID NOT NULL,
                                        from_account UUID NOT NULL,
                                        to_account UUID NOT NULL,
                                        amount NUMERIC NOT NULL,
                                        status TEXT NOT NULL,
                                        created_at TIMESTAMP WITH TIME ZONE NOT NULL
);