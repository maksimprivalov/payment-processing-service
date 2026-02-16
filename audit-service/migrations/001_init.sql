CREATE TABLE IF NOT EXISTS audit_events (
                                            id UUID PRIMARY KEY,
                                            service_name TEXT NOT NULL,
                                            action TEXT NOT NULL,
                                            status TEXT NOT NULL,
                                            details TEXT,
                                            created_at TIMESTAMP WITH TIME ZONE NOT NULL
);