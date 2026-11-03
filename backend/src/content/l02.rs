use super::model::*;

const THEORY: &[Block] = &[
    Block::Heading("Teilmenge und echte Teilmenge"),
    Block::Paragraph(r#"$A$ ist eine Teilmenge von $B$, geschrieben $A \subseteq B$, wenn jedes Element von $A$ auch Element von $B$ ist. Formal: Für jedes $x$ gilt $x \in A \rightarrow x \in B$. Gleichheit ist der Spezialfall, in dem auch die umgekehrte Richtung gilt. Aus $A \subseteq B$ und $B \subseteq A$ folgt $A = B$."#),
    Block::Paragraph(r#"$A$ ist eine echte Teilmenge von $B$, geschrieben $A \subsetneq B$, wenn $A \subseteq B$ und zugleich $A \neq B$ gilt. Dann liegt mindestens ein Element von $B$ nicht in $A$. In diesem Kurs verwenden wir das Zeichen $\subset$ nicht. In manchen Texten bedeutet $\subset$ «Teilmenge», in anderen «echte Teilmenge». Die Zeichen $\subseteq$ und $\subsetneq$ machen die Bedeutung eindeutig."#),
    Block::Heading("Die leere Menge"),
    Block::Paragraph(r#"Die leere Menge $\emptyset$ ist die Menge ohne Elemente. Sie ist Teilmenge jeder Menge: Die Bedingung «jedes Element von $\emptyset$ liegt in $A$» ist erfüllt, weil es kein Gegenbeispiel geben kann. Insbesondere gilt $\emptyset \subseteq \emptyset$, aber nicht $\emptyset \subsetneq \emptyset$, denn die beiden Seiten sind gleich."#),
    Block::Math(r#"\emptyset \subseteq A \quad\text{für jede Menge } A."#),
    Block::Paragraph(r#"Die Menge $\{\emptyset\}$ ist nicht leer. Sie hat genau ein Element, nämlich $\emptyset$. Damit gilt $\emptyset \neq \{\emptyset\}$, $\emptyset \in \{\emptyset\}$ und $\emptyset \subseteq \{\emptyset\}$. Das erste Zeichen $\in$ sagt, dass die leere Menge als Objekt in $\{\emptyset\}$ liegt. Das Zeichen $\subseteq$ sagt, dass jedes Element der linken Seite auch rechts liegt. Beide Aussagen sind wahr und bedeuten Verschiedenes."#),
    Block::Heading("Element oder Teilmenge"),
    Block::Paragraph(r#"Für Mengen ganzer Zahlen gilt zum Beispiel $2 \in \{1,2,3\}$ und $\{2\} \subseteq \{1,2,3\}$. Die Zahl $2$ ist ein Element. Die Menge $\{2\}$ ist eine Teilmenge. Dagegen ist $\{2\} \in \{1,2,3\}$ falsch, denn die Elemente von $\{1,2,3\}$ sind die Zahlen $1$, $2$ und $3$, nicht die Menge $\{2\}$. Ganze Zahlen behandeln wir hier als Elemente, nicht als Mengen. Eine Aussage wie $2 \subseteq \{1,2,3\}$ formulieren wir deshalb nicht."#),
];

const EX_A: &[Block] = &[
    Block::Paragraph(r#"Sei $A = \{1,2\}$ und $B = \{1,2,3\}$. Jedes Element von $A$ liegt in $B$, also $A \subseteq B$. Wegen $3 \in B$ und $3 \notin A$ ist $A \neq B$, also $A \subsetneq B$. Ausserdem gilt $B \not\subseteq A$."#),
];

const EX_B: &[Block] = &[
    Block::Paragraph(r#"Sei $A = \{1,2\}$. Dann gilt $A \subseteq A$, weil jedes Element von $A$ in $A$ liegt. Es gilt nicht $A \subsetneq A$, weil die Mengen gleich sind. Echte Teilmenge verlangt einen echten Unterschied."#),
];

const EX_C: &[Block] = &[
    Block::Paragraph(r#"Vergleiche $\emptyset$ und $\{\emptyset\}$. Es gilt $\emptyset \subseteq \{\emptyset\}$ und $\emptyset \in \{\emptyset\}$. Es gilt nicht $\emptyset = \{\emptyset\}$, denn die rechte Menge hat ein Element und die linke keines. Ebenso gilt nicht $\{\emptyset\} \in \{\emptyset\}$, denn das einzige Element rechts ist $\emptyset$, nicht $\{\emptyset\}$."#),
];

const SUMMARY: &[Block] = &[
    Block::Paragraph(r#"$A \subseteq B$ bedeutet: Jedes Element von $A$ liegt in $B$. $A \subsetneq B$ bedeutet zusätzlich $A \neq B$. Das Zeichen $\subset$ verwenden wir nicht. $\emptyset$ ist Teilmenge jeder Menge, aber $\emptyset \neq \{\emptyset\}$."#),
    Block::Bullets(&[
        r#"$\in$ fragt nach einem Element, $\subseteq$ vergleicht zwei Mengen."#,
        r#"$\{1\} \subseteq \{1,2\}$ ist wahr, $\{1\} \in \{1,2\}$ ist falsch."#,
        r#"$\emptyset \subseteq A$ ist immer wahr. $\emptyset \in A$ nur, wenn die leere Menge ausdrücklich Element von $A$ ist."#,
    ]),
];

const EXERCISES: &[Exercise] = &[
    Exercise {
        id: "l02e01",
        lesson_id: "l02",
        difficulty: Difficulty::Einstieg,
        prompt: r#"Wahr oder falsch? Es gilt $\{2,3\} \subseteq \{1,2,3,4\}$."#,
        hint: r#"Prüfe jedes Element der linken Menge einzeln."#,
        explanation: r#"Wahr. $2$ und $3$ liegen beide in $\{1,2,3,4\}$. Damit ist jedes Element der linken Menge auch Element der rechten."#,
        kind: ExerciseKind::TrueFalse { correct: true },
    },
    Exercise {
        id: "l02e02",
        lesson_id: "l02",
        difficulty: Difficulty::Einstieg,
        prompt: r#"Wahr oder falsch? Es gilt $\{1,2\} \subsetneq \{1,2\}$."#,
        hint: r#"Eine echte Teilmenge muss Teilmenge sein und zusätzlich ungleich."#,
        explanation: r#"Falsch. Zwar gilt $\{1,2\} \subseteq \{1,2\}$, aber die beiden Mengen sind gleich. Die Bedingung $A \neq B$ für $A \subsetneq B$ ist nicht erfüllt."#,
        kind: ExerciseKind::TrueFalse { correct: false },
    },
    Exercise {
        id: "l02e03",
        lesson_id: "l02",
        difficulty: Difficulty::Aufbau,
        prompt: r#"Wahr oder falsch? Es gilt $\emptyset = \{\emptyset\}$."#,
        hint: r#"Zähle die Elemente beider Seiten."#,
        explanation: r#"Falsch. $\emptyset$ hat keine Elemente. $\{\emptyset\}$ hat genau ein Element, nämlich $\emptyset$. Verschiedene Elementanzahl bedeutet verschiedene Mengen."#,
        kind: ExerciseKind::TrueFalse { correct: false },
    },
    Exercise {
        id: "l02e04",
        lesson_id: "l02",
        difficulty: Difficulty::Aufbau,
        prompt: r#"Welche Aussage ist wahr?"#,
        hint: r#"Entscheide bei jeder Option, ob zwei Mengen verglichen werden oder ob eine Menge als Element behauptet wird."#,
        explanation: r#"Wahr ist $\{1\} \subseteq \{1,2,3\}$, denn das einzige Element $1$ liegt rechts. $\{1,2,3\} \subseteq \{1,2\}$ scheitert an $3$. $\{1\} \in \{1,2,3\}$ ist falsch, weil rechts Zahlen stehen und nicht die Menge $\{1\}$. $\{1,2\} \subsetneq \{1,2\}$ ist falsch, weil die Seiten gleich sind."#,
        kind: ExerciseKind::SingleChoice {
            options: &[
                r#"$\{1,2,3\} \subseteq \{1,2\}$"#,
                r#"$\{1\} \in \{1,2,3\}$"#,
                r#"$\{1\} \subseteq \{1,2,3\}$"#,
                r#"$\{1,2\} \subsetneq \{1,2\}$"#,
            ],
            correct_index: 2,
        },
    },
    Exercise {
        id: "l02e05",
        lesson_id: "l02",
        difficulty: Difficulty::Vertiefung,
        prompt: r#"Wahr oder falsch? Die leere Menge ist Teilmenge jeder Menge."#,
        hint: r#"Eine Allaussage über die Elemente der leeren Menge hat kein Gegenbeispiel."#,
        explanation: r#"Wahr. Sei $A$ eine beliebige Menge. Es gibt kein $x$ mit $x \in \emptyset$ und $x \notin A$. Die definierende Bedingung von $\emptyset \subseteq A$ ist also erfüllt. Das gilt auch für $A = \emptyset$."#,
        kind: ExerciseKind::TrueFalse { correct: true },
    },
];

pub static LESSON: Lesson = Lesson {
    id: "l02",
    order: 2,
    title: "Teilmengen und die leere Menge",
    description: "⊆, ⊊, die leere Menge und der Unterschied zwischen ∈ und ⊆.",
    difficulty: Difficulty::Einstieg,
    objectives: &[
        r#"Du prüfst $A \subseteq B$ und $A \subsetneq B$ an endlichen Mengen."#,
        r#"Du verwendest $\subseteq$ für Teilmengen und $\subsetneq$ für echte Teilmengen und vermeidest das mehrdeutige $\subset$."#,
        r#"Du unterscheidest $\emptyset$ von $\{\emptyset\}$ sowie $\in$ von $\subseteq$."#,
    ],
    theory: THEORY,
    examples: &[
        Example { title: "Echte Teilmenge", blocks: EX_A },
        Example { title: "Jede Menge ist Teilmenge von sich", blocks: EX_B },
        Example { title: "Leer ist nicht dasselbe wie «enthält die leere Menge»", blocks: EX_C },
    ],
    misconceptions: &[
        Misconception {
            title: "∈ und ⊆ austauschen",
            text: r#"Aus 1 ∈ {1,2} folgt nicht {1} ∈ {1,2}. Die erste Aussage nennt eine Zahl als Element. Die zweite behauptet, die Menge {1} sei eines der Elemente. Richtig ist {1} ⊆ {1,2}."#,
        },
        Misconception {
            title: "Die leere Menge und {∅} gleichsetzen",
            text: r#"∅ hat null Elemente, {∅} hat ein Element. Beide Beziehungen ∅ ∈ {∅} und ∅ ⊆ {∅} sind wahr, die Gleichheit aber nicht. «Liegt als Element darin» und «ist Teilmenge» dürfen nicht zusammenfallen."#,
        },
    ],
    summary: SUMMARY,
    exercises: EXERCISES,
};
