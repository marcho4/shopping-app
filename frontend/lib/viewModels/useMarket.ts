import { useState, useTransition, useCallback, useEffect } from "react";
import { createOrderAction } from "@/lib/actions/orders";
import { CreateOrderDTO } from "@/lib/models/dto";

interface OrderResult {
    success: boolean;
    error: string | null;
    orderId?: string;
}

export function useMarket() {
    const [isPending, startTransition] = useTransition();
    const [orderResult, setOrderResult] = useState<OrderResult | null>(null);
    const [description, setDescription] = useState<string | undefined>();

    const createOrder = useCallback((orderData: CreateOrderDTO, onSuccess?: () => void) => {
        startTransition(async () => {
            const result = await createOrderAction(orderData);

            setOrderResult({
                success: result.success,
                error: result.error,
                orderId: result.data?.id
            });

            if (result.success && onSuccess) {
                onSuccess();
            }
        });
    }, []);

    const clearOrderResult = useCallback(() => {
        setOrderResult(null);
    }, []);

    useEffect(() => {
        if (orderResult) {
            const timer = setTimeout(() => {
                clearOrderResult();
            }, 3000);

            return () => clearTimeout(timer);
        }
    }, [orderResult, clearOrderResult]);

    return {
        isPending,
        orderResult,
        createOrder,
        clearOrderResult,
        description,
        setDescription
    };
}