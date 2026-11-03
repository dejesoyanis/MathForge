import Link from "next/link";

export default function NotFound() {
  return (
    <section className="max-w-xl">
      <h1 className="text-3xl">Seite nicht gefunden</h1>
      <p className="mt-3 text-muted">Diese Adresse gehört nicht zu MathForge.</p>
      <Link href="/" className="mt-6 inline-block text-accent underline">
        Zur Übersicht
      </Link>
    </section>
  );
}
