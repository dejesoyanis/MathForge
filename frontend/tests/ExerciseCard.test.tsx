import { fireEvent, render, screen } from "@testing-library/react";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { ExerciseCard } from "@/components/ExerciseCard";
import type { Exercise } from "@/lib/types";

const exercise: Exercise = {
  id: "l01e01",
  lesson_id: "l01",
  difficulty: "Einstieg",
  kind: "true_false",
  prompt: "Wahr oder falsch? Die Mengen sind gleich.",
  hint: "Vergleiche die Elemente.",
  options: undefined,
};

const graded = {
  exercise_id: "l01e01",
  attempt_id: 1,
  auto_graded: true,
  correct: true,
  feedback: "Richtig.",
  normalized: null,
  solution_latex: "\\text{wahr}",
  explanation: "Beide Mengen enthalten dieselben Elemente.",
  self_assessment: null,
  your_answer_display: "wahr",
};

describe("ExerciseCard", () => {
  beforeEach(() => {
    vi.stubGlobal(
      "fetch",
      vi.fn().mockResolvedValue({
        ok: true,
        status: 200,
        text: async () => JSON.stringify(graded),
      }),
    );
  });

  it("zeigt die Musterlösung erst nach dem Abschicken", async () => {
    render(<ExerciseCard exercise={exercise} onSubmitted={() => undefined} />);
    expect(screen.queryByRole("heading", { name: "Musterlösung" })).toBeNull();
    expect(screen.queryByText("Beide Mengen enthalten dieselben Elemente.")).toBeNull();

    fireEvent.click(screen.getByRole("button", { name: "Hinweis anzeigen" }));
    expect(screen.getByText("Vergleiche die Elemente.")).toBeTruthy();

    fireEvent.click(screen.getByRole("button", { name: "Wahr" }));
    fireEvent.click(screen.getByRole("button", { name: "Antwort prüfen" }));

    expect(await screen.findByRole("heading", { name: "Musterlösung" })).toBeTruthy();
    expect(screen.getByText("Beide Mengen enthalten dieselben Elemente.")).toBeTruthy();
    expect(fetch).toHaveBeenCalled();
    const call = vi.mocked(fetch).mock.calls[0];
    expect(String(call?.[0])).toContain("/api/v1/exercises/l01e01/submit");
  });
});
