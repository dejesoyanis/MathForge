"use client";

import { useState } from "react";
import { errorText, submitExercise } from "@/lib/api";
import type { Answer, Exercise, LatestAttempt, SelfAssessment, SubmitResponse } from "@/lib/types";
import { Formula, MathText } from "./MathText";

const INTEGER = /^-?\d+$/;

function fromLatest(attempt: LatestAttempt): SubmitResponse {
  return {
    exercise_id: attempt.exercise_id,
    attempt_id: 0,
    auto_graded: attempt.auto_graded,
    correct: attempt.correct,
    feedback: attempt.feedback,
    normalized: attempt.normalized,
    solution_latex: attempt.solution_latex,
    explanation: attempt.explanation,
    self_assessment: attempt.self_assessment,
    your_answer_display: attempt.answer_display,
  };
}

export function ExerciseCard({
  exercise,
  initial,
  onSubmitted,
}: {
  exercise: Exercise;
  initial?: LatestAttempt;
  onSubmitted: () => void;
}) {
  const [hintOpen, setHintOpen] = useState(false);
  const [answer, setAnswer] = useState<Answer | null>(null);
  const [freeText, setFreeText] = useState(initial?.answer_display ?? "");
  const [result, setResult] = useState<SubmitResponse | null>(initial ? fromLatest(initial) : null);
  const [error, setError] = useState<string | null>(null);
  const [pending, setPending] = useState(false);

  async function send(next: Answer) {
    setPending(true);
    setError(null);
    try {
      const graded = await submitExercise(exercise.id, next);
      setResult(graded);
      onSubmitted();
    } catch (caught) {
      setError(errorText(caught));
    } finally {
      setPending(false);
    }
  }

  async function onSubmit(event: React.FormEvent) {
    event.preventDefault();
    if (exercise.kind === "free_text") {
      await send({ type: "free_text", value: freeText });
      return;
    }
    if (!answer) {
      setError("Bitte zuerst eine Antwort wählen oder eintragen.");
      return;
    }
    if (answer.type === "set" && answer.value.trim() === "") {
      setError("Bitte eine Menge eintragen.");
      return;
    }
    await send(answer);
  }

  async function assess(value: SelfAssessment) {
    await send({ type: "free_text", value: freeText || result?.your_answer_display || "", self_assessment: value });
  }

  return (
    <article className="rounded-lg border border-line bg-card p-4">
      <div className="mb-2 flex flex-wrap items-center gap-2 text-sm text-muted">
        <span>{exercise.difficulty}</span>
        <span aria-hidden="true">·</span>
        <span>{kindLabel(exercise.kind)}</span>
      </div>
      <h3 className="text-lg leading-7">
        <MathText text={exercise.prompt} />
      </h3>
      <form className="mt-4 space-y-3" onSubmit={onSubmit}>
        <AnswerFields exercise={exercise} onChange={setAnswer} freeText={freeText} onFreeText={setFreeText} />
        <div className="flex flex-wrap gap-2">
          <button type="button" className="rounded-md border border-line px-3 py-2 text-sm" onClick={() => setHintOpen((open) => !open)} aria-expanded={hintOpen}>
            {hintOpen ? "Hinweis verbergen" : "Hinweis anzeigen"}
          </button>
          <button type="submit" className="rounded-md bg-accent px-3 py-2 text-sm font-semibold text-white disabled:opacity-60" disabled={pending}>
            {pending ? "Prüfe…" : exercise.kind === "free_text" ? "Mit Musterlösung vergleichen" : "Antwort prüfen"}
          </button>
        </div>
        {hintOpen ? (
          <p className="rounded-md bg-paper px-3 py-2 text-sm">
            <MathText text={exercise.hint} />
          </p>
        ) : null}
        {error ? (
          <p className="text-sm text-bad" role="alert">
            {error}
          </p>
        ) : null}
      </form>
      {result ? <ResultView result={result} onAssess={exercise.kind === "free_text" ? assess : undefined} pending={pending} /> : null}
    </article>
  );
}

function AnswerFields({
  exercise,
  onChange,
  freeText,
  onFreeText,
}: {
  exercise: Exercise;
  onChange: (answer: Answer | null) => void;
  freeText: string;
  onFreeText: (value: string) => void;
}) {
  const [choice, setChoice] = useState<boolean | null>(null);
  const [setText, setSetText] = useState("");

  if (exercise.kind === "true_false") {
    return (
      <div className="flex gap-2" role="group" aria-label="Wahr oder falsch">
        <ChoiceButton
          label="Wahr"
          pressed={choice === true}
          onClick={() => {
            setChoice(true);
            onChange({ type: "true_false", value: true });
          }}
        />
        <ChoiceButton
          label="Falsch"
          pressed={choice === false}
          onClick={() => {
            setChoice(false);
            onChange({ type: "true_false", value: false });
          }}
        />
      </div>
    );
  }
  if (exercise.kind === "single_choice") {
    return (
      <fieldset className="space-y-2">
        <legend className="sr-only">Antwortoptionen</legend>
        {(exercise.options ?? []).map((option, index) => (
          <label key={option} className="flex items-start gap-2">
            <input
              type="radio"
              name={exercise.id}
              className="mt-1"
              onChange={() => onChange({ type: "single_choice", index })}
            />
            <span>
              <MathText text={option} />
            </span>
          </label>
        ))}
      </fieldset>
    );
  }
  if (exercise.kind === "set") {
    return (
      <div>
        <label className="block text-sm" htmlFor={`${exercise.id}-set`}>
          Menge
        </label>
        <div className="mt-1 flex gap-2">
          <input
            id={`${exercise.id}-set`}
            className="w-full rounded-md border border-line bg-white px-3 py-2"
            maxLength={512}
            placeholder="{1,2,3}"
            value={setText}
            onChange={(event) => {
              setSetText(event.target.value);
              onChange({ type: "set", value: event.target.value });
            }}
          />
          <button
            type="button"
            className="rounded-md border border-line px-3"
            onClick={() => {
              setSetText("∅");
              onChange({ type: "set", value: "∅" });
            }}
          >
            ∅
          </button>
        </div>
        <p className="mt-1 text-sm text-muted">Format: ∅, {"{}"} oder {"{"}ganze Zahlen, kommagetrennt{"}"}. Beispiel: {"{-2,0,3}"}.</p>
      </div>
    );
  }
  if (exercise.kind === "numeric") {
    return (
      <div>
        <label className="block text-sm" htmlFor={`${exercise.id}-num`}>
          Zahl
        </label>
        <input
          id={`${exercise.id}-num`}
          className="mt-1 w-40 rounded-md border border-line bg-white px-3 py-2"
          inputMode="numeric"
          maxLength={20}
          onChange={(event) => {
            const raw = event.target.value.trim();
            if (INTEGER.test(raw)) {
              onChange({ type: "numeric", value: Number(raw) });
            } else {
              onChange(null);
            }
          }}
        />
      </div>
    );
  }
  return (
    <div>
      <label className="block text-sm" htmlFor={`${exercise.id}-text`}>
        Begründung
      </label>
      <textarea
        id={`${exercise.id}-text`}
        className="mt-1 min-h-32 w-full rounded-md border border-line bg-white px-3 py-2"
        maxLength={5000}
        value={freeText}
        onChange={(event) => onFreeText(event.target.value)}
      />
      <p className="mt-1 text-sm text-muted">Freitext wird nicht automatisch als Beweis gewertet. Nach dem Vergleich schätzt du dich selbst ein.</p>
    </div>
  );
}

function ChoiceButton({ label, onClick, pressed = false }: { label: string; onClick: () => void; pressed?: boolean }) {
  return (
    <button
      type="button"
      className={`rounded-md border px-4 py-2 ${pressed ? "border-accent bg-accent text-white" : "border-line bg-white"}`}
      aria-pressed={pressed}
      onClick={onClick}
    >
      {label}
    </button>
  );
}

function ResultView({
  result,
  onAssess,
  pending,
}: {
  result: SubmitResponse;
  onAssess?: (value: SelfAssessment) => void;
  pending: boolean;
}) {
  const tone = !result.auto_graded ? "text-ink" : result.correct ? "text-good" : "text-bad";
  const title = !result.auto_graded ? "Nicht automatisch bewertet" : result.correct ? "Richtig" : "Nicht korrekt";
  return (
    <div className="mt-4 space-y-3 border-t border-line pt-4" role="status">
      <p className={`font-semibold ${tone}`}>{title}</p>
      <p>{result.feedback}</p>
      {result.normalized ? (
        <p>
          Normalisierte Antwort: <span className="font-mono">{result.normalized}</span>
        </p>
      ) : null}
      <div>
        <h4 className="text-base">Musterlösung</h4>
        {result.solution_latex ? (
          <p className="mt-1">
            <Formula latex={result.solution_latex} />
          </p>
        ) : null}
        <p className="mt-1">
          <MathText text={result.explanation} />
        </p>
      </div>
      {onAssess ? (
        <div>
          <p className="text-sm">Selbsteinschätzung, nachdem du die Musterlösung gelesen hast:</p>
          <div className="mt-2 flex flex-wrap gap-2">
            <button type="button" className="rounded-md border border-line px-3 py-2 text-sm" disabled={pending} onClick={() => onAssess("correct")}>
              Korrekt
            </button>
            <button type="button" className="rounded-md border border-line px-3 py-2 text-sm" disabled={pending} onClick={() => onAssess("partial")}>
              Teilweise
            </button>
            <button type="button" className="rounded-md border border-line px-3 py-2 text-sm" disabled={pending} onClick={() => onAssess("incorrect")}>
              Nicht korrekt
            </button>
          </div>
          {result.self_assessment ? (
            <p className="mt-2 text-sm">Gespeicherte Selbsteinschätzung: {assessmentLabel(result.self_assessment)}. Das ersetzt keine automatische Bewertung.</p>
          ) : null}
        </div>
      ) : null}
    </div>
  );
}

function kindLabel(kind: Exercise["kind"]): string {
  switch (kind) {
    case "true_false":
      return "Wahr/Falsch";
    case "single_choice":
      return "Einzelauswahl";
    case "set":
      return "Menge";
    case "numeric":
      return "Zahl";
    case "free_text":
      return "Begründung";
  }
}

function assessmentLabel(value: SelfAssessment): string {
  if (value === "correct") return "korrekt";
  if (value === "partial") return "teilweise";
  return "nicht korrekt";
}

export function answerFromDraft(exercise: Exercise, draft: Answer | null, freeText: string): Answer | null {
  if (exercise.kind === "free_text") {
    return freeText.trim() ? { type: "free_text", value: freeText } : null;
  }
  if (!draft) return null;
  if (draft.type === "set" && draft.value.trim() === "") return null;
  return draft;
}
