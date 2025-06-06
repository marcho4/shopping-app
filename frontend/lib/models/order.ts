import { OrderStatus } from "./order_status";

export interface Order {
    id: string,
    user_id: number,
    product_id: number,
    product_price: number,
    amount: number,
    description: string,
    status: OrderStatus
}