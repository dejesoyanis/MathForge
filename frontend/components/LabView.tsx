"use client";

import { useState } from "react";
import { errorText, evaluateLab } from "@/lib/api";
import type { LabResponse } from "@/lib/types";
import { Formula } from "./MathText";

const DEFAULT_A = "{-1,0,2}";
const DEFAULT_B = "{0,3}";

export function LabView() {
  const [a, setA] = useState(DEFAULT_A);
  const [b, setB] = useState(DEFAULT_B);
  const [result, setResult] = useState<LabResponse | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [pending, setPending] = useState(false);

  async function onSubmit(event: React.FormEvent) {
    event.preventDefault();
    setPending(true);
    setError(null);
    try {
      setResult(await evaluateLab(a, b));
    } catch (caught) {
      setResult(null);
      setError(errorText(caught));
    } finally {
      setPending(false);
    }
  }

  return (
    <div className="max-w-3xl">
      <h1 className="text-4xl">Mengenlabor</h1>
      <p className="mt-3 leading-7 text-muted">
        Zwei endliche Mengen ganzer Zahlen. Vereinigung, Schnitt, Differenzen, symmetrische Differenz, Teilmengen und Mächtigkeiten berechnet das Backend.
      </p>
      <form className="mt-6 space-y-4" onSubmit={onSubmit}>
        <SetField id="set-a" label="Menge A" value={a} onChange={setA} />
        <SetField id="set-b" label="Menge B" value={b} onChange={setB} />
        <p className="text-sm text-muted">Erlaubt sind ∅, {"{}"} und Mengen wie {"{-2,0,3}"}. Höchstens 512 Zeichen und 64 Einträge. Verschachtelte Mengen sind hier nicht lesbar.</p>
        <div className="flex flex-wrap gap-2">
          <button type="submit" className="rounded-md bg-accent px-4 py-2 font-semibold text-white disabled:opacity-60" disabled={pending}>
            {pending ? "Berechne…" : "Berechnen"}
          </button>
          <button
            type="button"
            className="rounded-md border border-line px-4 py-2"
            onClick={() => {
              setA(DEFAULT_A);
              setB(DEFAULT_B);
            }}
          >
            Beispiel laden
          </button>
        </div>
        {error ? (
          <p className="text-bad" role="alert">
            {error}
          </p>
        ) : null}
      </form>
      {result ? (
        <dl className="mt-8 space-y-3">
          <Row term="A" latex={result.a_latex} extra={`Mächtigkeit ${result.cardinality_a}`} />
          <Row term="B" latex={result.b_latex} extra={`Mächtigkeit ${result.cardinality_b}`} />
          <Row term="A ∪ B" latex={result.union_latex} extra={`Mächtigkeit ${result.cardinality_union}`} />
          <Row term="A ∩ B" latex={result.intersection_latex} extra={`Mächtigkeit ${result.cardinality_intersection}`} />
          <Row term="A \\ B" latex={result.difference_a_b_latex} extra={`Mächtigkeit ${result.cardinality_difference_a_b}`} />
          <Row term="B \\ A" latex={result.difference_b_a_latex} extra={`Mächtigkeit ${result.cardinality_difference_b_a}`} />
          <Row term="A △ B" latex={result.symmetric_difference_latex} extra={`Mächtigkeit ${result.cardinality_symmetric_difference}`} />
          <div className="rounded-lg border border-line bg-card p-4 text-sm leading-6">
            <p>{result.equal ? "A = B" : "A ≠ B"}</p>
            <p>{result.a_subseteq_b ? "A ⊆ B" : "A ist keine Teilmenge von B"}</p>
            <p>{result.b_subseteq_a ? "B ⊆ A" : "B ist keine Teilmenge von A"}</p>
            <p>{result.a_subsetneq_b ? "A ⊊ B" : "A ist keine echte Teilmenge von B"}</p>
            <p>{result.b_subsetneq_a ? "B ⊊ A" : "B ist keine echte Teilmenge von A"}</p>
          </div>
        </dl>
      ) : null}
    </div>
  );
}

function SetField({ id, label, value, onChange }: { id: string; label: string; value: string; onChange: (value: string) => void }) {
  return (
    <div>
      <label className="block text-sm font-semibold" htmlFor={id}>
        {label}
      </label>
      <input id={id} className="mt-1 w-full rounded-md border border-line bg-white px-3 py-2" maxLength={512} value={value} onChange={(event) => onChange(event.target.value)} />
    </div>
  );
}

function Row({ term, latex, extra }: { term: string; latex: string; extra: string }) {
  return (
    <div className="rounded-lg border border-line bg-card px-4 py-3">
      <dt className="text-sm text-muted">{term}</dt>
      <dd className="text-lg">
        <Formula latex={latex} />
      </dd>
      <p className="text-sm text-muted">{extra}</p>
    </div>
  );
}
