export const API_ENDPOINTS = {
    ORDERS: '/orders',
    ORDER_STATUS: (orderId: string) => `/orders/status/${orderId}`,
    USER_ORDERS: (userId: number) => `/orders/${userId}`,
    
    PAYMENTS: '/payments',
    USER_ACCOUNTS: (userId: number) => `/payments/accounts/${userId}`,
    ACCOUNT_BALANCE: (accountId: string) => `/payments/balance/${accountId}`,
} as const;

export const HTTP_METHODS = {
    GET: 'GET',
    POST: 'POST',
    PUT: 'PUT',
    DELETE: 'DELETE',
    PATCH: 'PATCH',
} as const;

export const HTTP_STATUS = {
    OK: 200,
    CREATED: 201,
    BAD_REQUEST: 400,
    UNAUTHORIZED: 401,
    FORBIDDEN: 403,
    NOT_FOUND: 404,
    INTERNAL_SERVER_ERROR: 500,
} as const;

export const DEFAULT_ERROR_MESSAGES = {
    NETWORK_ERROR: 'Ошибка сети. Проверьте подключение к интернету.',
    SERVER_ERROR: 'Внутренняя ошибка сервера. Попробуйте позже.',
    NOT_FOUND: 'Запрашиваемый ресурс не найден.',
    UNAUTHORIZED: 'Необходимо авторизоваться.',
    FORBIDDEN: 'Недостаточно прав для выполнения операции.',
    BAD_REQUEST: 'Некорректные данные запроса.',
    UNKNOWN_ERROR: 'Произошла неизвестная ошибка.',
} as const; 