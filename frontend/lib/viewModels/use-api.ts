"use client";

import { useState, useEffect, useCallback } from 'react';
import {
    getUserOrders,
    createOrder,
    getOrderStatus,
    createBankAccount,
    depositToAccount,
    getUserAccounts,
    getAccountBalance
} from '@/lib/app-service';
import { Order } from '@/lib/models/order';
import { OrderStatus } from '@/lib/models/order_status';
import { BankAccount } from '@/lib/models/bank_account';
import { CreateOrderDTO, CreateAccountDTO, DepositDTO, BalanceDTO } from '@/lib/models/dto';
import { getErrorMessage } from '../utils';

// Базовый тип для состояния загрузки
interface ApiState<T> {
    data: T | null;
    loading: boolean;
    error: string | null;
}

export function useUserOrders(userId: number | null) {
    const [state, setState] = useState<ApiState<Order[]>>({
        data: null,
        loading: false,
        error: null,
    });

    const fetchOrders = useCallback(async () => {
        if (!userId) {
            setState({ data: null, loading: false, error: null });
            return;
        }

        setState(prev => ({ ...prev, loading: true, error: null }));

        try {
            const orders = await getUserOrders(userId);
            setState({ data: orders, loading: false, error: null });
        } catch (error) {
            setState({ data: null, loading: false, error: getErrorMessage(error) });
        }
    }, [userId]);

    useEffect(() => {
        fetchOrders();
    }, [fetchOrders]);

    return { ...state, refetch: fetchOrders };
}

// Хук для создания заказа
export function useCreateOrder() {
    const [state, setState] = useState<ApiState<Order>>({
        data: null,
        loading: false,
        error: null,
    });

    const createOrderMutation = useCallback(async (orderData: CreateOrderDTO) => {
        setState(prev => ({ ...prev, loading: true, error: null }));

        try {
            const order = await createOrder(orderData);
            setState({ data: order, loading: false, error: null });
            return order;
        } catch (error) {
            const errorMessage = getErrorMessage(error);
            setState({ data: null, loading: false, error: errorMessage });
            throw error;
        }
    }, []);

    return { ...state, createOrder: createOrderMutation };
}

// Хук для получения статуса заказа
export function useOrderStatus(orderId: string | null) {
    const [state, setState] = useState<ApiState<OrderStatus>>({
        data: null,
        loading: false,
        error: null,
    });

    const fetchStatus = useCallback(async () => {
        if (!orderId) {
            setState({ data: null, loading: false, error: null });
            return;
        }

        setState(prev => ({ ...prev, loading: true, error: null }));

        try {
            const status = await getOrderStatus(orderId);
            setState({ data: status, loading: false, error: null });
        } catch (error) {
            setState({ data: null, loading: false, error: getErrorMessage(error) });
        }
    }, [orderId]);

    useEffect(() => {
        fetchStatus();
    }, [fetchStatus]);

    return { ...state, refetch: fetchStatus };
}

// Хук для получения счетов пользователя
export function useUserAccounts(userId: number | null) {
    const [state, setState] = useState<ApiState<BankAccount>>({
        data: null,
        loading: false,
        error: null,
    });

    const fetchAccounts = useCallback(async () => {
        if (!userId) {
            setState({ data: null, loading: false, error: null });
            return;
        }

        setState(prev => ({ ...prev, loading: true, error: null }));

        try {
            const accounts = await getUserAccounts(userId);
            setState({ data: accounts, loading: false, error: null });
        } catch (error) {
            setState({ data: null, loading: false, error: getErrorMessage(error) });
        }
    }, [userId]);

    useEffect(() => {
        fetchAccounts();
    }, [fetchAccounts]);

    return { ...state, refetch: fetchAccounts };
}

// Хук для создания банковского счета
export function useCreateBankAccount() {
    const [state, setState] = useState<ApiState<BankAccount>>({
        data: null,
        loading: false,
        error: null,
    });

    const createAccountMutation = useCallback(async (accountData: CreateAccountDTO) => {
        setState(prev => ({ ...prev, loading: true, error: null }));

        try {
            const account = await createBankAccount(accountData);
            setState({ data: account, loading: false, error: null });
            return account;
        } catch (error) {
            const errorMessage = getErrorMessage(error);
            setState({ data: null, loading: false, error: errorMessage });
            throw error;
        }
    }, []);

    return { ...state, createAccount: createAccountMutation };
}

export function useDepositToAccount() {
    const [state, setState] = useState<ApiState<BalanceDTO>>({
        data: null,
        loading: false,
        error: null,
    });

    const depositMutation = useCallback(async (depositData: DepositDTO) => {
        setState(prev => ({ ...prev, loading: true, error: null }));

        try {
            const balance = await depositToAccount(depositData);
            setState({ data: balance, loading: false, error: null });
            return balance;
        } catch (error) {
            const errorMessage = getErrorMessage(error);
            setState({ data: null, loading: false, error: errorMessage });
            throw error;
        }
    }, []);

    return { ...state, deposit: depositMutation };
}

export function useAccountBalance(accountId: string | null) {
    const [state, setState] = useState<ApiState<BalanceDTO>>({
        data: null,
        loading: false,
        error: null,
    });

    const fetchBalance = useCallback(async () => {
        if (!accountId) {
            setState({ data: null, loading: false, error: null });
            return;
        }

        setState(prev => ({ ...prev, loading: true, error: null }));

        try {
            const balance = await getAccountBalance(accountId);
            setState({ data: balance, loading: false, error: null });
        } catch (error) {
            setState({ data: null, loading: false, error: getErrorMessage(error) });
        }
    }, [accountId]);

    useEffect(() => {
        fetchBalance();
    }, [fetchBalance]);

    return { ...state, refetch: fetchBalance };
}