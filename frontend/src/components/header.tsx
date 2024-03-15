"use client";

import { useUserStore } from "@/utils/hooks";
import { Button } from "@mantine/core";
import { useEffect } from "react";
import { observer } from "mobx-react-lite";
import { AuthStatus } from "@/store/store";
import Link from "next/link";

function Header() {
  const userStore = useUserStore();

  useEffect(() => {
    if (userStore.authStatus === AuthStatus.Unauthenticated) {
      userStore.fetchUser();
    }

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return (
    <div className={"w-full h-12 border-b border-gray-500 flex items-center justify-center"}>
      <div className={"flex items-center justify-between container"}>
        <Link href={"/"}>Home</Link>

        {userStore.user ? (
          <div>
            <Link href={`/games/${userStore.user.id}`}>{userStore.user.username}</Link>
            <Button onClick={() => userStore.logout()}>Logout</Button>
          </div>
        ) : (
          <div>
            <Button component={Link} href={"/login"}>
              Login
            </Button>
          </div>
        )}
      </div>
    </div>
  );
}

export default observer(Header);
