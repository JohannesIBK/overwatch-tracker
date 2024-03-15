import "./globals.css";
import "@mantine/notifications/styles.css";
import "@mantine/carousel/styles.css";
import "@mantine/core/styles.css";

import type { Metadata } from "next";
import { Inter } from "next/font/google";
import React from "react";
import { Provider } from "@/app/_provider";
import { MantineProvider } from "@mantine/core";
import Header from "@/components/header";

const inter = Inter({ subsets: ["latin"] });

export const metadata: Metadata = {
  title: "Overwatch Stats Overview",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body className={inter.className}>
        <MantineProvider forceColorScheme={"dark"}>
          <Header />
          <Provider>{children}</Provider>
        </MantineProvider>
      </body>
    </html>
  );
}
