"use client";

import Link from "next/link";
import { useEffect, useState } from "react";
import { completeLesson, errorText, getLesson, getProgress } from "@/lib/api";
import type { LessonDetail, ProgressResponse } from "@/lib/types";
import { Blocks } from "./Blocks";
import { ExerciseCard } from "./ExerciseCard";
import { MathText } from "./MathText";
import { ErrorNote, Loading } from "./Status";

const SECTIONS = [
  ["erklaerung", "Erklärung"],
  ["beispiele", "Beispiele"],
  ["uebungen", "Übungen"],
  ["zusammenfassung", "Zusammenfassung"],
] as const;

type SectionId = (typeof SECTIONS)[number][0];

export function LessonView({ id }: { id: string }) {
  const [lesson, setLesson] = useState<LessonDetail | null>(null);
  const [progress, setProgress] = useState<ProgressResponse | null>(null);
  const [section, setSection] = useState<SectionId>("erklaerung");
  const [error, setError] = useState<string | null>(null);
  const [pending, setPending] = useState(false);

  async function loadProgress() {
    const data = await getProgress();
    setProgress(data);
  }

  useEffect(() => {
    let active = true;
    Promise.all([getLesson(id), getProgress()])
      .then(([lessonData, progressData]) => {
        if (!active) return;
        setLesson(lessonData);
        setProgress(progressData);
      })
      .catch((caught: unknown) => {
        if (active) setError(errorText(caught));
      });
    return () => {
      active = false;
    };
  }, [id]);

  if (error) return <ErrorNote message={error} />;
  if (!lesson || !progress) return <Loading label="Lektion wird geladen…" />;

  const item = progress.lessons.find((entry) => entry.lesson_id === lesson.id);
  const ready = Boolean(item && item.attempted >= lesson.exercises.length && !item.completed);
  const attempts = new Map(progress.latest_attempts.map((attempt) => [attempt.exercise_id, attempt]));

  async function finish() {
    setPending(true);
    setError(null);
    try {
      await completeLesson(lesson!.id);
      await loadProgress();
    } catch (caught) {
      setError(errorText(caught));
    } finally {
      setPending(false);
    }
  }

  return (
    <article className="max-w-3xl">
      <p className="text-sm text-muted">
        <Link href="/" className="underline">
          Übersicht
        </Link>
        {" · Lektion "}
        {lesson.order}
        {" · "}
        {lesson.difficulty}
      </p>
      <h1 className="mt-2 text-4xl">{lesson.title}</h1>
      <p className="mt-3 text-lg text-muted">{lesson.description}</p>
      <div className="mt-6 flex flex-wrap gap-2" role="tablist" aria-label="Abschnitte">
        {SECTIONS.map(([sectionId, label]) => (
          <button
            key={sectionId}
            type="button"
            role="tab"
            aria-selected={section === sectionId}
            className={`rounded-full px-3 py-1.5 text-sm ${section === sectionId ? "bg-accent text-white" : "border border-line bg-card"}`}
            onClick={() => setSection(sectionId)}
          >
            {label}
          </button>
        ))}
      </div>
      <div className="mt-6" role="tabpanel">
        {section === "erklaerung" ? (
          <div className="space-y-6">
            <section>
              <h2 className="text-2xl">Lernziele</h2>
              <ul className="mt-3 list-disc space-y-2 pl-5">
                {lesson.objectives.map((goal) => (
                  <li key={goal}>
                    <MathText text={goal} />
                  </li>
                ))}
              </ul>
            </section>
            <Blocks blocks={lesson.theory} />
            <section>
              <h2 className="text-2xl">Typische Denkfehler</h2>
              <div className="mt-3 space-y-3">
                {lesson.misconceptions.map((item) => (
                  <div key={item.title} className="rounded-lg border border-line bg-card p-4">
                    <h3 className="text-lg">{item.title}</h3>
                    <p className="mt-1">
                      <MathText text={item.text} />
                    </p>
                  </div>
                ))}
              </div>
            </section>
          </div>
        ) : null}
        {section === "beispiele" ? (
          <div className="space-y-6">
            {lesson.examples.map((example) => (
              <section key={example.title}>
                <h2 className="text-2xl">{example.title}</h2>
                <div className="mt-3">
                  <Blocks blocks={example.blocks} />
                </div>
              </section>
            ))}
          </div>
        ) : null}
        {section === "uebungen" ? (
          <div className="space-y-4">
            <p className="text-sm text-muted">
              {item ? `${item.attempted} von ${lesson.exercises.length} Übungen bearbeitet` : null}
              {item?.completed ? " · abgeschlossen" : ""}
            </p>
            {lesson.exercises.map((exercise) => (
              <ExerciseCard
                key={exercise.id}
                exercise={exercise}
                initial={attempts.get(exercise.id)}
                onSubmitted={() => {
                  loadProgress().catch((caught: unknown) => setError(errorText(caught)));
                }}
              />
            ))}
            {ready ? (
              <button type="button" className="rounded-md bg-clay px-4 py-2 font-semibold text-white disabled:opacity-60" onClick={finish} disabled={pending}>
                {pending ? "Speichere…" : "Lektion abschliessen"}
              </button>
            ) : null}
            {item?.completed ? <p className="text-good">Diese Lektion ist abgeschlossen.</p> : null}
          </div>
        ) : null}
        {section === "zusammenfassung" ? (
          <section>
            <h2 className="text-2xl">Zusammenfassung</h2>
            <div className="mt-3">
              <Blocks blocks={lesson.summary} />
            </div>
          </section>
        ) : null}
      </div>
    </article>
  );
}
