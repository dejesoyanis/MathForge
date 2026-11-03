use super::model::*;

const THEORY: &[Block] = &[
    Block::Heading("Geordnete Paare"),
    Block::Paragraph(r#"Ein geordnetes Paar $(a,b)$ merkt sich, welches Objekt zuerst und welches danach steht. Zwei Paare sind genau dann gleich, wenn die Komponenten der Reihe nach übereinstimmen."#),
    Block::Math(r#"(a,b) = (c,d) \iff a = c \text{ und } b = d."#),
    Block::Paragraph(r#"Deshalb ist $(1,2) \neq (2,1)$. Das unterscheidet Paare von Mengen: $\{1,2\} = \{2,1\}$, aber die Paare sind verschieden. Ein Paar ist auch keine zweielementige Menge. $(1,1)$ ist ein sinnvolles Paar, während $\{1,1\} = \{1\}$ nur ein Element hat. Die mengentheoretische Kodierung von Paaren nach Kuratowski, $(a,b) = \{\{a\},\{a,b\}\}$, erwähnen wir nur als Ausblick. In den Aufgaben rechnen wir mit Paaren direkt und nicht über diese Kodierung."#),
    Block::Heading("Das kartesische Produkt"),
    Block::Paragraph(r#"Das kartesische Produkt $A \times B$ besteht aus allen Paaren, deren erste Komponente aus $A$ und deren zweite aus $B$ stammt."#),
    Block::Math(r#"A \times B = \{ (a,b) \mid a \in A \text{ und } b \in B \}."#),
    Block::Paragraph(r#"Sind $A$ und $B$ endlich, so gilt $|A \times B| = |A| \cdot |B|$. Jedes der $|A|$ Elemente kann mit jedem der $|B|$ Elemente kombiniert werden. Ist eine Seite leer, gibt es keine Paare: $A \times \emptyset = \emptyset = \emptyset \times A$. Im Allgemeinen ist $A \times B \neq B \times A$, weil $(a,b)$ und $(b,a)$ verschiedene Paare sind. Die Elemente von $A \times B$ sind Paare, nicht ganze Zahlen. Antworten dazu werden als Anzahl oder als Auswahl gestellt, nicht als Integer-Menge."#),
];

const EX_A: &[Block] = &[
    Block::Paragraph(r#"Sei $A = \{1,2\}$ und $B = \{3\}$. Dann ist $A \times B = \{(1,3),(2,3)\}$. Die Mächtigkeit ist $2 \cdot 1 = 2$. Das Produkt $B \times A = \{(3,1),(3,2)\}$ ist eine andere Menge von Paaren."#),
];

const EX_B: &[Block] = &[
    Block::Paragraph(r#"Sei $A = \{1\}$ und $B = \{1,2\}$. Dann ist $A \times B = \{(1,1),(1,2)\}$ und $|A \times B| = 2$. Das Paar $(2,1)$ liegt nicht darin, weil $2 \notin A$. Das Paar $(1,1)$ ist erlaubt, obwohl beide Komponenten gleich sind."#),
];

const EX_C: &[Block] = &[
    Block::Paragraph(r#"Sei $A = \{1,2\}$ und $B = \emptyset$. Es gibt kein $b \in B$, also kein Paar $(a,b)$. Damit ist $A \times B = \emptyset$ und $|A \times B| = 0 = 2 \cdot 0$."#),
];

const SUMMARY: &[Block] = &[
    Block::Bullets(&[
        r#"$(a,b) = (c,d)$ genau dann, wenn $a = c$ und $b = d$."#,
        r#"$A \times B$ enthält genau die Paare mit erster Komponente aus $A$ und zweiter aus $B$."#,
        r#"Für endliche Mengen gilt $|A \times B| = |A| \cdot |B|$, und ein leerer Faktor ergibt $\emptyset$."#,
    ]),
];

const EXERCISES: &[Exercise] = &[
    Exercise {
        id: "l06e01",
        lesson_id: "l06",
        difficulty: Difficulty::Einstieg,
        prompt: r#"Wie viele Elemente hat $\{1,2,3\} \times \{4,5\}$?"#,
        hint: r#"Multipliziere die beiden Mächtigkeiten."#,
        explanation: r#"Die linke Menge hat $3$ Elemente, die rechte $2$. Jede Kombination kommt genau einmal vor. $|A \times B| = 3 \cdot 2 = 6$. Die sechs Paare sind $(1,4)$, $(1,5)$, $(2,4)$, $(2,5)$, $(3,4)$ und $(3,5)$."#,
        kind: ExerciseKind::Numeric { correct: 6 },
    },
    Exercise {
        id: "l06e02",
        lesson_id: "l06",
        difficulty: Difficulty::Einstieg,
        prompt: r#"Wie viele Elemente hat $\{1,2\} \times \emptyset$?"#,
        hint: r#"Ein Paar braucht eine zweite Komponente aus der rechten Menge."#,
        explanation: r#"Die rechte Menge hat kein Element. Es lässt sich kein Paar bilden. Das Produkt ist $\emptyset$ und hat $0$ Elemente. Dasselbe liefert $2 \cdot 0 = 0$."#,
        kind: ExerciseKind::Numeric { correct: 0 },
    },
    Exercise {
        id: "l06e03",
        lesson_id: "l06",
        difficulty: Difficulty::Aufbau,
        prompt: r#"Wahr oder falsch? Es gilt $(1,2) = (2,1)$."#,
        hint: r#"Vergleiche die erste Komponente mit der ersten und die zweite mit der zweiten."#,
        explanation: r#"Falsch. Die ersten Komponenten sind $1$ und $2$, also verschieden. Bei Mengen wäre $\{1,2\} = \{2,1\}$, bei Paaren zählt die Position."#,
        kind: ExerciseKind::TrueFalse { correct: false },
    },
    Exercise {
        id: "l06e04",
        lesson_id: "l06",
        difficulty: Difficulty::Aufbau,
        prompt: r#"Wahr oder falsch? Für alle Mengen $A$ und $B$ gilt $A \times B = B \times A$."#,
        hint: r#"Vergleiche $\{1\} \times \{2\}$ mit $\{2\} \times \{1\}$."#,
        explanation: r#"Falsch. $\{1\} \times \{2\} = \{(1,2)\}$ und $\{2\} \times \{1\} = \{(2,1)\}$. Diese Paarmengen sind verschieden. Es gibt Sonderfälle mit Gleichheit, etwa wenn $A = B$ oder eine Seite leer ist. Die allgemeine Aussage ist trotzdem falsch."#,
        kind: ExerciseKind::TrueFalse { correct: false },
    },
    Exercise {
        id: "l06e05",
        lesson_id: "l06",
        difficulty: Difficulty::Vertiefung,
        prompt: r#"Welche Paare liegen in $\{1,2\} \times \{2\}$?"#,
        hint: r#"Die zweite Komponente muss $2$ sein. Die erste darf $1$ oder $2$ sein."#,
        explanation: r#"Die erste Komponente durchläuft $\{1,2\}$, die zweite ist fest $2$. Also liegen genau $(1,2)$ und $(2,2)$ im Produkt. $(2,1)$ hätte die Komponenten vertauscht und läge in $\{2\} \times \{1,2\}$, nicht hier."#,
        kind: ExerciseKind::SingleChoice {
            options: &[
                r#"nur $(2,1)$"#,
                r#"$(1,2)$ und $(2,2)$"#,
                r#"$(2,1)$ und $(2,2)$"#,
                r#"$(1,1)$ und $(2,2)$"#,
            ],
            correct_index: 1,
        },
    },
];

pub static LESSON: Lesson = Lesson {
    id: "l06",
    order: 6,
    title: "Kartesische Produkte",
    description: "Geordnete Paare, A × B und die Produktregel für endliche Mächtigkeiten.",
    difficulty: Difficulty::Aufbau,
    objectives: &[
        r#"Du unterscheidest $(a,b)$ von $\{a,b\}$ und prüfst Gleichheit von Paaren komponentenweise."#,
        r#"Du zählst und beschreibst die Elemente von $A \times B$ für kleine endliche Mengen."#,
        r#"Du verwendest $|A \times B| = |A| \cdot |B|$ und den Sonderfall eines leeren Faktors."#,
    ],
    theory: THEORY,
    examples: &[
        Example { title: "Ein Produkt mit zwei Paaren", blocks: EX_A },
        Example { title: "Gleiche Komponenten sind erlaubt", blocks: EX_B },
        Example { title: "Leerer Faktor", blocks: EX_C },
    ],
    misconceptions: &[
        Misconception {
            title: "Paar und Menge gleichsetzen",
            text: r#"(1,2) ist nicht {1,2}. Im Paar ist die Reihenfolge Teil der Information, und (1,1) hat zwei Komponenten, obwohl eine Menge dieselben Einträge zusammenfallen lassen würde."#,
        },
        Misconception {
            title: "Das Produkt für kommutativ halten",
            text: r#"A × B und B × A haben bei endlichen, nicht leeren Mengen dieselbe Mächtigkeit, aber nicht dieselben Paare. Gleiche Anzahl ist schwächer als Gleichheit."#,
        },
    ],
    summary: SUMMARY,
    exercises: EXERCISES,
};
