"use client";

import React from "react";
import { QueryClient } from "@tanstack/query-core";
import { QueryClientProvider } from "@tanstack/react-query";
import { Notifications } from "@mantine/notifications";
import { UserStoreContext, userStoreInstance } from "@/store/store";

export function Provider({ children }: { children: React.ReactNode }) {
  const [queryClient] = React.useState(() => {
    return new QueryClient({
      defaultOptions: {
        queries: {
          refetchOnWindowFocus: false,
          retry: false,
          gcTime: 1000 * 60 * 10,
        },
      },
    });
  });

  return (
    <QueryClientProvider client={queryClient}>
      <UserStoreContext.Provider value={userStoreInstance}>{children}</UserStoreContext.Provider>
      <Notifications />
    </QueryClientProvider>
  );
}
