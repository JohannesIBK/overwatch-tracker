"use client";

import { useContext } from "react";
import { UserStoreContext } from "@/store/store";
import { useInfiniteQuery, useMutation, useQueryClient } from "@tanstack/react-query";
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

  return useInfiniteQuery({
    queryKey: ["games", id],
    initialPageParam: 0,
    queryFn: ({ pageParam }) => fetchGames(id, pageParam),
    getNextPageParam: (page, _, currentIndex) => {
      if (page.length < 50) return undefined;

      return currentIndex + 1;
    },
  });
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
