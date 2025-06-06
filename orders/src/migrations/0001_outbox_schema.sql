CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TYPE outbox_status AS ENUM ('pending', 'processed', 'failed');

CREATE TABLE IF NOT EXISTS outbox (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    order_id UUID NOT NULL,
    payload JSONB NOT NULL,
    status outbox_status NOT NULL DEFAULT 'pending'
);