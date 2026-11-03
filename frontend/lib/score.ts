export function examScoreText(correct: number, total: number): string {
  return `${correct} von ${total} automatisch bewerteten Aufgaben richtig`;
}

export function lessonWorkText(attempted: number, total: number): string {
  return `${attempted} von ${total} Übungen bearbeitet`;
}
