import { Order } from "@/lib/models/order";
import { Badge } from "./ui/badge";

export default function OrderCard({ order }: { order: Order }) {
    return (
        <div className=" w-full mx-auto p-4 bg-white rounded-lg shadow flex items-center justify-between">
            <div className="grid grid-cols-12 gap-2 flex-1">
                <p className="col-span-1 font-mono text-sm">{order.product_id}</p>
                <p className="col-span-2 font-mono text-sm">{order.amount} шт.</p>
                <p className="col-span-4 font-semibold text-sm">
                    {order.product_price * order.amount} ₽
                </p>
                <p className="col-span-5 text-sm truncate">{order.description}</p>
            </div>
            <Badge
                variant="outline"
                className={`ml-4 ${
                    order.status === "approved" ? "bg-emerald-300" : (order.status == "pending" ? "bg-yellow-300" : "bg-red-400")
                }`}
            >
                {order.status === "approved" ? "Оплачен" : (order.status == "pending" ? "Ожидает" : "Отказано")}
            </Badge>
        </div>
    );
}