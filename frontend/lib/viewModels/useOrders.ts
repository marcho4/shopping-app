import { useEffect, useState } from "react";
import { getUserOrders } from "../app-service";
import { Order } from "../models/order";


export default function useOrders() {
    const [orders, setOrders] = useState<Order[]>([]);
    const [userId, setUserId] = useState<number | undefined>(undefined);
    const [isLoading, setIsLoading] = useState(false);

    useEffect(() => {
        if (userId !== undefined) {
            setIsLoading(true);
            getUserOrders(userId)
                .then(setOrders)
                .catch((error) => {
                    console.error(error);
                })
                .finally(() => setIsLoading(false));
        }
    }, [userId]);

    return {
        orders,
        setUserId,
        userId,
        isLoading
    };
}