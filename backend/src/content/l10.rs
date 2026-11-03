use super::model::*;

const THEORY: &[Block] = &[
    Block::Heading("Funktionen sind besondere Relationen"),
    Block::Paragraph(r#"Eine Funktion $f$ von einer Definitionsmenge $A$ in eine Zielmenge $B$ ist eine Relation $f \subseteq A \times B$ mit einer zusätzlichen Bedingung: Zu jedem $a \in A$ gibt es genau ein $b \in B$ mit $(a,b) \in f$. Dieses eindeutige $b$ heisst Funktionswert und wird $f(a)$ geschrieben."#),
    Block::Math(r#"\forall a \in A\ \exists!\ b \in B:\ (a,b) \in f."#),
    Block::Paragraph(r#"Das Zeichen $\exists!$ bedeutet «es gibt genau ein». Zwei Fehler machen eine Relation zur Nicht-Funktion. Erstens: Ein Element von $A$ hat gar kein Paar. Zweitens: Ein Element von $A$ hat zwei Paare mit verschiedenen zweiten Komponenten. Jede Funktion ist damit eine Relation. Die Umkehrung gilt nicht."#),
    Block::Heading("Definitionsmenge, Zielmenge, Bild"),
    Block::Paragraph(r#"$A$ heisst Definitionsmenge, $B$ Zielmenge. Die Zielmenge muss die Funktionswerte enthalten, darf aber weitere Elemente enthalten. Das Bild von $f$ ist die Menge der tatsächlich angenommenen Werte."#),
    Block::Math(r#"f(A) = \{ b \in B \mid \text{es gibt ein } a \in A \text{ mit } f(a) = b \}."#),
    Block::Paragraph(r#"Es gilt stets $f(A) \subseteq B$. Gleichheit ist nicht verlangt. Für $f = \{(1,10),(2,20),(3,10)\}$ mit $A = \{1,2,3\}$ und $B = \{10,20,30\}$ ist $f$ eine Funktion, und das Bild ist $\{10,20\}$, eine echte Teilmenge der Zielmenge. Ob Funktionen injektiv oder surjektiv sind, vertiefen wir hier nicht. Für den Einstieg genügt: Definitionsmenge vollständig und eindeutig bedienen, Bild und Zielmenge auseinanderhalten."#),
    Block::Paragraph(r#"Paarmengen sind keine Mengen ganzer Zahlen. Ob eine angegebene Relation eine Funktion ist, wird als Aussage oder über das Bild als Anzahl geprüft."#),
];

const EX_A: &[Block] = &[
    Block::Paragraph(r#"Sei $A = \{1,2,3\}$, $B = \{4,5,6\}$ und $f = \{(1,4),(2,4),(3,5)\}$. Jedes Element von $A$ kommt genau einmal als erste Komponente vor. $f$ ist eine Funktion $A \to B$. Das Bild ist $\{4,5\}$. Die Zahl $6$ liegt in der Zielmenge, aber nicht im Bild. Zwei Argumente dürfen denselben Wert haben; das widerspricht der Definition nicht."#,
    ),
];

const EX_B: &[Block] = &[
    Block::Paragraph(r#"Sei $R = \{(1,2),(1,3)\}$ mit angestrebter Definitionsmenge $\{1\}$ und Zielmenge $\{2,3\}$. Die Zahl $1$ besitzt zwei verschiedene Werte. $R$ ist eine Relation, aber keine Funktion."#),
];

const EX_C: &[Block] = &[
    Block::Paragraph(r#"Sei $S = \{(1,2)\}$ mit angestrebter Definitionsmenge $\{1,2\}$ und Zielmenge $\{2\}$. Für $1$ gibt es genau einen Wert. Für $2 \in A$ gibt es keinen. $S$ ist auf dieser Definitionsmenge keine Funktion. Dieselbe Paarmenge wäre eine Funktion von $\{1\}$ nach $\{2\}$."#),
];

const PROOF: &str = r#"Eine Funktion $f \colon A \to B$ ist als Relation $f \subseteq A \times B$ definiert, die jedes $a \in A$ genau einem $b \in B$ zuordnet. Jede Funktion ist daher eine Relation. Nicht jede Relation ist eine Funktion. Gegenbeispiel: $R = \{(1,2),(1,3)\}$ ist eine Teilmenge von $\{1\} \times \{2,3\}$, also eine Relation von $\{1\}$ nach $\{2,3\}$. Sie ist keine Funktion, weil dem Argument $1$ zwei Werte zugeordnet sind."#;

const SUMMARY: &[Block] = &[
    Block::Bullets(&[
        r#"$f \colon A \to B$ ist eine Relation $f \subseteq A \times B$, die jedem $a \in A$ genau ein $b \in B$ zuordnet."#,
        r#"Das Bild $f(A)$ ist eine Teilmenge der Zielmenge, nicht automatisch die ganze Zielmenge."#,
        r#"Fehlende oder mehrfache Werte für ein $a \in A$ zerstören die Funktionseigenschaft."#,
    ]),
];

const EXERCISES: &[Exercise] = &[
    Exercise {
        id: "l10e01",
        lesson_id: "l10",
        difficulty: Difficulty::Einstieg,
        prompt: r#"Wahr oder falsch? $\{(1,4),(2,4),(3,5)\}$ ist eine Funktion von $\{1,2,3\}$ nach $\{4,5,6\}$."#,
        hint: r#"Zähle für jedes Element der Definitionsmenge die zugeordneten Werte. Die Zielmenge darf unbenutzte Elemente enthalten."#,
        explanation: r#"Wahr. $1 \mapsto 4$, $2 \mapsto 4$ und $3 \mapsto 5$. Jedes Argument hat genau einen Wert, und alle Werte liegen in der Zielmenge. Dass $4$ zweimal vorkommt und $6$ nicht vorkommt, ist erlaubt."#,
        kind: ExerciseKind::TrueFalse { correct: true },
    },
    Exercise {
        id: "l10e02",
        lesson_id: "l10",
        difficulty: Difficulty::Einstieg,
        prompt: r#"Wahr oder falsch? $\{(1,2),(1,3)\}$ ist eine Funktion von $\{1\}$ nach $\{2,3\}$."#,
        hint: r#"Genau ein Wert pro Argument, nicht mindestens einer."#,
        explanation: r#"Falsch. Dem Element $1$ werden $2$ und $3$ zugeordnet. Eine Funktion darf pro Argument nur einen Wert haben. Als Relation existiert die Paarmenge trotzdem."#,
        kind: ExerciseKind::TrueFalse { correct: false },
    },
    Exercise {
        id: "l10e03",
        lesson_id: "l10",
        difficulty: Difficulty::Aufbau,
        prompt: r#"Wahr oder falsch? $\{(1,2)\}$ ist eine Funktion von $\{1,2\}$ nach $\{2\}$."#,
        hint: r#"Prüfe nicht nur $1$, sondern jedes Element der genannten Definitionsmenge."#,
        explanation: r#"Falsch. $2$ gehört zur Definitionsmenge, hat aber kein Paar. Dieselbe einelementige Relation wäre eine Funktion von $\{1\}$ nach $\{2\}$, nicht von $\{1,2\}$ nach $\{2\}$."#,
        kind: ExerciseKind::TrueFalse { correct: false },
    },
    Exercise {
        id: "l10e04",
        lesson_id: "l10",
        difficulty: Difficulty::Aufbau,
        prompt: r#"Die Funktion $f = \{(1,7),(2,8),(3,7)\}$ hat die Definitionsmenge $\{1,2,3\}$. Wie viele Elemente hat das Bild von $f$?"#,
        hint: r#"Gleiche Funktionswerte zählen im Bild nur einmal."#,
        explanation: r#"Die angenommenen Werte sind $7$ und $8$. Der Wert $7$ wird zweimal angenommen und ist trotzdem nur ein Element des Bildes. $|f(A)| = 2$."#,
        kind: ExerciseKind::Numeric { correct: 2 },
    },
    Exercise {
        id: "l10e05",
        lesson_id: "l10",
        difficulty: Difficulty::Aufbau,
        prompt: r#"Sei $f = \{(1,2),(2,3)\}$ eine Funktion mit Definitionsmenge $\{1,2\}$ und Zielmenge $\{1,2,3\}$. Welche Menge ist das Bild?"#,
        hint: r#"Das Bild enthält die zweiten Komponenten, nicht die Paare und nicht automatisch die ganze Zielmenge."#,
        explanation: r#"Die Funktionswerte sind $2$ und $3$. Das Bild ist $\{2,3\}$. Die Zielmenge $\{1,2,3\}$ ist grösser, weil $1$ nicht angenommen wird. Die Paarmenge selbst ist $f$, nicht das Bild."#,
        kind: ExerciseKind::SingleChoice {
            options: &[
                r#"$\{1,2,3\}$"#,
                r#"$\{2,3\}$"#,
                r#"$\{1,2\}$"#,
                r#"$\{(1,2),(2,3)\}$"#,
            ],
            correct_index: 1,
        },
    },
    Exercise {
        id: "l10e06",
        lesson_id: "l10",
        difficulty: Difficulty::Vertiefung,
        prompt: r#"Erkläre, warum jede Funktion eine Relation ist, aber nicht jede Relation eine Funktion. Gib ein konkretes Gegenbeispiel mit einer kleinen Definitionsmenge an."#,
        hint: r#"Starte bei der Definition $f \subseteq A \times B$ und verletze danach die Eindeutigkeit an einem einzigen Argument."#,
        explanation: PROOF,
        kind: ExerciseKind::FreeText { sample_solution: PROOF },
    },
];

pub static LESSON: Lesson = Lesson {
    id: "l10",
    order: 10,
    title: "Funktionen als Relationen",
    description: "Definitionsmenge, Zielmenge und Bild: wann eine Relation eine Funktion ist.",
    difficulty: Difficulty::Vertiefung,
    objectives: &[
        r#"Du prüfst, ob eine endliche Paarmenge eine Funktion von einer genannten Definitionsmenge in eine Zielmenge ist."#,
        r#"Du bildest das Bild und unterscheidest es von der Zielmenge."#,
        r#"Du begründest an einem Gegenbeispiel, dass nicht jede Relation eine Funktion ist."#,
    ],
    theory: THEORY,
    examples: &[
        Example { title: "Eine Funktion mit kleinerem Bild", blocks: EX_A },
        Example { title: "Zwei Werte für ein Argument", blocks: EX_B },
        Example { title: "Ein Argument ohne Wert", blocks: EX_C },
    ],
    misconceptions: &[
        Misconception {
            title: "Zielmenge und Bild gleichsetzen",
            text: r#"Die Zielmenge wird mit der Funktion genannt. Das Bild ist das, was wirklich herauskommt. {10,20} kann das Bild einer Funktion nach {10,20,30} sein."#,
        },
        Misconception {
            title: "Jeden Pfeil schon für eine Funktion halten",
            text: r#"Eine Liste von Paaren ist immer eine Relation, sobald sie im passenden kartesischen Produkt liegt. Zur Funktion wird sie erst, wenn jedes Element der Definitionsmenge genau einmal als erste Komponente vorkommt."#,
        },
    ],
    summary: SUMMARY,
    exercises: EXERCISES,
};
