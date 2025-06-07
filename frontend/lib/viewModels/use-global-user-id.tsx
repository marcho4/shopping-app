"use client";

import {createContext, useContext, useState, ReactNode} from 'react';

type GlobalUserIdContextType = {
  userId: number | undefined;
  setUserId: (id: number | undefined) => void;
};

const GlobalUserIdContext = createContext<GlobalUserIdContextType | undefined>(undefined);

export function GlobalUserIdProvider({ children }: { children: ReactNode }) {
  const [userId, setUserId] = useState<number | undefined>( undefined);

  return (
    <GlobalUserIdContext.Provider value={{ userId, setUserId }}>
      {children}
    </GlobalUserIdContext.Provider>
  );
}

export function useGlobalUserId() {
  const context = useContext(GlobalUserIdContext);
  if (context === undefined) {
    throw new Error('useGlobalUserId must be used within a GlobalUserIdProvider');
  }
  return context;
}