import axios from "axios";
import { API_URL } from "@/environment";
import { CreateGamePayload, Game } from "@/types/games";

export async function fetchGames(id: string, page: number = 0) {
  const response = await axios.get<Game[]>(`${API_URL}/game/user/${id}?page=${page}`, { withCredentials: true });

  return response.data.map((game) => ({
    ...game,
    played_at: new Date(game.played_at),
  }));
}

export async function createGame(payload: CreateGamePayload) {
  const response = await axios.put<Game>(`${API_URL}/game`, payload, { withCredentials: true });

  response.data.played_at = new Date(response.data.played_at);

  return response.data;
}
