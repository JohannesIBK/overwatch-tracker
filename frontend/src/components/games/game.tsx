import { Game as GameInterface } from "@/types/games";
import { Accordion, Badge, Center, Group, Image, Paper } from "@mantine/core";
import { Carousel } from "@mantine/carousel";
import { roleToColor } from "@/utils/game";

const intl = new Intl.DateTimeFormat("en-GB", {
  month: "numeric",
  day: "numeric",
  hour: "numeric",
  minute: "numeric",
});

function Game({ game }: { game: GameInterface }) {
  return (
    <Accordion.Item key={game.id} value={game.id}>
      <Accordion.Control>
        <Group justify={"space-between"} px={"sm"}>
          <Center>
            <Group gap={"xs"}>
              <Badge color={roleToColor(game.role)}>{game.role.toUpperCase()}</Badge>
              <Badge color={game.result === "win" ? "teal" : "red"}>{game.result.toUpperCase()}</Badge>
              <Badge>{game.rank_adjustment} SR</Badge>
            </Group>
          </Center>

          <Center>
            <Group gap={"xs"}>
              <Badge color={"gray"}>{intl.format(game.played_at)}</Badge>
            </Group>
          </Center>
        </Group>
      </Accordion.Control>
      <Accordion.Panel>
        <Paper>{game.note}</Paper>

        <Carousel withIndicators height={500}>
          {game.images.map((image) => (
            <Center key={image.id} w={"100%"}>
              <Image src={image.url} mah={500} alt="game screenshot" />
            </Center>
          ))}
        </Carousel>
      </Accordion.Panel>
    </Accordion.Item>
  );
}

export default Game;
