"use client";

import Link from "next/link";
import { useEffect, useState } from "react";
import { errorText, getLessons, getProgress, resetProgress } from "@/lib/api";
import { lessonWorkText } from "@/lib/score";
import type { LessonSummary, ProgressResponse } from "@/lib/types";
import { ErrorNote, Loading } from "./Status";

export function Dashboard() {
  const [lessons, setLessons] = useState<LessonSummary[] | null>(null);
  const [progress, setProgress] = useState<ProgressResponse | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [resetting, setResetting] = useState(false);

  async function load() {
    setError(null);
    try {
      const [lessonData, progressData] = await Promise.all([getLessons(), getProgress()]);
      setLessons(lessonData.lessons);
      setProgress(progressData);
    } catch (caught) {
      setError(errorText(caught));
    }
  }

  useEffect(() => {
    let active = true;
    Promise.all([getLessons(), getProgress()])
      .then(([lessonData, progressData]) => {
        if (!active) return;
        setLessons(lessonData.lessons);
        setProgress(progressData);
      })
      .catch((caught: unknown) => {
        if (active) setError(errorText(caught));
      });
    return () => {
      active = false;
    };
  }, []);

  async function onReset() {
    if (!window.confirm("Fortschritt, Antworten und Prüfungsversuche lokal löschen?")) return;
    setResetting(true);
    try {
      await resetProgress();
      await load();
    } catch (caught) {
      setError(errorText(caught));
    } finally {
      setResetting(false);
    }
  }

  if (error) return <ErrorNote message={error} />;
  if (!lessons || !progress) return <Loading label="Lektionen werden geladen…" />;

  const next = progress.next_lesson_id;
  return (
    <div className="space-y-8">
      <div className="max-w-3xl">
        <h1 className="text-4xl">Mengenlehre</h1>
        <p className="mt-3 text-lg leading-8 text-muted">
          Zehn aufeinander aufbauende Lektionen, ein Mengenlabor und eine kurze Prüfung. Alles bleibt auf diesem Rechner.
        </p>
      </div>
      <section aria-label="Lernstand" className="grid gap-3 sm:grid-cols-3">
        <Stat label="Bearbeitete Übungen" value={String(progress.exercises_attempted)} />
        <Stat label="Richtige Antworten" value={String(progress.exercises_correct)} />
        <Stat label="Abgeschlossene Lektionen" value={`${progress.lessons_completed} von ${progress.lessons_total}`} />
      </section>
      <p className="max-w-3xl text-sm text-muted">{progress.score_note}</p>
      <div className="flex flex-wrap gap-3">
        {next ? (
          <Link href={`/lessons/${next}`} className="rounded-md bg-accent px-4 py-2 font-semibold text-white">
            Weiterlernen
          </Link>
        ) : (
          <Link href="/exam" className="rounded-md bg-accent px-4 py-2 font-semibold text-white">
            Zur Prüfung
          </Link>
        )}
        <button type="button" className="rounded-md border border-line px-4 py-2" onClick={onReset} disabled={resetting}>
          {resetting ? "Lösche…" : "Fortschritt zurücksetzen"}
        </button>
      </div>
      {lessons.length === 0 ? <p>Keine Lektionen vorhanden.</p> : null}
      <ol className="grid gap-3">
        {lessons.map((lesson) => {
          const item = progress.lessons.find((entry) => entry.lesson_id === lesson.id);
          const status = item?.completed ? "abgeschlossen" : item && item.attempted > 0 ? "in Arbeit" : "offen";
          return (
            <li key={lesson.id}>
              <Link href={`/lessons/${lesson.id}`} className="block rounded-lg border border-line bg-card p-4 hover:border-accent">
                <div className="flex flex-wrap items-baseline justify-between gap-2">
                  <h2 className="text-xl">
                    {lesson.order}. {lesson.title}
                  </h2>
                  <span className="text-sm text-muted">{status}</span>
                </div>
                <p className="mt-1 text-muted">{lesson.description}</p>
                <p className="mt-2 text-sm">
                  {lesson.difficulty}
                  {" · "}
                  {lessonWorkText(item?.attempted ?? 0, lesson.exercise_count)}
                </p>
              </Link>
            </li>
          );
        })}
      </ol>
    </div>
  );
}

function Stat({ label, value }: { label: string; value: string }) {
  return (
    <div className="rounded-lg border border-line bg-card px-4 py-3">
      <p className="text-sm text-muted">{label}</p>
      <p className="mt-1 text-2xl">{value}</p>
    </div>
  );
}
