"use server";

import { Order } from "@/lib/models/order";

export async function getUserOrders(userId: number): Promise<Order[]> {
    const response = await fetch(`${process.env.API_URL}/orders/user/${userId}`);
    if (!response.ok) {
        throw new Error(`Failed to fetch orders: ${response.statusText}`);
    }
    return await response.json() as Order[];
}