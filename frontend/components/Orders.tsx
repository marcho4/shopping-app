import OrderCard from "./order";
import { Card, CardContent, CardHeader, CardTitle } from "./ui/card";
import useOrders from "@/lib/viewModels/useOrders";
import {useGlobalUserId} from "@/lib/viewModels/use-global-user-id";
import { Button } from "./ui/button";
import { RefreshCcwIcon } from "lucide-react";


export default function Orders() {
    const { orders, isLoading, fetchOrders } = useOrders();
    const {userId} = useGlobalUserId();
    return (
        <Card>
            <CardHeader>
                <CardTitle>
                    <div className="flex flex-row w-full items-center gap-1.5 justify-between">
                        <p>Мои заказы</p>
                        {userId && (
                            <Button variant="outline" size="icon" onClick={() => fetchOrders()}>
                                <RefreshCcwIcon className="w-4 h-4" />
                            </Button>
                        )}
                    </div>
                </CardTitle>
            </CardHeader>
            <CardContent className="flex flex-col gap-4">
                {userId ? (
                    <div className={'flex flex-col gap-1 max-h-72 overflow-y-auto'}>
                        {orders.map((order) => (
                            <OrderCard key={order.id} order={order} />
                        ))}
                        {!isLoading && orders.length === 0 && <p>У вас нет заказов</p>}
                    </div>
                ) : <p>Введите User ID чтобы посмотреть свои заказы</p>
                }
            </CardContent>
        </Card>
    )
}