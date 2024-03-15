"use client";

import GamesList from "@/components/games/games-list";
import { Container, Title } from "@mantine/core";
import AddGameButton from "@/components/games/add-game-button";
import GamesFilter from "@/components/games/games-filter";

function Page() {
  return (
    <Container>
      <Title order={2}>Game Page</Title>

      <AddGameButton className={"mb-2"} />
      <GamesList />
    </Container>
  );
}

export default Page;
