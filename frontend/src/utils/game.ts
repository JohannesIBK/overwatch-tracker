export enum GameResult {
  Win = "win",
  Lose = "lose",
  Draw = "draw",
}

export enum Role {
  Support = "support",
  Tank = "tank",
  Damage = "damage",
  OpenQueue = "open_queue",
}

export const ROLE_SELECT_OPTIONS = [
  { value: Role.Support, label: "Support" },
  { value: Role.Tank, label: "Tank" },
  { value: Role.Damage, label: "Damage" },
  { value: Role.OpenQueue, label: "Open Queue" },
];

export const GAME_RESULT_OPTIONS = [
  { value: GameResult.Win, label: "Win" },
  { value: GameResult.Lose, label: "Lose" },
  { value: GameResult.Draw, label: "Draw" },
];

export function roleToColor(role: Role) {
  switch (role) {
    case Role.Support:
      return "yellow.9";
    case Role.Tank:
      return "blue";
    case Role.Damage:
      return "teal";
    case Role.OpenQueue:
      return "gray";
  }
}
