import Market from "@/components/Market"
import BankAccount from "@/components/BankAccount"
import Orders from "@/components/Orders";

export default function Home() {
  return (
    <div className="flex flex-col items-center justify-items-center min-h-screen p-8 pb-20 gap-16 sm:p-20">
      <main className="flex flex-col items-start w-full gap-4">
        <h1 className="text-4xl font-bold w-full text-left">Приложение для оплаты заказов</h1>
        <div className="grid grid-cols-12 items-start w-full gap-4">
          <Market />
          <div className="col-span-12 sm:col-span-4 md:col-span-5 flex flex-col gap-y-4">
            <BankAccount />
            <Orders />
          </div>
        </div>
      </main>
    </div>
  );
}
