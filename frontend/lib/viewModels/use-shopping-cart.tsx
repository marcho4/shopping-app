"use client";

import { useState, useMemo } from "react";
import { PRODUCTS } from "@/lib/constants";

export interface CartItem {
  product_id: number;
  amount: number;
  product_price: number;
}

interface CartState {
  productId: number;
  quantity: number;
}

export function useShoppingCart() {
  const [cart, setCart] = useState<CartState | null>(null);

  const cartItem = useMemo((): CartItem | null => {
    if (!cart) return null;

    const product = PRODUCTS.find(p => p.id === cart.productId);
    return {
      product_id: cart.productId,
      amount: cart.quantity,
      product_price: product?.price || 0
    };
  }, [cart]);

  const clearCart = () => {
    setCart(null);
  };

  const total = useMemo(() => {
    return cartItem ? cartItem.amount * cartItem.product_price : 0;
  }, [cartItem]);

  const itemCount = useMemo(() => {
    return cartItem?.amount || 0;
  }, [cartItem]);

  const addToCart = (productId: number) => {
    setCart(prevCart => {
      if (prevCart && prevCart.productId === productId) {
        return {
          productId,
          quantity: prevCart.quantity + 1
        };
      }
      return {
        productId,
        quantity: 1
      };
    });
  };

  const removeOneFromCart = (productId: number) => {
    setCart(prevCart => {
      if (!prevCart || prevCart.productId !== productId) {
        return prevCart;
      }

      if (prevCart.quantity > 1) {
        return {
          productId,
          quantity: prevCart.quantity - 1
        };
      }

      return null;
    });
  };

  const getItemQuantity = (productId: number): number => {
    return cart && cart.productId === productId ? cart.quantity : 0;
  };

  const isInCart = (productId: number): boolean => {
    return cart ? cart.productId === productId : false;
  };

  return {
    cartItem,
    total,
    itemCount,
    addToCart,
    removeOneFromCart,
    getItemQuantity,
    isInCart,
    isEmpty: cart === null,
    clearCart
  };
}
