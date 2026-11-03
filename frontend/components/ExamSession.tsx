"use client";

import Link from "next/link";
import { useEffect, useState } from "react";
import { errorText, getExam, submitExam } from "@/lib/api";
import { examScoreText } from "@/lib/score";
import type { Answer, ExamDetail, Exercise } from "@/lib/types";
import { Formula, MathText } from "./MathText";
import { ErrorNote, Loading } from "./Status";

const INTEGER = /^-?\d+$/;

export function ExamSession({ id }: { id: string }) {
  const [exam, setExam] = useState<ExamDetail | null>(null);
  const [drafts, setDrafts] = useState<Record<string, string>>({});
  const [error, setError] = useState<string | null>(null);
  const [pending, setPending] = useState(false);

  useEffect(() => {
    let active = true;
    getExam(id)
      .then((data) => {
        if (active) setExam(data);
      })
      .catch((caught: unknown) => {
        if (active) setError(errorText(caught));
      });
    return () => {
      active = false;
    };
  }, [id]);

  if (error && !exam) return <ErrorNote message={error} />;
  if (!exam) return <Loading label="Prüfung wird geladen…" />;

  const finished = exam.status === "abgeschlossen" && exam.results;
  async function onSubmit(event: React.FormEvent) {
    event.preventDefault();
    if (!exam) return;
    const answers: { exercise_id: string; answer: Answer }[] = [];
    for (const question of exam.questions) {
      const built = buildAnswer(question.exercise, drafts[question.exercise.id] ?? "");
      if (!built) {
        setError(`Die Antwort für «${question.lesson_title}» fehlt oder hat das falsche Format.`);
        return;
      }
      answers.push({ exercise_id: question.exercise.id, answer: built });
    }
    setPending(true);
    setError(null);
    try {
      setExam(await submitExam(exam.id, answers));
    } catch (caught) {
      setError(errorText(caught));
    } finally {
      setPending(false);
    }
  }

  return (
    <div className="max-w-3xl">
      <p className="text-sm text-muted">
        <Link href="/exam" className="underline">
          Alle Versuche
        </Link>
      </p>
      <h1 className="mt-2 text-4xl">Prüfungsversuch</h1>
      <p className="mt-3 text-sm leading-6 text-muted">{exam.score_note}</p>
      {finished && exam.auto_correct !== null && exam.auto_total !== null ? (
        <p className="mt-4 text-2xl">{examScoreText(exam.auto_correct, exam.auto_total)}</p>
      ) : null}
      {finished ? (
        <ol className="mt-6 space-y-4">
          {exam.questions.map((question, index) => {
            const result = exam.results?.find((item) => item.exercise_id === question.exercise.id);
            return (
              <li key={question.exercise.id} className="rounded-lg border border-line bg-card p-4">
                <p className="text-sm text-muted">
                  {index + 1}. {question.lesson_title}
                </p>
                <h2 className="mt-1 text-lg">
                  <MathText text={question.exercise.prompt} />
                </h2>
                {result ? (
                  <div className="mt-3 space-y-2">
                    <p className={result.correct ? "text-good" : "text-bad"}>{result.correct ? "Richtig" : "Nicht korrekt"}</p>
                    <p>Deine Antwort: {result.your_answer_display}</p>
                    {result.solution_latex ? <Formula latex={result.solution_latex} /> : null}
                    <p>
                      <MathText text={result.explanation} />
                    </p>
                  </div>
                ) : null}
              </li>
            );
          })}
        </ol>
      ) : (
        <form className="mt-6 space-y-5" onSubmit={onSubmit}>
          {exam.questions.map((question, index) => (
            <fieldset key={question.exercise.id} className="rounded-lg border border-line bg-card p-4">
              <legend className="px-1 text-sm text-muted">
                {index + 1}. {question.lesson_title}
              </legend>
              <div className="text-lg">
                <MathText text={question.exercise.prompt} />
              </div>
              <ExamInput
                exercise={question.exercise}
                value={drafts[question.exercise.id] ?? ""}
                onChange={(value) => setDrafts((current) => ({ ...current, [question.exercise.id]: value }))}
              />
            </fieldset>
          ))}
          {error ? <ErrorNote message={error} /> : null}
          <button type="submit" className="rounded-md bg-accent px-4 py-2 font-semibold text-white disabled:opacity-60" disabled={pending}>
            {pending ? "Werte aus…" : "Prüfung abschicken"}
          </button>
        </form>
      )}
    </div>
  );
}

function ExamInput({ exercise, value, onChange }: { exercise: Exercise; value: string; onChange: (value: string) => void }) {
  if (exercise.kind === "true_false") {
    return (
      <div className="mt-3 flex gap-3">
        <label className="flex items-center gap-2">
          <input type="radio" name={exercise.id} checked={value === "true"} onChange={() => onChange("true")} />
          Wahr
        </label>
        <label className="flex items-center gap-2">
          <input type="radio" name={exercise.id} checked={value === "false"} onChange={() => onChange("false")} />
          Falsch
        </label>
      </div>
    );
  }
  if (exercise.kind === "single_choice") {
    return (
      <div className="mt-3 space-y-2">
        {(exercise.options ?? []).map((option, index) => (
          <label key={option} className="flex items-start gap-2">
            <input type="radio" name={exercise.id} checked={value === String(index)} onChange={() => onChange(String(index))} />
            <MathText text={option} />
          </label>
        ))}
      </div>
    );
  }
  if (exercise.kind === "set") {
    return (
      <input
        className="mt-3 w-full rounded-md border border-line bg-white px-3 py-2"
        maxLength={512}
        aria-label="Menge"
        placeholder="{1,2,3}"
        value={value}
        onChange={(event) => onChange(event.target.value)}
      />
    );
  }
  return (
    <input
      className="mt-3 w-40 rounded-md border border-line bg-white px-3 py-2"
      inputMode="numeric"
      maxLength={20}
      aria-label="Zahl"
      value={value}
      onChange={(event) => onChange(event.target.value)}
    />
  );
}

function buildAnswer(exercise: Exercise, raw: string): Answer | null {
  if (exercise.kind === "true_false") {
    if (raw !== "true" && raw !== "false") return null;
    return { type: "true_false", value: raw === "true" };
  }
  if (exercise.kind === "single_choice") {
    if (!/^\d+$/.test(raw)) return null;
    return { type: "single_choice", index: Number(raw) };
  }
  if (exercise.kind === "set") {
    return raw.trim() ? { type: "set", value: raw } : null;
  }
  if (exercise.kind === "numeric") {
    return INTEGER.test(raw.trim()) ? { type: "numeric", value: Number(raw.trim()) } : null;
  }
  return null;
}
