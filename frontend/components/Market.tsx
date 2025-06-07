"use client";

import { PRODUCTS } from "@/lib/constants";
import ProductCard from "./ProductCard";
import { Card, CardContent, CardHeader, CardTitle } from "./ui/card";
import { Button } from "./ui/button";
import { Minus } from "lucide-react";
import { useShoppingCart } from "@/lib/viewModels/use-shopping-cart";
import { useMarket } from "@/lib/viewModels/useMarket";
import { useGlobalUserId } from '@/lib/viewModels/use-global-user-id';
import {Input} from "@/components/ui/input";

export default function Market() {
    const { cartItem, total, addToCart, removeOneFromCart, isEmpty, clearCart } = useShoppingCart();
    const { isPending, orderResult, createOrder, description, setDescription } = useMarket();
    const { userId } = useGlobalUserId();

    return (
        <Card className="col-span-12 lg:col-span-7">
            <CardHeader>
                <CardTitle>Магазин товаров</CardTitle>
            </CardHeader>
            <CardContent className="flex flex-col gap-4">
                {userId ? (
                    <>
                        <h3 className="text-lg font-medium w-full text-left text-gray-800">Выберите товар</h3>
                        <div className="grid grid-cols-3 gap-4">
                            <div className="grid grid-cols-1 sm:grid-cols-2 col-span-2 w-full gap-4 h-full items-stretch">
                                {PRODUCTS.map((product) => (
                                    <ProductCard key={product.id} name={product.name} description={product.description} price={product.price} onAddToCart={() => addToCart(product.id)} />
                                ))}
                            </div>
                            <div className="flex flex-col items-start w-full gap-4 col-span-1">
                                <h3 className="text-lg font-semibold w-full text-left text-gray-800">Ваша корзина</h3>
                                <div className="flex flex-col items-start w-full gap-4">
                                    {cartItem && (
                                        <div key={cartItem.product_id} className="flex flex-row items-center w-full gap-4 justify-between">
                                            <p className="text-sm text-gray-800 w-2/5">{PRODUCTS.find((product) => product.id === cartItem.product_id)?.name}</p>
                                            <p className="text-sm text-gray-800 w-1/5">{cartItem.amount}</p>
                                            <Button variant="outline" size='icon' onClick={() => removeOneFromCart(cartItem.product_id)}>
                                                <Minus className="w-4 h-4" />
                                            </Button>
                                        </div>
                                    )}
                                    <p className="text-lg font-medium w-full text-left text-gray-800">Итого: {total} руб.</p>
                                    <Input placeholder={'Введите описание покупки'} onChange={(e) => setDescription(e.target.value)}/>
                                    <Button
                                        size='icon'
                                        className="w-full"
                                        disabled={isEmpty || isPending || description === undefined}
                                        onClick={async () => {
                                            if (!cartItem) return;
                                            createOrder({
                                                user_id: userId,
                                                description: description || "",
                                                product_id: cartItem.product_id,
                                                amount: cartItem.amount,
                                                product_price: cartItem.product_price
                                            }, () => {
                                                clearCart();
                                                setDescription(undefined);
                                            });
                                        }}
                                    >
                                        {isPending ? "Загрузка..." : "Оплатить"}
                                    </Button>
                                    {orderResult?.error && (
                                        <p className="text-sm text-red-500 w-full text-left">
                                            Ошибка: {orderResult.error}
                                        </p>
                                    )}
                                    {orderResult?.success && (
                                        <p className="text-sm text-green-500 w-full text-left">
                                            Заказ успешно создан! ID: {orderResult.orderId}
                                        </p>
                                    )}
                                </div>
                            </div>
                        </div>
                    </>
                    ) : (
                    <p>Введите User ID чтобы зайти в магазин </p>
                )}

            </CardContent>
        </Card>
    )
}