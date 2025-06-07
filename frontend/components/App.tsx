"use client";

import dynamic from 'next/dynamic';
import { Input } from '@/components/ui/input';
import { GlobalUserIdProvider, useGlobalUserId } from '@/lib/viewModels/use-global-user-id';

const Market = dynamic(() => import("@/components/Market"), { ssr: false });
const BankAccount = dynamic(() => import("@/components/BankAccount"), { ssr: false });
const Orders = dynamic(() => import("@/components/Orders"), { ssr: false });

function AppContent() {
  const { userId, setUserId } = useGlobalUserId();

  return (
    <div className="container mx-auto mt-10 flex flex-col items-center justify-items-center min-h-screen gap-16  px-10">
      <main className="flex flex-col items-start w-full gap-4">
        <div className="flex flex-row w-full items-center gap-1.5 justify-between">
            <h1 className="text-4xl font-bold w-full text-left">Приложение для оплаты заказов</h1>
            <div className="flex flex-row w-full max-w-sm items-center gap-1.5">
            <label htmlFor="user_id">User ID</label>
            <Input
                id="user_id"
                type="number"
                placeholder="User ID"
                className="w-full max-w-64"
                value={userId}
                onChange={(e) => setUserId(Number(e.target.value))}
            />
            </div>
        </div>
        
        <div className="grid grid-cols-12 items-start w-full gap-4">
          <Market />
          <div className="col-span-12 sm:col-span-4 md:col-span-5 flex flex-col gap-y-4">
            <BankAccount />
            <Orders />
          </div>
        </div>
      </main>
    </div>
  )
}

export default function App() {
  return (
    <GlobalUserIdProvider>
      <AppContent />
    </GlobalUserIdProvider>
  );
}
