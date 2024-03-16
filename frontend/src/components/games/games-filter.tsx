import { Game } from "@/types/games";
import { UseState } from "@/types/react";
import { Box, Chip, ChipGroup, Group, Stack, Title } from "@mantine/core";
import { useState } from "react";
import { Role } from "@/utils/game";

function GamesFilter({ setGames, games }: { setGames: UseState<Game[]>; games: Game[] }) {
  const [value, setValue] = useState<string>("all");
  const [time, setTime] = useState<number | null>(null);

  return (
    <Box>
      <Title order={5}>Filter games</Title>

      <Stack>
        <ChipGroup
          value={JSON.stringify(time)}
          onChange={(val) => {
            let parsed: number | null = parseInt(val as string);
            parsed = isNaN(parsed) ? null : parsed;

            setTime(parsed);

            const filteredGames = filterGames([...games], { days: parsed, role: value });
            setGames(filteredGames);
          }}>
          <Group my={10}>
            <Chip value={"null"}>All</Chip>
            <Chip value={"0"}>Today</Chip>
            <Chip value={"6"}>7d</Chip>
            <Chip value={"29"}>30d</Chip>
          </Group>
        </ChipGroup>

        <ChipGroup
          value={value}
          onChange={(val) => {
            setValue(val as string);

            const filteredGames = filterGames([...games], { days: time, role: val as string });
            setGames(filteredGames);
          }}>
          <Group my={10}>
            <Chip value={"all"} color={"lime"}>
              All
            </Chip>
            <Chip value={Role.Support} color={"yellow.9"}>
              Support
            </Chip>
            <Chip value={Role.Tank} color={"blue"}>
              Tank
            </Chip>
            <Chip value={Role.Damage} color={"teal"}>
              DPS
            </Chip>
            <Chip value={Role.OpenQueue} color={"gray"}>
              Open Queue
            </Chip>
          </Group>
        </ChipGroup>
      </Stack>
    </Box>
  );
}

export default GamesFilter;

function filterGames(games: Game[], filter: { days: number | null; role: string }) {
  const { days, role } = filter;

  if (days != null) {
    const date = new Date();
    date.setSeconds(0);
    date.setMinutes(0);
    date.setHours(0);

    date.setDate(date.getDate() - days);

    games = games.filter((game) => game.played_at > date);
  }

  if (role !== "all") {
    games = games.filter((game) => game.role === role);
  }

  return games;
}
