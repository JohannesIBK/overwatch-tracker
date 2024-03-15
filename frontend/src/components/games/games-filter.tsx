import { Game } from "@/types/games";
import { UseState } from "@/types/react";
import { Box, Chip, ChipGroup, Group, Stack, Title } from "@mantine/core";
import { useState } from "react";
import { Role } from "@/utils/game";

function GamesFilter({ setGames, games }: { setGames: UseState<Game[]>; games: Game[] }) {
  const [value, setValue] = useState<string>("all");
  const [time, setTime] = useState<number | null>(0);

  return (
    <Box>
      <Title order={5}>Filter games</Title>

      <Stack>
        <ChipGroup
          value={time?.toString() || "0"}
          onChange={(val) => {
            let parsed = parseInt(val as string);

            setTime(parsed === 0 ? null : parsed);

            const filteredGames = filterGames([...games], { days: parsed, role: value });
            setGames(filteredGames);
          }}>
          <Group my={10}>
            <Chip value={"0"}>All</Chip>
            <Chip value={"1"}>1d</Chip>
            <Chip value={"7"}>7d</Chip>
            <Chip value={"30"}>30d</Chip>
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
  console.log(games.length);

  const { days, role } = filter;

  if (days) {
    const date = new Date();
    date.setDate(date.getDate() - days);

    games = games.filter((game) => game.played_at > date);
  }

  if (role !== "all") {
    games = games.filter((game) => game.role === role);
  }

  return games;
}
