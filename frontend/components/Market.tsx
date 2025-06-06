"use client";

import { PRODUCTS } from "@/lib/constants";
import ProductCard from "./ProductCard";
import { Card, CardContent, CardHeader, CardTitle } from "./ui/card";
import { useState } from "react";
import { Button } from "./ui/button";
import { Minus } from "lucide-react";

export default function Market() {
    const [cart, setCart] = useState<Map<number, number>>(new Map());
    const [total, setTotal] = useState<number>(0);

    const handleAddToCart = (productId: number) => {
        setCart((prevCart) => {
            const newCart = new Map();
            
            // Проверяем, есть ли уже этот продукт в корзине
            if (prevCart.has(productId)) {
                // Если есть, увеличиваем количество
                newCart.set(productId, (prevCart.get(productId) || 0) + 1);
            } else {
                // Если нет, очищаем корзину и добавляем новый продукт
                newCart.set(productId, 1);
            }
            
            return newCart;
        });
        
        // Пересчитываем total
        setCart((currentCart) => {
            const productPrice = PRODUCTS.find((product) => product.id === productId)?.price || 0;
            const quantity = currentCart.get(productId) || 0;
            setTotal(productPrice * quantity);
            return currentCart;
        });
    }

    const handleRemoveOneProduct = (productId: number) => {
        setCart((prevCart) => {
            const newCart = new Map(prevCart);
            const quantity = newCart.get(productId) || 0;
            if (quantity > 1) {
                newCart.set(productId, quantity - 1);
            }
            if (quantity === 1) {
                newCart.delete(productId);
            }
            return newCart;
        });
        
        // Пересчитываем total
        setCart((currentCart) => {
            const productPrice = PRODUCTS.find((product) => product.id === productId)?.price || 0;
            const quantity = currentCart.get(productId) || 0;
            setTotal(productPrice * quantity);
            return currentCart;
        });
    }

    return ( 
        <Card className="col-span-12 sm:col-span-8 md:col-span-7">
            <CardHeader>
                <CardTitle>Магазин товаров</CardTitle>
            </CardHeader>
            <CardContent className="flex flex-col gap-4">
                <h3 className="text-lg font-medium w-full text-left text-gray-800">Выберите товар</h3>
                <div className="grid grid-cols-3 gap-4">
                    <div className="grid grid-cols-1 sm:grid-cols-2 col-span-2 w-full gap-4 h-full items-stretch">
                        {PRODUCTS.map((product) => (
                            <ProductCard key={product.id} name={product.name} description={product.description} price={product.price} onAddToCart={() => handleAddToCart(product.id)} />
                        ))}
                    </div>
                    <div className="flex flex-col items-start w-full gap-4 col-span-1">
                        <h3 className="text-lg font-semibold w-full text-left text-gray-800">Ваша корзина</h3>
                        <div className="flex flex-col items-start w-full gap-4">
                            {Array.from(cart.entries()).map(([productId, quantity]) => (
                                <div key={productId} className="flex flex-row items-center w-full gap-4 justify-between">
                                    <p className="text-sm text-gray-800 w-2/5">{PRODUCTS.find((product) => product.id === productId)?.name}</p>
                                    <p className="text-sm text-gray-800 w-1/5">{quantity}</p>
                                    <Button variant="outline" size='icon' onClick={() => handleRemoveOneProduct(productId)}>
                                        <Minus className="w-4 h-4" />
                                    </Button>
                                </div>
                            ))}
                            <p className="text-lg font-medium w-full text-left text-gray-800">Итого: {total} руб.</p>
                            <Button size='icon' className="w-full" disabled={total === 0}>
                                Оплатить
                            </Button>
                        </div>
                    </div>
                </div>
            </CardContent>  
        </Card>
    )   
}