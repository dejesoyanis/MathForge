use super::model::*;

const EXAM: &[Exercise] = &[
    Exercise {
        id: "x01",
        lesson_id: "l01",
        difficulty: Difficulty::Einstieg,
        prompt: r#"Wahr oder falsch? Es gilt $\{1,2,2,3\} = \{3,2,1\}$."#,
        hint: r#"Vergleiche die verschiedenen Elemente."#,
        explanation: r#"Wahr. Beide Seiten enthalten genau $1$, $2$ und $3$. Die doppelte $2$ auf der linken Seite erzeugt kein weiteres Element."#,
        kind: ExerciseKind::TrueFalse { correct: true },
    },
    Exercise {
        id: "x02",
        lesson_id: "l02",
        difficulty: Difficulty::Einstieg,
        prompt: r#"Wahr oder falsch? Es gilt $\{1\} \subsetneq \{1,2,3\}$."#,
        hint: r#"Prüfe Teilmenge und Ungleichheit getrennt."#,
        explanation: r#"Wahr. $1$ liegt in $\{1,2,3\}$, also gilt die Inklusion. Wegen $2 \notin \{1\}$ sind die Mengen ungleich, die Teilmenge ist echt."#,
        kind: ExerciseKind::TrueFalse { correct: true },
    },
    Exercise {
        id: "x03",
        lesson_id: "l03",
        difficulty: Difficulty::Einstieg,
        prompt: r#"Bestimme $\{1,3,5\} \cap \{2,3,4\}$."#,
        hint: r#"Nur die gemeinsame Zahl bleibt übrig."#,
        explanation: r#"Die einzige Zahl in beiden Mengen ist $3$. Der Schnitt ist $\{3\}$."#,
        kind: ExerciseKind::Set { correct: &[3] },
    },
    Exercise {
        id: "x04",
        lesson_id: "l04",
        difficulty: Difficulty::Aufbau,
        prompt: r#"Sei $U = \{1,2,3,4,5,6\}$ und $A = \{2,4,6\}$. Bestimme $\complement_U(A)$."#,
        hint: r#"Streiche die geraden Elemente aus $U$."#,
        explanation: r#"Von $1$ bis $6$ liegen $2$, $4$ und $6$ in $A$. Das Komplement ist $\{1,3,5\}$."#,
        kind: ExerciseKind::Set { correct: &[1, 3, 5] },
    },
    Exercise {
        id: "x05",
        lesson_id: "l05",
        difficulty: Difficulty::Aufbau,
        prompt: r#"Wie viele Elemente hat $\mathcal{P}(\{1,2\})$?"#,
        hint: r#"Eine zweielementige Menge hat $2^{2}$ Teilmengen."#,
        explanation: r#"Die Teilmengen sind $\emptyset$, $\{1\}$, $\{2\}$ und $\{1,2\}$. Das sind $2^{2} = 4$ Elemente."#,
        kind: ExerciseKind::Numeric { correct: 4 },
    },
    Exercise {
        id: "x06",
        lesson_id: "l06",
        difficulty: Difficulty::Aufbau,
        prompt: r#"Wie viele Elemente hat $\{1\} \times \{1,2,3\}$?"#,
        hint: r#"Es gibt nur eine Wahl für die erste Komponente."#,
        explanation: r#"$|\{1\}| \cdot |\{1,2,3\}| = 1 \cdot 3 = 3$. Die Paare sind $(1,1)$, $(1,2)$ und $(1,3)$."#,
        kind: ExerciseKind::Numeric { correct: 3 },
    },
    Exercise {
        id: "x07",
        lesson_id: "l07",
        difficulty: Difficulty::Aufbau,
        prompt: r#"Wahr oder falsch? Für alle Mengen $A$ und $B$ gilt $A \cap (A \cup B) = A$."#,
        hint: r#"Das ist eines der Absorptionsgesetze."#,
        explanation: r#"Wahr. $A \cap (A \cup B) = A$. Jedes Element von $A$ liegt in $A$ und in der Vereinigung. Jedes Element des Schnitts liegt insbesondere in $A$."#,
        kind: ExerciseKind::TrueFalse { correct: true },
    },
    Exercise {
        id: "x08",
        lesson_id: "l08",
        difficulty: Difficulty::Aufbau,
        prompt: r#"Es gelte $|A| = 10$, $|B| = 7$ und $|A \cap B| = 4$. Wie gross ist $|A \cup B|$?"#,
        hint: r#"Ziehe den Schnitt einmal von der Summe ab."#,
        explanation: r#"$|A \cup B| = 10 + 7 - 4 = 13$."#,
        kind: ExerciseKind::Numeric { correct: 13 },
    },
    Exercise {
        id: "x09",
        lesson_id: "l09",
        difficulty: Difficulty::Vertiefung,
        prompt: r#"Wahr oder falsch? Die Relation $R = \{(1,1),(1,2),(2,2)\}$ auf $A = \{1,2\}$ ist symmetrisch."#,
        hint: r#"Suche ein Paar, dessen Spiegelbild fehlt."#,
        explanation: r#"Falsch. $(1,2) \in R$, aber $(2,1) \notin R$. Die Relation ist reflexiv, das ist für die Symmetrie aber nicht ausreichend."#,
        kind: ExerciseKind::TrueFalse { correct: false },
    },
    Exercise {
        id: "x10",
        lesson_id: "l10",
        difficulty: Difficulty::Vertiefung,
        prompt: r#"Wahr oder falsch? $\{(1,1),(2,1),(3,1)\}$ ist eine Funktion von $\{1,2,3\}$ nach $\{1\}$."#,
        hint: r#"Mehrere Argumente dürfen denselben Wert haben, wenn jedes Argument genau einen Wert hat."#,
        explanation: r#"Wahr. Jedes der Argumente $1$, $2$ und $3$ hat genau den Wert $1$, und $1$ liegt in der Zielmenge. Das Bild ist $\{1\}$."#,
        kind: ExerciseKind::TrueFalse { correct: true },
    },
];

pub fn exam_exercises() -> &'static [Exercise] {
    EXAM
}

pub const SCORE_NOTE: &str = "Der Prüfungsscore zählt ausschliesslich automatisch bewertete Aufgaben. Eine Freitext- oder Beweisaufgabe wird nie automatisch als korrekt markiert. Diese Prüfung enthält nur automatisch bewertete Aufgaben, eine aus jeder Lektion. Der Nenner ist deshalb die Anzahl der Prüfungsaufgaben. Selbsteinschätzungen kommen in diesem Score nicht vor.";
