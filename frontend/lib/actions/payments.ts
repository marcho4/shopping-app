"use server";

import {
    createBankAccount,
    depositToAccount,
    getUserAccounts,
    getAccountBalance
} from "@/lib/app-service";
import { CreateAccountDTO, DepositDTO } from "@/lib/models/dto";
import { revalidatePath } from "next/cache";

export async function createBankAccountAction(accountData: CreateAccountDTO) {
    try {
        const account = await createBankAccount(accountData);
        revalidatePath('/');
        return {
            success: true,
            data: account,
            error: null
        };
    } catch (error) {
        console.error('Failed to create bank account:', error);
        return {
            success: false,
            data: null,
            error: error instanceof Error ? error.message : 'Произошла неизвестная ошибка'
        };
    }
}

export async function depositToAccountAction(depositData: DepositDTO) {
    try {
        const balance = await depositToAccount(depositData);
        revalidatePath('/');
        return {
            success: true,
            data: balance,
            error: null
        };
    } catch (error) {
        console.error('Failed to deposit to account:', error);
        return {
            success: false,
            data: null,
            error: error instanceof Error ? error.message : 'Произошла неизвестная ошибка'
        };
    }
}

export async function getUserAccountsAction(userId: number) {
    try {
        const accounts = await getUserAccounts(userId);
        return {
            success: true,
            data: accounts,
            error: null
        };
    } catch (error) {
        console.error('Failed to get user accounts:', error);
        return {
            success: false,
            data: null,
            error: error instanceof Error ? error.message : 'Произошла неизвестная ошибка'
        };
    }
}

export async function getAccountBalanceAction(accountId: string) {
    try {
        const balance = await getAccountBalance(accountId);
        return {
            success: true,
            data: balance,
            error: null
        };
    } catch (error) {
        console.error('Failed to get account balance:', error);
        return {
            success: false,
            data: null,
            error: error instanceof Error ? error.message : 'Произошла неизвестная ошибка'
        };
    }
}