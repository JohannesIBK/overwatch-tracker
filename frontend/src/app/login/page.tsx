"use client";

import { useForm } from "@mantine/form";
import { Button, Container, TextInput } from "@mantine/core";
import axios from "axios";
import { API_URL } from "@/environment";
import { useUserStore } from "@/utils/hooks";
import { useRouter } from "next/navigation";
import { notifications } from "@mantine/notifications";

function Page() {
  const { push } = useRouter();
  const userStore = useUserStore();

  const form = useForm({
    initialValues: {
      username: "",
      password: "",
    },
    validate: {
      username: (value) => value.length < 4 && "Username is too short",
      password: (value) => value.length < 8 && "Password is too short",
    },
  });

  return (
    <Container>
      <form
        onSubmit={form.onSubmit((values) => {
          login(values)
            .then((user) => {
              userStore.setUser(user);
              push("/");
            })
            .catch((res) => {
              notifications.show({
                title: "Error",
                message: res.response.data.error,
                color: "red",
              });
            });
        })}>
        <TextInput label={"Username"} {...form.getInputProps("username")} />
        <TextInput label={"Password"} {...form.getInputProps("password")} />

        <Button type="submit">Login</Button>
      </form>
    </Container>
  );
}

export default Page;

async function login(payload: { username: string; password: string }) {
  const response = await axios.post<{ id: string; username: string }>(`${API_URL}/user/login`, payload, {
    withCredentials: true,
  });

  return response.data;
}
