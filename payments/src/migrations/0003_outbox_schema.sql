CREATE TYPE outbox_status AS ENUM ('pending', 'processed', 'failed');

CREATE TABLE IF NOT EXISTS outbox (
    order_id UUID PRIMARY KEY NOT NULL,
    order_status order_status NOT NULL,
    status outbox_status NOT NULL DEFAULT 'pending'
);

CREATE INDEX IF NOT EXISTS idx_outbox_order_id ON outbox(order_id);