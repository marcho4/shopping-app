import { clsx, type ClassValue } from "clsx"
import { ApiError } from "./app-service";
import { twMerge } from "tailwind-merge"

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}

export function isApiError(error: unknown): error is ApiError {
    return error instanceof ApiError;
}

export function getErrorMessage(error: unknown): string {
    if (isApiError(error)) {
        return error.errorResponse?.message || error.message;
    }

    if (error instanceof Error) {
        return error.message;
    }

    return 'Произошла неизвестная ошибка';
}