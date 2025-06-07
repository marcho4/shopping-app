import {useCallback, useEffect, useState, useTransition} from "react";
import {
    createBankAccountAction,
    depositToAccountAction,
    getUserAccountsAction,
} from "@/lib";
import { useGlobalUserId } from "@/lib/viewModels/use-global-user-id";



export function useBankAccount() {
    const { userId } = useGlobalUserId();
    const [accountId, setAccountId] = useState<string | null>(null);
    const [depAmount, setDepAmount] = useState<number | null>(null);
    const [error, setError] = useState<string>();
    const [isPending, startTransition] = useTransition();
    const [balance, setBalance] = useState<number | null>(null);


    useEffect(() => {
        fetchAccount();
    }, [userId]);

    const fetchAccount = useCallback(async () => {
        if (userId !== undefined) {
            const result = await getUserAccountsAction(userId);
            if (result.success) {
                setAccountId(result.data?.id || null);
                setBalance(result.data?.balance || null);
            } else {
                setAccountId(null);
                setBalance(null);
                setError(result.error || "У вас нет банковского аккаунта");
            }
        } else {
            setBalance(null);
            setAccountId(null);
            setError(undefined);
            setDepAmount(null);
        }
    }, [userId]);

    const deposit = useCallback((onSuccess?: () => void) => {
        if (userId !== undefined && accountId !== null && depAmount !== null) {
            startTransition(async () => {
                const result = await depositToAccountAction({user_id: userId, account_id: accountId, amount: depAmount});
                if (result.success) {
                    setBalance(result.data?.balance || null);
                    if (onSuccess) {
                        onSuccess();
                    }
                } else {
                    setError(result.error || "Не удалось пополнить счет");
                }
            });
        } else {
            setError("Вы не ввели ID пользователя или сумму для пополнения или нет банковского аккаунта");
        }
    }, [userId, accountId, depAmount]);

    const createBankAccount = useCallback((onSuccess?: () => void) => {
        if (userId !== undefined) {
            startTransition(async () => {
                const result = await createBankAccountAction({user_id: userId});

                if (result.success) {
                    if (result.data?.balance !== undefined) {
                        setBalance(result.data?.balance);
                        setAccountId(result.data.id);
                    }
                    setError(undefined);
                }

                if (result.success && onSuccess) {
                    onSuccess();
                }
            });
        } else {
            setError("Вы не ввели ID пользователя");
        }
    }, [userId]);

    useEffect(() => {
        if (error) {
            const timer = setTimeout(() => {
                setError(undefined);
            }, 3000);

            return () => clearTimeout(timer);
        }
    }, [error]);

    return {
        depAmount,
        deposit,
        setDepAmount,
        createBankAccount,
        balance,
        accountId,
        error,
        fetchAccount,
    };
}