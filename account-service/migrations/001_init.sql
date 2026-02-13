CREATE TABLE IF NOT EXISTS users (
                                     id UUID PRIMARY KEY,
                                     email TEXT UNIQUE NOT NULL,
                                     password_hash TEXT NOT NULL,
                                     status TEXT NOT NULL,
                                     created_at TIMESTAMP WITH TIME ZONE NOT NULL
);