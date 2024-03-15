"use client";

import { useCurrentUser, useUserStore } from "@/utils/hooks";
import { useState } from "react";
import { Box, Button } from "@mantine/core";
import AddGame from "@/components/games/add-game";
import { observer } from "mobx-react-lite";

function AddGameButton({ className }: { className?: string }) {
  const userStore = useUserStore();
  const currentUser = useCurrentUser();

  const [open, setOpen] = useState(false);

  const isSameUser = userStore.user?.id === currentUser;

  if (!isSameUser) {
    return null;
  }

  return (
    <Box className={className} mt={5}>
      <Button onClick={() => setOpen(true)} color="teal">
        Add Game
      </Button>

      <AddGame open={open} setOpen={setOpen} />
    </Box>
  );
}

export default observer(AddGameButton);
