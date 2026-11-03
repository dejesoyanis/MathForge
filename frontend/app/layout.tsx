import type { Metadata } from "next";
import "katex/dist/katex.min.css";
import "./globals.css";
import { Nav } from "@/components/Nav";

export const metadata: Metadata = {
  title: "MathForge — Mengenlehre",
  description: "Lokale Lernplattform für Mengenlehre auf Universitätsniveau.",
};

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="de">
      <body>
        <Nav />
        <main id="inhalt" className="mx-auto max-w-5xl px-4 py-8">
          {children}
        </main>
      </body>
    </html>
  );
}
