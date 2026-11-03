import { render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import { examScoreText, lessonWorkText } from "@/lib/score";

function StatsPreview() {
  return (
    <section aria-label="Lernstand">
      <p>4 bearbeitete Übungen</p>
      <p>2 richtige Antworten</p>
      <p>1 von 10 abgeschlossene Lektionen</p>
      <p>{lessonWorkText(2, 5)}</p>
      <p>{examScoreText(8, 10)}</p>
    </section>
  );
}

describe("Lernstand ohne Prozent", () => {
  it("formuliert Zähler und Nenner in Worten", () => {
    expect(examScoreText(8, 10)).toBe("8 von 10 automatisch bewerteten Aufgaben richtig");
    expect(lessonWorkText(2, 5)).toBe("2 von 5 Übungen bearbeitet");
    expect(examScoreText(8, 10).includes("%")).toBe(false);
  });

  it("zeigt den Lernstand ohne Prozentzeichen", () => {
    render(<StatsPreview />);
    const region = screen.getByRole("region", { name: "Lernstand" });
    expect(region.textContent).toContain("2 richtige Antworten");
    expect(region.textContent).not.toContain("%");
  });
});
