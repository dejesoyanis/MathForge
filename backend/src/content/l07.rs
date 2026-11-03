use super::model::*;

const THEORY: &[Block] = &[
    Block::Heading("Gesetze, die für alle Mengen gelten"),
    Block::Paragraph(r#"Die Operationen aus den vorigen Lektionen genügen Gesetzen, die nicht von einer zufälligen Beispielmenge abhängen. Für alle Mengen $A$, $B$ und $C$ und, wo Komplemente vorkommen, für $A,B \subseteq U$ gilt:"#),
    Block::Bullets(&[
        r#"Kommutativität: $A \cup B = B \cup A$ und $A \cap B = B \cap A$."#,
        r#"Assoziativität: $(A \cup B) \cup C = A \cup (B \cup C)$ und ebenso für $\cap$."#,
        r#"Distributivität: $A \cap (B \cup C) = (A \cap B) \cup (A \cap C)$ und $A \cup (B \cap C) = (A \cup B) \cap (A \cup C)$."#,
        r#"Idempotenz: $A \cup A = A$ und $A \cap A = A$."#,
        r#"Absorption: $A \cup (A \cap B) = A$ und $A \cap (A \cup B) = A$."#,
        r#"Neutrale Elemente: $A \cup \emptyset = A$. Ist $A \subseteq U$, so gilt $A \cap U = A$."#,
        r#"Komplemente in $U$: $A \cup A^{c} = U$ und $A \cap A^{c} = \emptyset$."#,
    ]),
    Block::Heading("Was ein Elementbeweis ist"),
    Block::Paragraph(r#"Um $X \subseteq Y$ zu beweisen, nimmt man ein beliebiges $x \in X$ und zeigt $x \in Y$. Mehr darf über $x$ nicht vorausgesetzt werden. Um $X = Y$ zu beweisen, zeigt man beide Inklusionen $X \subseteq Y$ und $Y \subseteq X$. Gleichwertig kann man für ein beliebiges $x$ zeigen, dass $x \in X$ genau dann gilt, wenn $x \in Y$ gilt."#),
    Block::Paragraph(r#"Ein Zahlenbeispiel kann ein Gesetz widerlegen, wenn die beiden Seiten verschieden ausfallen. Es kann ein allgemeines Gesetz nicht beweisen. Auch eine Skizze ist eine Illustration und kein Ersatz für die Elementbedingung."#),
    Block::Heading("Ein vollständiger Elementbeweis"),
    Block::Paragraph(r#"Behauptung: Für alle Mengen $A$ und $B$ gilt $A \cap B \subseteq A$. Beweis: Sei $x \in A \cap B$. Nach Definition des Schnitts gilt $x \in A$ und $x \in B$. Also gilt insbesondere $x \in A$. Da $x$ beliebig aus $A \cap B$ gewählt war, folgt $A \cap B \subseteq A$."#),
];

const EX_A: &[Block] = &[
    Block::Paragraph(r#"Sei $A = \{1,2,3\}$, $B = \{2,4\}$ und $C = \{3,4\}$. Dann ist $B \cup C = \{2,3,4\}$ und $A \cap (B \cup C) = \{2,3\}$. Andererseits ist $A \cap B = \{2\}$, $A \cap C = \{3\}$ und $(A \cap B) \cup (A \cap C) = \{2,3\}$. Beide Seiten der Distributivität stimmen in diesem Beispiel überein. Das bestätigt die Rechnung, beweist das Gesetz aber nicht für alle Mengen."#),
];

const EX_B: &[Block] = &[
    Block::Paragraph(r#"Absorption an denselben $A$ und $B$: $A \cap B = \{2\}$ und $A \cup (A \cap B) = \{1,2,3\} = A$. Die hinzugefügten Elemente lagen schon in $A$. Genau das sagt $A \cup (A \cap B) = A$."#),
];

const EX_C: &[Block] = &[
    Block::Paragraph(r#"Das eine Beispiel $A = \{1\}$, $B = \{2\}$ widerlegt die falsche Behauptung $A \setminus B = B \setminus A$, weil die Seiten $\{1\}$ und $\{2\}$ sind. Ein Gegenbeispiel genügt zum Widerlegen. Zum Beweisen müsste die Argumentation für ein beliebiges Element funktionieren."#),
];

const PROOF: &str = r#"Sei $x \in A \cap B$. Nach Definition des Schnitts gilt dann $x \in A$ und $x \in B$. Aus $x \in A$ folgt, weil $x$ beliebig gewählt war, $A \cap B \subseteq A$. Die Zugehörigkeit von $x$ zu $B$ wird für diese Inklusion nicht mehr gebraucht."#;

const SUMMARY: &[Block] = &[
    Block::Paragraph(r#"Mengengesetze sind Allaussagen. Ein Beispiel widerlegt sie, wenn es scheitert, und illustriert sie, wenn es passt. Bewiesen wird eine Gleichheit durch zwei Inklusionen oder durch eine genau-dann-wenn-Bedingung für ein beliebiges Element."#),
];

const EXERCISES: &[Exercise] = &[
    Exercise {
        id: "l07e01",
        lesson_id: "l07",
        difficulty: Difficulty::Aufbau,
        prompt: r#"Wahr oder falsch? Für alle Mengen $A$, $B$ und $C$ gilt $A \cup (B \cap C) = (A \cup B) \cap (A \cup C)$."#,
        hint: r#"Das ist eine der beiden Distributivgesetze. Prüfe die Richtung der Verknüpfungen."#,
        explanation: r#"Wahr. Die Vereinigung distribuiert über den Schnitt. Ein Element $x$ liegt genau dann in $A$ oder in beiden Mengen $B$ und $C$, wenn es in $A \cup B$ und in $A \cup C$ liegt. Die andere Distributivität, Schnitt über Vereinigung, ist ebenfalls wahr, hier aber nicht gefragt."#,
        kind: ExerciseKind::TrueFalse { correct: true },
    },
    Exercise {
        id: "l07e02",
        lesson_id: "l07",
        difficulty: Difficulty::Vertiefung,
        prompt: r#"Wahr oder falsch? Die Gleichheit $\{1,2\} \cup \{2,3\} = \{2,3\} \cup \{1,2\}$ beweist bereits, dass die Vereinigung für alle Mengen kommutativ ist."#,
        hint: r#"Was leistet ein einzelnes Beispiel, und was leistet es nicht?"#,
        explanation: r#"Falsch. Die Rechnung ist korrekt und passt zur Kommutativität, aber ein Paar von Mengen ist kein Beweis für alle Mengen. Ein Beweis muss ein beliebiges Element der linken Seite auf die rechte Seite bringen und umgekehrt. Die Kommutativität selbst ist wahr; falsch ist nur die Behauptung, dieses Beispiel habe sie bereits bewiesen."#,
        kind: ExerciseKind::TrueFalse { correct: false },
    },
    Exercise {
        id: "l07e03",
        lesson_id: "l07",
        difficulty: Difficulty::Aufbau,
        prompt: r#"Sei $A = \{1,2,3\}$ und $B = \{2,4\}$. Bestimme $A \cup (A \cap B)$."#,
        hint: r#"Berechne zuerst den Schnitt. Das Absorptionsgesetz sagt dir, welches Ergebnis du erwarten darfst."#,
        explanation: r#"$A \cap B = \{2\}$. Die Vereinigung mit $A$ ist $\{1,2,3\}$. Das ist wieder $A$, wie es die Absorption $A \cup (A \cap B) = A$ allgemein sagt. Für die Aufgabe genügt die konkrete Menge $\{1,2,3\}$."#,
        kind: ExerciseKind::Set { correct: &[1, 2, 3] },
    },
    Exercise {
        id: "l07e04",
        lesson_id: "l07",
        difficulty: Difficulty::Vertiefung,
        prompt: r#"Beweise elementweise, dass für alle Mengen $A$ und $B$ gilt: $A \cap B \subseteq A$. Schreibe mit einem beliebigen Element $x$."#,
        hint: r#"Beginne mit «Sei $x \in A \cap B$» und verwende nur die Definition des Schnitts."#,
        explanation: PROOF,
        kind: ExerciseKind::FreeText { sample_solution: PROOF },
    },
    Exercise {
        id: "l07e05",
        lesson_id: "l07",
        difficulty: Difficulty::Aufbau,
        prompt: r#"Welcher Schritt beweist eine Mengengleichheit $X = Y$?"#,
        hint: r#"Gleichheit bedeutet beide Richtungen der Teilmengenbeziehung."#,
        explanation: r#"Man zeigt $X \subseteq Y$ und $Y \subseteq X$. Ein leerer Schnitt wäre Disjunktheit, kein Gleichheitsbeweis. Ein Zahlenbeispiel illustriert höchstens. $X \in Y$ würde behaupten, die Menge $X$ sei ein Element von $Y$, nicht dass beide dieselben Elemente haben."#,
        kind: ExerciseKind::SingleChoice {
            options: &[
                r#"Man zeigt $X \subseteq Y$ und $Y \subseteq X$."#,
                r#"Man zeigt nur $X \cap Y = \emptyset$."#,
                r#"Man rechnet ein einziges Zahlenbeispiel."#,
                r#"Man zeigt $X \in Y$."#,
            ],
            correct_index: 0,
        },
    },
];

pub static LESSON: Lesson = Lesson {
    id: "l07",
    order: 7,
    title: "Mengengesetze und Elementbeweise",
    description: "Kommutativität, Distributivität, Absorption und Beweise über ein beliebiges Element.",
    difficulty: Difficulty::Aufbau,
    objectives: &[
        r#"Du erkennst die grundlegenden Gesetze für $\cup$, $\cap$ und Komplemente."#,
        r#"Du unterscheidest Illustration, Gegenbeispiel und allgemeinen Beweis."#,
        r#"Du führst einen kurzen Elementbeweis für eine Inklusion."#,
    ],
    theory: THEORY,
    examples: &[
        Example { title: "Distributivität in Zahlen, ohne sie zu beweisen", blocks: EX_A },
        Example { title: "Absorption konkret", blocks: EX_B },
        Example { title: "Ein Gegenbeispiel widerlegt", blocks: EX_C },
    ],
    misconceptions: &[
        Misconception {
            title: "Ein passendes Beispiel ist schon der Beweis",
            text: r#"Wer nur {1,2} ∪ {2,3} ausrechnet, hat einen Fall gesehen. Der Allquantor «für alle Mengen» bleibt offen, solange das Argument nicht mit einem beliebigen x arbeitet."#,
        },
        Misconception {
            title: "Nur eine Inklusion zeigen",
            text: r#"Aus X ⊆ Y folgt nicht X = Y. Die echte Teilmenge {1} ⊆ {1,2} ist das Standardgegenbeispiel. Gleichheit braucht beide Richtungen."#,
        },
    ],
    summary: SUMMARY,
    exercises: EXERCISES,
};
