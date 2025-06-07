import { OrderStatus } from "@/lib";

export interface Order {
    id: string,
    user_id: number,
    product_id: number,
    product_price: number,
    amount: number,
    description: string,
    status: OrderStatus
}

export { OrderStatus };
