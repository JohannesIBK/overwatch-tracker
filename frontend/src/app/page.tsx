import Link from "next/link";
import { Button, Group } from "@mantine/core";

export default function Home() {
  return (
    <Group>
      <Button component={Link} href={"/login"}>
        Login
      </Button>
      <Button component={Link} href={"/register"}>
        Register
      </Button>
    </Group>
  );
}
