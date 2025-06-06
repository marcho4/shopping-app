"use client";

import { useState } from "react";
import { Card, CardContent, CardHeader, CardTitle } from "./ui/card";
import { Input } from "./ui/input";
import { Button } from "./ui/button";

export default function BankAccount() {
    const [userId, setUserId] = useState<number | null>(null);
    
    return ( 
        <Card className="col-span-12 sm:col-span-4 md:col-span-5">
            <CardHeader>
                <CardTitle>Банковский счет</CardTitle>
            </CardHeader>
            <CardContent className="flex flex-col gap-y-2">
                <p>Введите ваш ID, чтобы получить информацию о вашем счете</p>
                <Input type="number" className="w-64" placeholder="Введите ID пользователя" value={userId || undefined} onChange={(e) => setUserId(Number(e.target.value))} />
                <Button variant="default" className="w-32">Подтвердить</Button>
            </CardContent>
        </Card>
    )
}       