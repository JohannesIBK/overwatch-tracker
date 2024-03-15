import { GameResult, Role } from "@/utils/game";

export interface Game {
  id: string;
  note: string | null;
  rank_adjustment: number;
  replay_id: string | null;
  result: GameResult;
  role: Role;
  played_at: Date;
  images: Image[];
}

export interface CreateGamePayload {
  note: string | null;
  rank_adjustment: number;
  replay_id: string | null;
  result: GameResult;
  role: Role;
  played_at: string;
  stats_url: string | null;
}

export interface Image {
  id: number;
  url: string;
}
