import { Button } from "./ui/button";

interface ProductCardProps {
    name: string;
    description: string;
    price: number;
    onAddToCart: () => void;
}

export default function ProductCard({ name, description, price, onAddToCart }: ProductCardProps) {
    return (
        <div className="flex flex-col justify-between w-full max-w-[300px] h-full gap-4 bg-background text-foreground p-4 rounded-md border-2">
            <div className="grid grid-cols-1 items-start w-full gap-4 justify-between h-full">
                <h3 className="text-lg font-bold w-full text-left">{name}</h3>
                <div className="flex flex-col items-start w-full gap-2">
                    <p className="text-sm text-foreground w-full text-left">{description}</p>
                </div>
                <div className="flex flex-col items-start w-full gap-2">
                    <p className="text-sm text-foreground w-full text-left">Цена: {price} руб.</p>
                </div>
                <Button variant="default" className="w-full" onClick={onAddToCart}>
                    Купить
                </Button>
            </div>
        </div>
    )
}