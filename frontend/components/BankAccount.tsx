"use client";

import { Card, CardContent, CardHeader, CardTitle } from "./ui/card";
import { Input } from "./ui/input";
import { Button } from "./ui/button";
import { useBankAccount } from "@/lib/viewModels/useBankAccount";
import { useGlobalUserId } from "@/lib/viewModels/use-global-user-id";
import {RefreshCcwIcon} from "lucide-react";

export default function BankAccount() {
    const { userId } = useGlobalUserId();
    const vm = useBankAccount();

    return (
        <Card className="col-span-12 lg:col-span-5">
            <CardHeader>
                <div className="flex flex-row w-full items-center gap-1.5 justify-between">
                    <CardTitle>Банковский счет</CardTitle>
                    {userId && (
                        <Button variant="outline" size="icon" onClick={() => vm.fetchAccount()}>
                            <RefreshCcwIcon className="w-4 h-4" />
                        </Button>
                    )}
                </div>
            </CardHeader>
            <CardContent className="flex flex-col gap-y-2">
                {userId ? (
                    <div className="flex flex-col gap-y-2">
                        {!vm.accountId ? (<div className='flex flex-col items-center'>
                            <Button className=' w-52'
                                    onClick={() => vm.createBankAccount()}>
                                Создать банковский аккаунт
                            </Button>
                        </div>) : (
                            <div className="flex flex-col w-full gap-4">
                                <div>
                                    <p>{vm.balance || 0} рублей</p>
                                </div>
                                <div className="flex flex-row gap-x-2 items-center justify-between">
                                    <Input
                                        type="number"
                                        className="w-64"
                                        placeholder="Введите сумму для пополнения"
                                        value={vm.depAmount || undefined}
                                        onChange={(e) => vm.setDepAmount(Number(e.target.value))}/>
                                    <Button
                                        variant="default"
                                        className="w-32"
                                        onClick={() => vm.deposit()}>
                                        Пополнить
                                    </Button>
                                </div>
                            </div>
                        )}
                    </div>
                ) : (
                    <>
                        <p>Введите ваш ID, чтобы получить информацию о вашем счете</p>
                    </>
                )}
                {vm.error && <p className="text-red-500">{vm.error}</p>}
            </CardContent>
        </Card>
    )
}