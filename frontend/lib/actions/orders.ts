"use server";

import { createOrder, getUserOrders, getOrderStatus } from "@/lib/app-service";
import { CreateOrderDTO } from "@/lib/models/dto";
import { revalidatePath } from "next/cache";

/**
 * Server Action для создания заказа
 */
export async function createOrderAction(orderData: CreateOrderDTO) {
    try {
        console.log('Creating order on server:', orderData);
        const order = await createOrder(orderData);
        
        // Обновляем кэш страницы после создания заказа
        revalidatePath('/');
        
        return {
            success: true,
            data: order,
            error: null
        };
    } catch (error) {
        console.error('Failed to create order:', error);
        return {
            success: false,
            data: null,
            error: error instanceof Error ? error.message : 'Произошла неизвестная ошибка'
        };
    }
}

/**
 * Server Action для получения заказов пользователя
 */
export async function getUserOrdersAction(userId: number) {
    try {
        const orders = await getUserOrders(userId);
        return {
            success: true,
            data: orders,
            error: null
        };
    } catch (error) {
        console.error('Failed to get user orders:', error);
        return {
            success: false,
            data: null,
            error: error instanceof Error ? error.message : 'Произошла неизвестная ошибка'
        };
    }
}

/**
 * Server Action для получения статуса заказа
 */
export async function getOrderStatusAction(orderId: string) {
    try {
        const status = await getOrderStatus(orderId);
        return {
            success: true,
            data: status,
            error: null
        };
    } catch (error) {
        console.error('Failed to get order status:', error);
        return {
            success: false,
            data: null,
            error: error instanceof Error ? error.message : 'Произошла неизвестная ошибка'
        };
    }
} 