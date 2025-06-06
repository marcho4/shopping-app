import { Order } from "@/lib/models/order";
import { Badge } from "./ui/badge";

export default function OrderCard({ order }: { order: Order }) {
    return (
        <div className="flex flex-row gap-y-2 justify-between">
            <div className="flex flex-row gap-x-2">
                <p>{order.product_id}</p>
                <p>{order.amount}</p>
                <p>{order.product_price * order.amount} руб.</p>
                <p>{order.description}</p>
            </div>
            <Badge variant="outline">{order.status}</Badge>
        </div>
    )
}