import { useGames } from "@/utils/hooks";
import { Accordion, Badge, Box, Group, ScrollArea, Title } from "@mantine/core";
import Game from "@/components/games/game";
import { useEffect, useMemo, useRef, useState } from "react";
import { GameResult } from "@/utils/game";
import type { Game as GamesType } from "@/types/games";
import GamesFilter from "@/components/games/games-filter";

function GamesList() {
  const scrollRef = useRef<HTMLDivElement>(null);
  const [games, setGames] = useState<GamesType[]>([]);

  const { error, isLoading, data, fetchNextPage } = useGames();

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

  const apiGames = useMemo(() => {
    return data?.pages.flat() || [];
  }, [data]);

  // Only set on first time
  useEffect(() => {
    if (apiGames && games.length === 0) {
      setGames(apiGames);
    }

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [apiGames]);

  if (isLoading) return <div>Loading...</div>;
  if (error) return <div>Error: {JSON.stringify(error)}</div>;

  return (
    <Box mt={"md"}>
      <GamesFilter setGames={setGames} games={apiGames!} />

      <Box>
        <Title order={5}>Games</Title>
        <Group>
          <Badge>Win-Rate: {winRate}</Badge>
          <Badge>Games: {games.length}</Badge>
          <Badge>SR: {sr}</Badge>
        </Group>
      </Box>

      <ScrollArea.Autosize
        mah={"65dvh"}
        onScrollPositionChange={(pos) => {
          if (pos.y === scrollRef.current!.scrollHeight - scrollRef.current!.clientHeight) {
            fetchNextPage().then(() => {
              console.log("Fetched next page");
            });
          }
        }}
        viewportRef={scrollRef}>
        <Accordion>
          {games.map((game) => (
            <Game key={game.id} game={game} />
          ))}
        </Accordion>
      </ScrollArea.Autosize>
    </Box>
  );
}

export default GamesList;
