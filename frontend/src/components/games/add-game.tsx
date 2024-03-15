import { Button, Modal, NumberInput, Select, Textarea, TextInput } from "@mantine/core";
import { UseState } from "@/types/react";
import { useForm } from "@mantine/form";
import { GAME_RESULT_OPTIONS, GameResult, Role, ROLE_SELECT_OPTIONS } from "@/utils/game";
import { useNewGame } from "@/utils/hooks";
import { useState } from "react";
import { notifications } from "@mantine/notifications";

function AddGame({ open, setOpen }: { open: boolean; setOpen: UseState<boolean> }) {
  const addGame = useNewGame();
  const [loading, setLoading] = useState(false);

  const form = useForm({
    initialValues: {
      role: "",
      result: "",
      rank_adjustment: 0,
      replay_id: "",
      note: "",
      stats_url: "",
    },
    validate: {
      note: (value) => value.length > 1000 && "Note is too long",
      stats_url: (value) => (!value.startsWith("https://i.imgur.com") || value.length > 50) && "Invalid URL",
      replay_id: (value) => value.length !== 6 && value.length !== 0 && "Invalid replay ID",
      role: (value) => !ROLE_SELECT_OPTIONS.map((option) => option.value).includes(value as any) && "Invalid role",
      result: (value) => !GAME_RESULT_OPTIONS.map((option) => option.value).includes(value as any) && "Invalid result",
      rank_adjustment: (value) => (value < -100 || value > 100) && "Invalid SR adjustment",
    },
  });

  return (
    <Modal title={"Add new game"} opened={open} onClose={() => setOpen(false)}>
      <form
        onSubmit={form.onSubmit((values) => {
          setLoading(true);

          addGame
            .mutateAsync({
              role: values.role as Role,
              result: values.result as GameResult,
              rank_adjustment: values.rank_adjustment,
              note: values.note || null,
              replay_id: values.replay_id || null,
              stats_url: values.stats_url,
              played_at: new Date().toISOString(),
            })
            .then(() => {
              setOpen(false);
              setLoading(false);
              form.reset();
            })
            .catch((e) => {
              notifications.show({
                title: "Error",
                message: e.response?.data?.error,
              });

              setLoading(false);
            });
        })}>
        <Select label={"Role"} data={ROLE_SELECT_OPTIONS} {...form.getInputProps("role")} />
        <NumberInput label={"SR"} {...form.getInputProps("rank_adjustment")} allowDecimal={false} />
        <Select label={"Result"} data={GAME_RESULT_OPTIONS} {...form.getInputProps("result")} />
        <TextInput label={"Replay ID"} {...form.getInputProps("replay_id")} />
        <Textarea label={"Note"} {...form.getInputProps("note")} />
        <TextInput label={"Stats URL"} {...form.getInputProps("stats_url")} />

        <Button type={"submit"} loading={loading}>
          Add Game
        </Button>
      </form>
    </Modal>
  );
}

export default AddGame;
