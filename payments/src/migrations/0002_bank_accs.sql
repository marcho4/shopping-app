CREATE TABLE IF NOT EXISTS bank_accounts (
    id UUID PRIMARY KEY UNIQUE DEFAULT uuid_generate_v4(),
    user_id INT NOT NULL UNIQUE,
    balance INT NOT NULL DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_bank_accounts_user_id ON bank_accounts(user_id);