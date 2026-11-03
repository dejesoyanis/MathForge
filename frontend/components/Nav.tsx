"use client";

import Link from "next/link";
import { usePathname } from "next/navigation";

const LINKS = [
  { href: "/", label: "Übersicht" },
  { href: "/lab", label: "Mengenlabor" },
  { href: "/exam", label: "Prüfung" },
];

export function Nav() {
  const pathname = usePathname();
  return (
    <header className="border-b border-line bg-card">
      <a className="skip-link" href="#inhalt">
        Zum Inhalt
      </a>
      <div className="mx-auto flex max-w-5xl flex-wrap items-center justify-between gap-3 px-4 py-4">
        <Link href="/" className="serif text-2xl text-accent">
          MathForge
        </Link>
        <nav aria-label="Hauptnavigation" className="flex flex-wrap gap-2">
          {LINKS.map((link) => {
            const active = link.href === "/" ? pathname === "/" : pathname.startsWith(link.href);
            return (
              <Link
                key={link.href}
                href={link.href}
                aria-current={active ? "page" : undefined}
                className={`rounded-full px-3 py-1.5 text-sm ${
                  active ? "bg-accent text-white" : "text-ink hover:bg-paper"
                }`}
              >
                {link.label}
              </Link>
            );
          })}
        </nav>
      </div>
    </header>
  );
}
