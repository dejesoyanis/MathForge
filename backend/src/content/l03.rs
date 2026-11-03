use super::model::*;

const THEORY: &[Block] = &[
    Block::Heading("Vier Verknüpfungen"),
    Block::Paragraph(r#"Aus zwei Mengen $A$ und $B$ bilden wir neue Mengen, indem wir festlegen, welche Elemente aufgenommen werden. Alle folgenden Definitionen bleiben innerhalb der Elemente von $A$ und $B$. Es wird keine unbeschränkte Komprehension benötigt."#),
    Block::Math(r#"A \cup B = \{ x \mid x \in A \text{ oder } x \in B \}"#),
    Block::Math(r#"A \cap B = \{ x \mid x \in A \text{ und } x \in B \}"#),
    Block::Math(r#"A \setminus B = \{ x \in A \mid x \notin B \}"#),
    Block::Math(r#"A \triangle B = (A \setminus B) \cup (B \setminus A)"#),
    Block::Paragraph(r#"$A \cup B$ ist die Vereinigung, $A \cap B$ der Schnitt, $A \setminus B$ die Differenz «$A$ ohne $B$». Die symmetrische Differenz $A \triangle B$ enthält, was in genau einer der beiden Mengen liegt. Gleichwertig ist $A \triangle B = (A \cup B) \setminus (A \cap B)$."#),
    Block::Heading("Rechnen mit der leeren Menge"),
    Block::Paragraph(r#"Für jede Menge $A$ gilt $A \cup \emptyset = A$, $A \cap \emptyset = \emptyset$, $A \setminus \emptyset = A$ und $A \setminus A = \emptyset$. Ausserdem ist $A \triangle A = \emptyset$ und $A \triangle \emptyset = A$. Vereinigung und Schnitt sind kommutativ: $A \cup B = B \cup A$ und $A \cap B = B \cap A$. Die Differenz ist es nicht."#),
];

const EX_A: &[Block] = &[
    Block::Paragraph(r#"Sei $A = \{1,2,3\}$ und $B = \{3,4,5\}$. Dann ist $A \cup B = \{1,2,3,4,5\}$ und $A \cap B = \{3\}$. Die Differenz $A \setminus B = \{1,2\}$ behält nur, was in $A$, aber nicht in $B$ liegt. $B \setminus A = \{4,5\}$. Die symmetrische Differenz ist $A \triangle B = \{1,2,4,5\}$."#),
];

const EX_B: &[Block] = &[
    Block::Paragraph(r#"Sei $A = \{1,2\}$ und $B = \{1,2,3\}$. Dann ist $A \cap B = A$, $A \cup B = B$, $A \setminus B = \emptyset$ und $B \setminus A = \{3\}$. Das passt dazu, dass $A \subseteq B$ genau dann gilt, wenn $A \cap B = A$, und auch genau dann, wenn $A \setminus B = \emptyset$."#),
];

const EX_C: &[Block] = &[
    Block::Paragraph(r#"Sei $A = \{0,1\}$ und $B = \emptyset$. Dann ist $A \cup B = \{0,1\}$, $A \cap B = \emptyset$, $A \setminus B = \{0,1\}$ und $A \triangle B = \{0,1\}$. Die leere Menge ändert bei der Vereinigung nichts und löscht beim Schnitt alles."#),
];

const SUMMARY: &[Block] = &[
    Block::Bullets(&[
        r#"Vereinigung: in $A$ oder in $B$. Schnitt: in $A$ und in $B$."#,
        r#"$A \setminus B$ entfernt aus $A$ die Elemente, die auch in $B$ liegen."#,
        r#"$A \triangle B$ ist der Rest der Vereinigung nach Entfernen des Schnitts."#,
        r#"$A \setminus B$ und $B \setminus A$ sind im Allgemeinen verschieden."#,
    ]),
];

const EXERCISES: &[Exercise] = &[
    Exercise {
        id: "l03e01",
        lesson_id: "l03",
        difficulty: Difficulty::Einstieg,
        prompt: r#"Bestimme $\{1,2,4\} \cup \{2,3\}$."#,
        hint: r#"Nimm jedes Element, das in mindestens einer der beiden Mengen liegt."#,
        explanation: r#"Die Elemente $1$, $2$ und $4$ kommen aus der ersten Menge, $3$ aus der zweiten. $2$ liegt in beiden und erscheint nur einmal. Das Ergebnis ist $\{1,2,3,4\}$."#,
        kind: ExerciseKind::Set { correct: &[1, 2, 3, 4] },
    },
    Exercise {
        id: "l03e02",
        lesson_id: "l03",
        difficulty: Difficulty::Einstieg,
        prompt: r#"Bestimme $\{1,2,4\} \cap \{2,3,4\}$."#,
        hint: r#"Behalte nur Zahlen, die links und rechts vorkommen."#,
        explanation: r#"Gemeinsam sind $2$ und $4$. $1$ fehlt rechts, $3$ fehlt links. Der Schnitt ist $\{2,4\}$."#,
        kind: ExerciseKind::Set { correct: &[2, 4] },
    },
    Exercise {
        id: "l03e03",
        lesson_id: "l03",
        difficulty: Difficulty::Aufbau,
        prompt: r#"Bestimme $\{1,2,4\} \setminus \{2,3\}$."#,
        hint: r#"Streiche aus der linken Menge, was auch rechts steht."#,
        explanation: r#"Von $\{1,2,4\}$ wird $2$ entfernt, weil $2 \in \{2,3\}$. $3$ liegt nicht links und spielt für diese Differenz keine Rolle. Übrig bleibt $\{1,4\}$."#,
        kind: ExerciseKind::Set { correct: &[1, 4] },
    },
    Exercise {
        id: "l03e04",
        lesson_id: "l03",
        difficulty: Difficulty::Aufbau,
        prompt: r#"Bestimme die symmetrische Differenz $\{1,2,3\} \triangle \{2,3,5\}$."#,
        hint: r#"Entferne den Schnitt aus der Vereinigung, oder bilde beide Differenzen und vereinige sie."#,
        explanation: r#"Der Schnitt ist $\{2,3\}$, die Vereinigung ist $\{1,2,3,5\}$. Entfernt man den Schnitt, bleibt $\{1,5\}$. Dieselben Elemente liefern $(A \setminus B) \cup (B \setminus A) = \{1\} \cup \{5\}$."#,
        kind: ExerciseKind::Set { correct: &[1, 5] },
    },
    Exercise {
        id: "l03e05",
        lesson_id: "l03",
        difficulty: Difficulty::Vertiefung,
        prompt: r#"Wahr oder falsch? Für alle Mengen $A$ und $B$ gilt $A \setminus B = B \setminus A$."#,
        hint: r#"Probiere zwei kleine Mengen, bei denen ein Element nur links vorkommt."#,
        explanation: r#"Falsch. Für $A = \{1\}$ und $B = \{2\}$ ist $A \setminus B = \{1\}$ und $B \setminus A = \{2\}$. Die Differenz ist nicht kommutativ. Vereinigung und Schnitt sind es."#,
        kind: ExerciseKind::TrueFalse { correct: false },
    },
];

pub static LESSON: Lesson = Lesson {
    id: "l03",
    order: 3,
    title: "Vereinigung, Schnitt und Differenz",
    description: "∪, ∩, Differenz und symmetrische Differenz endlicher Mengen.",
    difficulty: Difficulty::Einstieg,
    objectives: &[
        r#"Du berechnest Vereinigung, Schnitt, Differenz und symmetrische Differenz endlicher Mengen."#,
        r#"Du übersetzt die vier Operationen in Bedingungen an ein einzelnes Element."#,
        r#"Du erkennst, welche dieser Operationen kommutativ sind."#,
    ],
    theory: THEORY,
    examples: &[
        Example { title: "Alle vier Operationen an einem Paar", blocks: EX_A },
        Example { title: "Wenn eine Menge Teilmenge ist", blocks: EX_B },
        Example { title: "Die leere Menge als Partner", blocks: EX_C },
    ],
    misconceptions: &[
        Misconception {
            title: "Differenz wie Subtraktion von Anzahlen behandeln",
            text: r#"{1,2,3} \ {3,4} ist nicht «3 minus 2». Es ist die Menge {1,2}. Die Zahl 4 kommt in der Differenz nicht vor, weil sie gar nicht in der linken Menge lag."#,
        },
        Misconception {
            title: "Symmetrische Differenz mit dem Schnitt verwechseln",
            text: r#"Im Schnitt liegt, was beide Mengen teilen. In der symmetrischen Differenz liegt genau das Gegenteil davon innerhalb der Vereinigung: was nicht geteilt wird."#,
        },
    ],
    summary: SUMMARY,
    exercises: EXERCISES,
};
