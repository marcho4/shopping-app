export interface CreateOrderDTO {
    product_id: number;
    user_id: number;
    amount: number;
    description: string;
    product_price: number;
}

export interface CreateAccountDTO {
    user_id: number;
}

export interface DepositDTO {
    account_id: string;
    amount: number;
    user_id: number;
}

export interface BalanceDTO {
    balance: number;
}

export interface ErrorResponse {
    error: string;
    message: string;
}