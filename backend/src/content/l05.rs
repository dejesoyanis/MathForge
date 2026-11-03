use super::model::*;

const THEORY: &[Block] = &[
    Block::Heading("Alle Teilmengen auf einmal"),
    Block::Paragraph(r#"Die Potenzmenge $\mathcal{P}(A)$ ist die Menge aller Teilmengen von $A$. Ihre Elemente sind selbst Mengen. Für eine endliche Menge kann man sie vollständig aufzählen."#),
    Block::Math(r#"\mathcal{P}(A) = \{ B \mid B \subseteq A \}."#),
    Block::Paragraph(r#"Die Bedingung $B \subseteq A$ ist relativ zu den Teilmengen einer bereits gegebenen Menge $A$ formuliert. Wir bilden hier keine Potenzmenge «aller Mengen». Zwei Teilmengen sind immer dabei: $\emptyset \subseteq A$ und $A \subseteq A$. Also gilt stets $\emptyset \in \mathcal{P}(A)$ und $A \in \mathcal{P}(A)$."#),
    Block::Heading("Wie viele Teilmengen?"),
    Block::Paragraph(r#"Hat eine endliche Menge $A$ genau $n$ Elemente, so hat $\mathcal{P}(A)$ genau $2^{n}$ Elemente. Für jedes Element von $A$ gibt es beim Bilden einer Teilmenge zwei Möglichkeiten: aufnehmen oder weglassen. Die Entscheidungen sind unabhängig, deshalb multipliziert sich der Faktor $2$ genau $n$-mal."#),
    Block::Math(r#"|\mathcal{P}(A)| = 2^{|A|} \quad\text{für endliches } A."#),
    Block::Paragraph(r#"Insbesondere ist $|\mathcal{P}(\emptyset)| = 2^{0} = 1$. Die einzige Teilmenge der leeren Menge ist $\emptyset$ selbst, also $\mathcal{P}(\emptyset) = \{\emptyset\}$. Diese Potenzmenge ist nicht leer. Das ist derselbe Unterschied wie zwischen $\emptyset$ und $\{\emptyset\}$."#),
    Block::Paragraph(r#"Die Elemente von $\mathcal{P}(A)$ sind Mengen, oft verschachtelt. Das Antwortformat für Mengen ganzer Zahlen kann sie nicht entgegennehmen. Aufgaben zur Potenzmenge werden deshalb als Zahl, als Wahr-oder-falsch-Aussage oder als Auswahl gestellt."#),
];

const EX_A: &[Block] = &[
    Block::Paragraph(r#"$\mathcal{P}(\emptyset) = \{\emptyset\}$. Die Aufzählung hat ein Element. $|\mathcal{P}(\emptyset)| = 1 = 2^{0}$."#),
];

const EX_B: &[Block] = &[
    Block::Paragraph(r#"$\mathcal{P}(\{1\}) = \{\emptyset, \{1\}\}$. Es gibt die Teilmenge ohne $1$ und die Teilmenge mit $1$. Die Mächtigkeit ist $2 = 2^{1}$."#),
];

const EX_C: &[Block] = &[
    Block::Paragraph(r#"$\mathcal{P}(\{1,2\}) = \{\emptyset, \{1\}, \{2\}, \{1,2\}\}$. Das sind $4 = 2^{2}$ Teilmengen. $\{1,2\}$ ist ein Element der Potenzmenge, aber $\{1,2,3\}$ ist keines, weil $\{1,2,3\} \not\subseteq \{1,2\}$."#),
];

const SUMMARY: &[Block] = &[
    Block::Bullets(&[
        r#"$\mathcal{P}(A)$ enthält genau die Mengen $B$ mit $B \subseteq A$."#,
        r#"Immer gilt $\emptyset \in \mathcal{P}(A)$ und $A \in \mathcal{P}(A)$."#,
        r#"Für endliches $A$ gilt $|\mathcal{P}(A)| = 2^{|A|}$, also $|\mathcal{P}(\emptyset)| = 1$."#,
    ]),
];

const EXERCISES: &[Exercise] = &[
    Exercise {
        id: "l05e01",
        lesson_id: "l05",
        difficulty: Difficulty::Einstieg,
        prompt: r#"Wie viele Elemente hat $\mathcal{P}(\{1,2,3,4\})$?"#,
        hint: r#"Bestimme zuerst $|\{1,2,3,4\}|$ und verwende dann $2^{n}$."#,
        explanation: r#"Die Menge $\{1,2,3,4\}$ hat $4$ Elemente. Jede Teilmenge entsteht durch unabhängige Ja-Nein-Entscheidungen für diese vier Zahlen. Es gibt $2^{4} = 16$ Teilmengen."#,
        kind: ExerciseKind::Numeric { correct: 16 },
    },
    Exercise {
        id: "l05e02",
        lesson_id: "l05",
        difficulty: Difficulty::Aufbau,
        prompt: r#"Wie viele Elemente hat $\mathcal{P}(\emptyset)$?"#,
        hint: r#"Welche Mengen $B$ erfüllen $B \subseteq \emptyset$?"#,
        explanation: r#"Nur $\emptyset$ ist Teilmenge von $\emptyset$. Also ist $\mathcal{P}(\emptyset) = \{\emptyset\}$ und hat $1$ Element. Die Formel $2^{0} = 1$ liefert dasselbe. Die Potenzmenge ist nicht die leere Menge."#,
        kind: ExerciseKind::Numeric { correct: 1 },
    },
    Exercise {
        id: "l05e03",
        lesson_id: "l05",
        difficulty: Difficulty::Einstieg,
        prompt: r#"Wahr oder falsch? Es gilt $\emptyset \in \mathcal{P}(\{1,2\})$."#,
        hint: r#"Liegt die leere Menge in der Menge aller Teilmengen?"#,
        explanation: r#"Wahr. $\emptyset \subseteq \{1,2\}$, also ist $\emptyset$ eines der Elemente von $\mathcal{P}(\{1,2\})$."#,
        kind: ExerciseKind::TrueFalse { correct: true },
    },
    Exercise {
        id: "l05e04",
        lesson_id: "l05",
        difficulty: Difficulty::Aufbau,
        prompt: r#"Wahr oder falsch? Es gilt $\{1,2,3\} \in \mathcal{P}(\{1,2\})$."#,
        hint: r#"Ein Element der Potenzmenge muss Teilmenge von $\{1,2\}$ sein."#,
        explanation: r#"Falsch. $\{1,2,3\} \subseteq \{1,2\}$ wäre nötig, scheitert aber an $3$. Die vier Elemente von $\mathcal{P}(\{1,2\})$ sind $\emptyset$, $\{1\}$, $\{2\}$ und $\{1,2\}$."#,
        kind: ExerciseKind::TrueFalse { correct: false },
    },
    Exercise {
        id: "l05e05",
        lesson_id: "l05",
        difficulty: Difficulty::Vertiefung,
        prompt: r#"Welche Menge ist $\mathcal{P}(\{0\})$?"#,
        hint: r#"Es gibt genau zwei Teilmengen: die ohne $0$ und die mit $0$."#,
        explanation: r#"Die Teilmengen von $\{0\}$ sind $\emptyset$ und $\{0\}$. Also ist $\mathcal{P}(\{0\}) = \{\emptyset, \{0\}\}$. $\emptyset$ wäre die leere Aufzählung, $\{\emptyset\}$ lässt $\{0\}$ weg, und $\{\{0\}\}$ lässt $\emptyset$ weg."#,
        kind: ExerciseKind::SingleChoice {
            options: &[
                r#"$\emptyset$"#,
                r#"$\{\emptyset\}$"#,
                r#"$\{\emptyset, \{0\}\}$"#,
                r#"$\{\{0\}\}$"#,
            ],
            correct_index: 2,
        },
    },
];

pub static LESSON: Lesson = Lesson {
    id: "l05",
    order: 5,
    title: "Potenzmengen",
    description: "Alle Teilmengen einer endlichen Menge und die Formel 2 hoch n.",
    difficulty: Difficulty::Aufbau,
    objectives: &[
        r#"Du beschreibst $\mathcal{P}(A)$ als Menge aller Teilmengen von $A$."#,
        r#"Du berechnest $|\mathcal{P}(A)| = 2^{|A|}$ für kleine endliche Mengen, einschliesslich $\emptyset$."#,
        r#"Du unterscheidest Elemente der Potenzmenge von Elementen von $A$."#,
    ],
    theory: THEORY,
    examples: &[
        Example { title: "Potenzmenge der leeren Menge", blocks: EX_A },
        Example { title: "Eine einelementige Menge", blocks: EX_B },
        Example { title: "Zwei Elemente, vier Teilmengen", blocks: EX_C },
    ],
    misconceptions: &[
        Misconception {
            title: "Die Potenzmenge der leeren Menge ist leer",
            text: r#"P(∅) enthält eine Teilmenge, nämlich ∅. Die Menge {∅} hat ein Element. Wer |P(∅)| = 0 setzt, verwechselt «keine Elemente in ∅» mit «keine Teilmengen von ∅»."#,
        },
        Misconception {
            title: "Teilmenge und Element der Potenzmenge auseinanderhalten",
            text: r#"1 ∈ {1,2} ist eine Aussage über die Ausgangsmenge. {1} ∈ P({1,2}) ist die passende Aussage auf der nächsten Stufe. {1} ⊆ {1,2} und {1} ∈ P({1,2}) beschreiben denselben Sachverhalt in zwei Sprachen."#,
        },
    ],
    summary: SUMMARY,
    exercises: EXERCISES,
};
