"use client";

import Link from "next/link";
import { useRouter } from "next/navigation";
import { useEffect, useState } from "react";
import { errorText, listExamAttempts, startExam } from "@/lib/api";
import { examScoreText } from "@/lib/score";
import type { ExamAttemptSummary } from "@/lib/types";
import { ErrorNote, Loading } from "./Status";

export function ExamHome() {
  const router = useRouter();
  const [attempts, setAttempts] = useState<ExamAttemptSummary[] | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [pending, setPending] = useState(false);

  useEffect(() => {
    let active = true;
    listExamAttempts()
      .then((data) => {
        if (active) setAttempts(data);
      })
      .catch((caught: unknown) => {
        if (active) setError(errorText(caught));
      });
    return () => {
      active = false;
    };
  }, []);

  async function begin() {
    setPending(true);
    setError(null);
    try {
      const exam = await startExam();
      router.push(`/exam/${exam.id}`);
    } catch (caught) {
      setError(errorText(caught));
      setPending(false);
    }
  }

  if (error && !attempts) return <ErrorNote message={error} />;
  if (!attempts) return <Loading label="Prüfungen werden geladen…" />;

  return (
    <div className="max-w-3xl space-y-6">
      <h1 className="text-4xl">Prüfung</h1>
      <p className="leading-7">
        Zehn automatisch bewertete Aufgaben, eine aus jeder Lektion. Der Score zählt nur diese Aufgaben. Freitextbeweise sind nicht Teil der Prüfung und werden nie automatisch als korrekt gewertet.
      </p>
      <p className="text-sm text-muted">Zwischeneingaben bleiben im Browser, bis du die Prüfung abschickst. Danach liegen Antworten und Auswertung in der lokalen Datenbank.</p>
      <button type="button" className="rounded-md bg-accent px-4 py-2 font-semibold text-white disabled:opacity-60" onClick={begin} disabled={pending}>
        {pending ? "Starte…" : "Prüfung starten"}
      </button>
      {error ? <ErrorNote message={error} /> : null}
      <section>
        <h2 className="text-2xl">Bisherige Versuche</h2>
        {attempts.length === 0 ? <p className="mt-2 text-muted">Noch keine Prüfungsversuche.</p> : null}
        <ul className="mt-3 space-y-2">
          {attempts.map((attempt) => (
            <li key={attempt.id}>
              <Link href={`/exam/${attempt.id}`} className="block rounded-lg border border-line bg-card px-4 py-3 hover:border-accent">
                <span className="font-semibold">{attempt.status}</span>
                <span className="text-muted"> · {attempt.started_at}</span>
                {attempt.auto_correct !== null && attempt.auto_total !== null ? (
                  <span className="mt-1 block">{examScoreText(attempt.auto_correct, attempt.auto_total)}</span>
                ) : (
                  <span className="mt-1 block">Noch nicht abgeschickt</span>
                )}
              </Link>
            </li>
          ))}
        </ul>
      </section>
    </div>
  );
}
