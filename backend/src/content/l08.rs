use super::model::*;

const THEORY: &[Block] = &[
    Block::Heading("Endliche Mächtigkeit"),
    Block::Paragraph(r#"Eine Menge, die wir in diesem Kurs antreffen, ist endlich: Man kann ihre verschiedenen Elemente zählen. Die Kardinalität $|A|$ ist diese Anzahl. Es gilt $|\emptyset| = 0$. Wiederholungen in einer Aufzählung erhöhen $|A|$ nicht, denn sie erzeugen keine neuen Elemente. Für endliche $A$ und $B$ gilt $|A \times B| = |A| \cdot |B|$ und $|\mathcal{P}(A)| = 2^{|A|}$, wie in den vorigen Lektionen."#),
    Block::Paragraph(r#"Ist $A \subseteq B$ und sind beide endlich, so gilt $|A| \le |B|$. Insbesondere ist $|A \cap B| \le |A|$ und $|A \cap B| \le |B|$, weil der Schnitt Teilmenge beider Mengen ist. Aus $|A| = |B|$ und $A \subseteq B$ folgt bei endlichen Mengen $A = B$. Bei unendlichen Mengen wäre dieser letzte Schluss falsch; wir bleiben bei endlichen Mengen."#),
    Block::Heading("Inklusion–Exklusion für zwei Mengen"),
    Block::Paragraph(r#"Zählt man $|A| + |B|$, so werden die Elemente des Schnitts zweimal gezählt: einmal in $A$ und einmal in $B$. In der Vereinigung sollen sie nur einmal zählen. Man zieht den Schnitt deshalb einmal ab."#),
    Block::Math(r#"|A \cup B| = |A| + |B| - |A \cap B|."#),
    Block::Paragraph(r#"Sind $A$ und $B$ disjunkt, also $A \cap B = \emptyset$, vereinfacht sich die Formel zu $|A \cup B| = |A| + |B|$. Die Gleichung $|A \cup B| = |A| + |B|$ gilt nicht für alle Paare endlicher Mengen. Sie gilt genau dann, wenn der Schnitt leer ist."#),
];

const EX_A: &[Block] = &[
    Block::Paragraph(r#"Sei $A = \{1,2,3,4\}$ und $B = \{3,4,5\}$. Dann ist $|A| = 4$, $|B| = 3$ und $A \cap B = \{3,4\}$, also $|A \cap B| = 2$. Die Formel liefert $|A \cup B| = 4 + 3 - 2 = 5$. Direktes Aufzählen bestätigt $A \cup B = \{1,2,3,4,5\}$."#),
];

const EX_B: &[Block] = &[
    Block::Paragraph(r#"Sei $A = \{1,2\}$ und $B = \{3,4,5\}$. Der Schnitt ist leer, $|A \cup B| = 2 + 3 - 0 = 5$, und $A \cup B = \{1,2,3,4,5\}$. Hier wäre die Summe der Mächtigkeiten schon das Ergebnis."#),
];

const EX_C: &[Block] = &[
    Block::Paragraph(r#"Sei $|A| = 20$, $|B| = 15$ und $|A \cap B| = 6$. Ohne die Elemente zu kennen, ist $|A \cup B| = 20 + 15 - 6 = 29$. Die Angabe $|A \cap B| = 16$ wäre dagegen unmöglich, weil der Schnitt nicht grösser als $A$ oder $B$ sein kann."#),
];

const SUMMARY: &[Block] = &[
    Block::Bullets(&[
        r#"$|A|$ zählt verschiedene Elemente."#,
        r#"$|A \cup B| = |A| + |B| - |A \cap B|$ für endliche Mengen."#,
        r#"$|A \cap B| \le |A|$ und $\le |B|$. Bei leerem Schnitt entfällt der Abzug."#,
    ]),
];

const EXERCISES: &[Exercise] = &[
    Exercise {
        id: "l08e01",
        lesson_id: "l08",
        difficulty: Difficulty::Einstieg,
        prompt: r#"Es gelte $|A| = 20$, $|B| = 15$ und $|A \cap B| = 6$. Wie gross ist $|A \cup B|$?"#,
        hint: r#"Addiere die beiden Mächtigkeiten und ziehe den doppelt gezählten Schnitt einmal ab."#,
        explanation: r#"$|A \cup B| = 20 + 15 - 6 = 29$. Der Schnitt ist nicht grösser als $A$ oder $B$, die Angaben sind also möglich."#,
        kind: ExerciseKind::Numeric { correct: 29 },
    },
    Exercise {
        id: "l08e02",
        lesson_id: "l08",
        difficulty: Difficulty::Aufbau,
        prompt: r#"Sei $A = \{-2,-1,0,1\}$ und $B = \{0,1,2,3\}$. Wie gross ist $|A \cup B|$?"#,
        hint: r#"Bestimme Schnitt und beide Mächtigkeiten, oder schreibe die Vereinigung hin."#,
        explanation: r#"$|A| = 4$ und $|B| = 4$. Der Schnitt ist $\{0,1\}$ und hat $2$ Elemente. Also ist $|A \cup B| = 4 + 4 - 2 = 6$. Die Vereinigung ist $\{-2,-1,0,1,2,3\}$."#,
        kind: ExerciseKind::Numeric { correct: 6 },
    },
    Exercise {
        id: "l08e03",
        lesson_id: "l08",
        difficulty: Difficulty::Einstieg,
        prompt: r#"Wahr oder falsch? Für alle endlichen Mengen $A$ und $B$ gilt $|A \cup B| = |A| + |B|$."#,
        hint: r#"Was passiert, wenn ein Element in beiden Mengen liegt?"#,
        explanation: r#"Falsch. Für $A = \{1,2\}$ und $B = \{2,3\}$ ist $|A \cup B| = 3$, aber $|A| + |B| = 4$. Die Summe zählt $2$ doppelt. Richtig wäre $|A| + |B| - |A \cap B|$."#,
        kind: ExerciseKind::TrueFalse { correct: false },
    },
    Exercise {
        id: "l08e04",
        lesson_id: "l08",
        difficulty: Difficulty::Aufbau,
        prompt: r#"Wahr oder falsch? Für alle endlichen Mengen $A$ und $B$ gilt $|A \cap B| \le |A|$."#,
        hint: r#"Der Schnitt ist eine Teilmenge von $A$."#,
        explanation: r#"Wahr. Es gilt $A \cap B \subseteq A$. Für endliche Mengen folgt daraus $|A \cap B| \le |A|$. Gleichheit gilt genau dann, wenn $A \subseteq B$."#,
        kind: ExerciseKind::TrueFalse { correct: true },
    },
    Exercise {
        id: "l08e05",
        lesson_id: "l08",
        difficulty: Difficulty::Aufbau,
        prompt: r#"Es gelte $|A| = 8$, $|B| = 5$ und $A \cap B = \emptyset$. Wie gross ist $|A \cup B|$?"#,
        hint: r#"Bei leerem Schnitt wird nichts doppelt gezählt."#,
        explanation: r#"$|A \cap B| = 0$, also $|A \cup B| = 8 + 5 - 0 = 13$."#,
        kind: ExerciseKind::Numeric { correct: 13 },
    },
];

pub static LESSON: Lesson = Lesson {
    id: "l08",
    order: 8,
    title: "Kardinalität und Inklusion–Exklusion",
    description: "Endliche Mächtigkeit und |A ∪ B| = |A| + |B| − |A ∩ B|.",
    difficulty: Difficulty::Aufbau,
    objectives: &[
        r#"Du bestimmst $|A|$ für explizite endliche Mengen und respektierst dabei Duplikate nicht als neue Elemente."#,
        r#"Du wendest die Inklusion–Exklusion für genau zwei Mengen an."#,
        r#"Du erkennst unmögliche Schnittgrössen und den Spezialfall disjunkter Mengen."#,
    ],
    theory: THEORY,
    examples: &[
        Example { title: "Überlappende Mengen", blocks: EX_A },
        Example { title: "Disjunkte Mengen", blocks: EX_B },
        Example { title: "Nur die Anzahlen sind bekannt", blocks: EX_C },
    ],
    misconceptions: &[
        Misconception {
            title: "Die Mächtigkeiten immer addieren",
            text: r#"|A ∪ B| = |A| + |B| gilt nur bei leerem Schnitt. Sonst sind die gemeinsamen Elemente in der Summe doppelt gezählt."#,
        },
        Misconception {
            title: "Einen zu grossen Schnitt akzeptieren",
            text: r#"Wenn |A| = 3 und |B| = 3 angegeben sind, kann |A ∩ B| nicht 4 sein. Der Schnitt ist Teilmenge beider Mengen."#,
        },
    ],
    summary: SUMMARY,
    exercises: EXERCISES,
};
