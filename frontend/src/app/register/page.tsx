"use client";

import { useForm } from "@mantine/form";
import { Button, Container, TextInput } from "@mantine/core";
import axios from "axios";
import { API_URL } from "@/environment";
import { useRouter } from "next/navigation";
import { notifications } from "@mantine/notifications";

function Page() {
  const { push } = useRouter();

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
          register(values)
            .then(() => {
              push("/login");
            })
            .catch((err) => {
              notifications.show({
                title: "Error",
                message: `Failed to register: ${err.response.data?.message}`,
              });
            });
        })}>
        <TextInput label={"Username"} {...form.getInputProps("username")} />
        <TextInput label={"Password"} {...form.getInputProps("password")} />

        <Button type="submit">Register</Button>
      </form>
    </Container>
  );
}

export default Page;

async function register(payload: { username: string; password: string }) {
  const response = await axios.post<void>(`${API_URL}/user/register`, payload, { withCredentials: true });

  return response.data;
}
