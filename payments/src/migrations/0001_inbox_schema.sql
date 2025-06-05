CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TYPE inbox_status AS ENUM ('pending', 'processed', 'failed');
CREATE TYPE order_status AS ENUM ('pending', 'approved', 'rejected');

CREATE TABLE IF NOT EXISTS inbox (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    order_id UUID UNIQUE NOT NULL,
    payload JSONB NOT NULL,
    status inbox_status NOT NULL DEFAULT 'pending'
);

CREATE INDEX IF NOT EXISTS idx_inbox_order_id ON inbox(order_id);
