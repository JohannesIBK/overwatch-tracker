"use client";

import { useContext } from "react";
import { UserStoreContext } from "@/store/store";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { useParams } from "next/navigation";
import { createGame, fetchGames } from "@/utils/requests";
import { CreateGamePayload, Game } from "@/types/games";

export function useCurrentUser() {
  const { id } = useParams();

  return id as string;
}

export function useUserStore() {
  return useContext(UserStoreContext);
}

export function useGames() {
  const id = useCurrentUser();

  return useQuery({ queryKey: ["games", id], queryFn: () => fetchGames(id) });
}

export function useNewGame() {
  const id = useCurrentUser();
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (payload: CreateGamePayload) => createGame(payload),
    onSuccess: (data) => {
      queryClient.setQueryData<Game[]>(["games", id], (oldData) => {
        if (!oldData) return [];

        return [data, ...oldData];
      });
    },
  });
}
