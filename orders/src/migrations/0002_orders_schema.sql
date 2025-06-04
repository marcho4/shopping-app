CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TYPE order_status AS ENUM ('pending', 'approved', 'rejected');

CREATE TABLE IF NOT EXISTS orders (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    product_id INT NOT NULL,
    product_price INT NOT NULL CHECK (product_price >= 0),
    user_id INT NOT NULL,
    amount INT NOT NULL CHECK (amount >= 0),
    description TEXT,
    status order_status NOT NULL DEFAULT 'pending'
);

CREATE INDEX IF NOT EXISTS idx_orders_user_id ON orders(user_id);