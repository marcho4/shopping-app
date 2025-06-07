import {useCallback, useEffect, useState} from "react";
import { getUserOrdersAction } from "@/lib";
import { Order } from "@/lib";
import { useGlobalUserId } from "@/lib/viewModels/use-global-user-id";


export default function useOrders() {
    const { userId } = useGlobalUserId();
    const [orders, setOrders] = useState<Order[]>([]);
    const [isLoading, setIsLoading] = useState(false);

    const fetchOrders = useCallback(() => {
        if (userId !== undefined) {
            setIsLoading(true);
            getUserOrdersAction(userId)
                .then((result) => {
                    if (result.success) {
                        setOrders(result.data || []);
                    } else {
                        setOrders([]);
                    }
                })
                .catch((error) => {
                    console.error(error);
                })
                .finally(() => {setIsLoading(false)});
        }
    }, [userId]);

    useEffect(() => {
        fetchOrders();
    }, [fetchOrders, userId]);

    return {
        orders,
        isLoading,
        fetchOrders
    };
}