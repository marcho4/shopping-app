import { Order } from "@/lib/models/order";
import { OrderStatus } from "@/lib/models/order_status";
import { BankAccount } from "@/lib/models/bank_account";
import {
    CreateOrderDTO,
    CreateAccountDTO,
    DepositDTO,
    BalanceDTO,
    ErrorResponse
} from "@/lib/models/dto";

const getApiBaseUrl = () => {
    if (typeof window === 'undefined') {
        return process.env.API_URL || 'http://gateway:8000';
    }
    return process.env.NEXT_PUBLIC_API_URL || 'http://localhost:8000';
};

const API_BASE_URL = getApiBaseUrl();

export class ApiError extends Error {
    constructor(
        message: string,
        public status: number,
        public errorResponse?: ErrorResponse
    ) {
        super(message);
        this.name = 'ApiError';
    }
}

async function apiRequest<T>(
    endpoint: string,
    options: RequestInit = {}
): Promise<T> {
    const url = `${API_BASE_URL}${endpoint}`;

    try {

        const response = await fetch(url, {
            headers: {
                'Content-Type': 'application/json',
                ...options.headers,
            },
            ...options,
        });

        if (!response.ok) {
            let errorResponse: ErrorResponse | undefined;
            try {
                errorResponse = await response.json() as ErrorResponse;
            } catch {
            }

            throw new ApiError(
                errorResponse?.error || `HTTP ${response.status}: ${response.statusText}`,
                response.status,
                errorResponse
            );
        }
        return await response.json() as T;
    } catch (error) {
        console.error('API request failed:', error);

        if (error instanceof TypeError && error.message.includes('fetch')) {
            throw new ApiError(
                `Failed to connect to the server at ${url}. Please ensure the backend is running.`,
                0
            );
        }

        throw error;
    }
}

export async function getUserOrders(userId: number): Promise<Order[]> {
    return apiRequest<Order[]>(`/orders/${userId}`);
}

export async function createOrder(orderData: CreateOrderDTO): Promise<Order> {
    return apiRequest<Order>('/orders', {
        method: 'POST',
        body: JSON.stringify(orderData),
    });
}

export async function getOrderStatus(orderId: string): Promise<OrderStatus> {
    return apiRequest<OrderStatus>(`/orders/status/${orderId}`);
}

export async function createBankAccount(accountData: CreateAccountDTO): Promise<BankAccount> {
    return apiRequest<BankAccount>('/payments', {
        method: 'POST',
        body: JSON.stringify(accountData),
    });
}

export async function depositToAccount(depositData: DepositDTO): Promise<BalanceDTO> {
    return apiRequest<BalanceDTO>('/payments', {
        method: 'PUT',
        body: JSON.stringify(depositData),
    });
}


export async function getUserAccounts(userId: number): Promise<BankAccount> {
    return apiRequest<BankAccount>(`/payments/accounts/${userId}`);
}

export async function getAccountBalance(accountId: string): Promise<BalanceDTO> {
    return apiRequest<BalanceDTO>(`/payments/balance/${accountId}`);
}


export function isApiError(error: unknown): error is ApiError {
    return error instanceof ApiError;
}

export function getErrorMessage(error: unknown): string {
    if (isApiError(error)) {
        return error.errorResponse?.message || error.message;
    }

    if (error instanceof Error) {
        return error.message;
    }

    return 'Произошла неизвестная ошибка';
}