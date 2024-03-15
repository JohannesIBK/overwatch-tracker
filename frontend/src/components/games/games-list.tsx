import { useGames } from "@/utils/hooks";
import { Accordion, Badge, Box, Group, Title } from "@mantine/core";
import Game from "@/components/games/game";
import { useEffect, useMemo, useState } from "react";
import { GameResult } from "@/utils/game";
import type { Game as GamesType } from "@/types/games";
import GamesFilter from "@/components/games/games-filter";

function GamesList() {
  const [games, setGames] = useState<GamesType[]>([]);

  const { error, isLoading, data } = useGames();

  const winRate = useMemo(() => {
    if (!games.length) return "0.0%";

    const wins = games.filter((game) => game.result === GameResult.Win).length;
    const total = games.length;

    return `${((wins / total) * 100).toFixed(1)}%`;
  }, [games]);

  const sr = useMemo(() => {
    return games
      .map((game) => game.rank_adjustment)
      .reduce((acc, cur) => acc + cur, 0)
      .toString();
  }, [games]);

  useEffect(() => {
    if (data) {
      setGames(data);
    }
  }, [data]);

  if (isLoading) return <div>Loading...</div>;
  if (error) return <div>Error: {JSON.stringify(error)}</div>;

  return (
    <Box mt={"md"} mb={"100vh"}>
      <GamesFilter setGames={setGames} games={data!} />

      <Box>
        <Title order={5}>Games</Title>
        <Group mb={10}>
          <Badge>Win-Rate: {winRate}</Badge>
          <Badge>Games: {games.length}</Badge>
          <Badge>SR: {sr}</Badge>
        </Group>
      </Box>

      <Accordion>
        {games.map((game) => (
          <Game key={game.id} game={game} />
        ))}
      </Accordion>
    </Box>
  );
}

export default GamesList;
