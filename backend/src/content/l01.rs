use super::model::*;

const THEORY: &[Block] = &[
    Block::Heading("Mengen und Elemente"),
    Block::Paragraph(r#"Eine Menge fasst Objekte zu einem Ganzen zusammen. In diesem Kurs sind die Objekte ganze Zahlen, und die Mengen sind endlich und explizit angegeben. Die Menge mit den Elementen $1$, $2$ und $3$ schreiben wir $A = \{1,2,3\}$. Geschweifte Klammern markieren die Aufzählung. Es gibt keine Reihenfolge der Elemente und kein mehrfaches Enthaltensein: Ein Objekt ist Element oder es ist es nicht."#),
    Block::Paragraph(r#"Die Aussage $x \in A$ bedeutet: $x$ ist ein Element von $A$. Die Verneinung ist $x \notin A$. Für $A = \{1,2,3\}$ gilt also $2 \in A$ und $4 \notin A$. Das Zeichen $\in$ verbindet ein Element mit einer Menge. Es ist kein Vergleich zweier Mengen."#),
    Block::Heading("Gleichheit durch dieselben Elemente"),
    Block::Paragraph(r#"Zwei Mengen sind genau dann gleich, wenn sie dieselben Elemente enthalten. Das ist das Extensionalitätsprinzip. Formal gilt $A = B$ genau dann, wenn für jedes Objekt $x$ die Aussagen $x \in A$ und $x \in B$ zugleich wahr oder zugleich falsch sind, kurz $x \in A \leftrightarrow x \in B$."#),
    Block::Math(r#"\{1,2,3\} = \{3,2,1\} = \{1,1,2,3\}."#),
    Block::Paragraph(r#"Die mittlere Menge nennt dieselben Elemente in anderer Reihenfolge. Die rechte Menge nennt $1$ doppelt. In einer Menge erzeugt das keine zusätzlichen Elemente. Deshalb sind alle drei Schreibweisen Namen für dieselbe Menge. Ungleich sind Mengen, sobald ein Element nur auf einer Seite vorkommt: $3 \in \{1,2,3\}$, aber $3 \notin \{1,2\}$, also $\{1,2,3\} \neq \{1,2\}$."#),
    Block::Heading("Wie wir Mengen bilden"),
    Block::Paragraph(r#"Wir verwenden die aufzählende Schreibweise für klar abgegrenzte endliche Mengen. Wenn eine Menge durch eine Eigenschaft beschrieben wird, nennen wir den Grundbereich dazu, zum Beispiel die geraden Zahlen in $\{1,2,3,4,5,6\}$. Eine Komprehension ohne jede Grundmenge, etwa «die Menge aller Mengen», verwenden wir nicht. Sie ist für diesen Kurs unnötig und in der naiven Mengenlehre eine Quelle von Widersprüchen."#),
];

const EX_A: &[Block] = &[
    Block::Paragraph(r#"Sei $A = \{1,2,3\}$. Dann gilt $1 \in A$, $2 \in A$, $3 \in A$ und $0 \notin A$. Die Menge hat drei verschiedene Elemente."#),
];

const EX_B: &[Block] = &[
    Block::Paragraph(r#"Es gilt $\{4,1,4,2\} = \{1,2,4\}$. Links wird $4$ zweimal genannt, rechts steht jedes Element einmal. Sortiert man die verschiedenen Elemente aufsteigend, entsteht beide Male die Normalform $\{1,2,4\}$."#),
];

const EX_C: &[Block] = &[
    Block::Paragraph(r#"Sei $A = \{1,2\}$ und $B = \{1,2,2\}$. Dann ist $A = B$. Sei $C = \{1,2,3\}$. Wegen $3 \in C$ und $3 \notin A$ ist $A \neq C$."#),
];

const SUMMARY: &[Block] = &[
    Block::Paragraph(r#"Eine Menge ist durch ihre Elemente bestimmt. $x \in A$ heisst, dass $x$ Element von $A$ ist. $A = B$ gilt genau dann, wenn $A$ und $B$ dieselben Elemente haben. Reihenfolge und Wiederholungen in der Aufzählung ändern die Menge nicht."#),
    Block::Bullets(&[
        r#"$\in$ steht zwischen Element und Menge, nicht zwischen zwei Mengen."#,
        r#"$\{1,2,2\} = \{2,1\}$, aber $\{1,2\} \neq \{1,2,3\}$."#,
        r#"Mengen in diesem Kurs sind endlich und ausdrücklich beschrieben."#,
    ]),
];

const EXERCISES: &[Exercise] = &[
    Exercise {
        id: "l01e01",
        lesson_id: "l01",
        difficulty: Difficulty::Einstieg,
        prompt: r#"Wahr oder falsch? Es gilt $\{4,1,4,2\} = \{1,2,4\}$."#,
        hint: r#"Prüfe, welche verschiedenen Zahlen links und rechts vorkommen."#,
        explanation: r#"Wahr. Links kommen genau die Zahlen $1$, $2$ und $4$ vor; die zweite $4$ erzeugt kein neues Element. Rechts stehen dieselben drei Elemente. Nach dem Extensionalitätsprinzip sind die Mengen gleich."#,
        kind: ExerciseKind::TrueFalse { correct: true },
    },
    Exercise {
        id: "l01e02",
        lesson_id: "l01",
        difficulty: Difficulty::Einstieg,
        prompt: r#"Wahr oder falsch? Es gilt $\{1,2\} = \{1,2,3\}$."#,
        hint: r#"Suche ein Element, das nur in einer der beiden Mengen liegt."#,
        explanation: r#"Falsch. $3 \in \{1,2,3\}$, aber $3 \notin \{1,2\}$. Da nicht dieselben Elemente vorliegen, sind die Mengen verschieden."#,
        kind: ExerciseKind::TrueFalse { correct: false },
    },
    Exercise {
        id: "l01e03",
        lesson_id: "l01",
        difficulty: Difficulty::Aufbau,
        prompt: r#"Welche Aussage ist korrekt?"#,
        hint: r#"Gleichheit von Mengen hängt nur an den Elementen, nicht an der Schreibweise."#,
        explanation: r#"Richtig ist die zweite Aussage. $\{1,2,3\}$ und $\{3,2,1\}$ enthalten dieselben Elemente, also sind sie gleich. Die Reihenfolge ist unwesentlich. $\{1,1\}$ hat nur ein Element. Eine Zahl ist nicht dasselbe wie die einelementige Menge, die diese Zahl enthält: $1 \neq \{1\}$."#,
        kind: ExerciseKind::SingleChoice {
            options: &[
                r#"In einer Menge ist die Reihenfolge der Elemente wesentlich."#,
                r#"$\{1,2,3\}$ und $\{3,2,1\}$ sind gleich, weil sie dieselben Elemente enthalten."#,
                r#"Die Menge $\{1,1\}$ hat zwei Elemente."#,
                r#"Es gilt $1 = \{1\}$."#,
            ],
            correct_index: 1,
        },
    },
    Exercise {
        id: "l01e04",
        lesson_id: "l01",
        difficulty: Difficulty::Aufbau,
        prompt: r#"Schreibe die Menge, die durch die Aufzählung $5,1,5,3,1$ bestimmt ist, in normalisierter Form."#,
        hint: r#"Jede Zahl nur einmal verwenden. Die übliche Normalform sortiert aufsteigend; für die Korrektur ist die Reihenfolge egal."#,
        explanation: r#"Die verschiedenen Elemente sind $1$, $3$ und $5$. Also ist die Menge $\{1,3,5\}$. Dieselbe Menge ist $\{5,3,1\}$ oder $\{1,5,5,3\}$."#,
        kind: ExerciseKind::Set { correct: &[1, 3, 5] },
    },
    Exercise {
        id: "l01e05",
        lesson_id: "l01",
        difficulty: Difficulty::Aufbau,
        prompt: r#"Wie viele verschiedene Elemente hat die Menge $\{-1,0,-1,2,0\}$?"#,
        hint: r#"Zähle gleiche Zahlen nur einmal. Auch $0$ und negative Zahlen sind Elemente."#,
        explanation: r#"Die verschiedenen Elemente sind $-1$, $0$ und $2$. Die Menge hat $3$ Elemente. Wiederholungen von $-1$ und $0$ ändern die Anzahl nicht."#,
        kind: ExerciseKind::Numeric { correct: 3 },
    },
];

pub static LESSON: Lesson = Lesson {
    id: "l01",
    order: 1,
    title: "Mengen, Elemente und Gleichheit",
    description: "Aufzählungen, das Zeichen ∈ und wann zwei Mengen gleich sind.",
    difficulty: Difficulty::Einstieg,
    objectives: &[
        r#"Du erkennst Elemente einer explizit gegebenen endlichen Menge und verwendest $\in$ und $\notin$."#,
        r#"Du entscheidest Mengengleichheit allein über die Elemente, unabhängig von Reihenfolge und Duplikaten."#,
        r#"Du unterscheidest eine Zahl von der Menge, die diese Zahl enthält."#,
    ],
    theory: THEORY,
    examples: &[
        Example { title: "Elemente ablesen", blocks: EX_A },
        Example { title: "Dieselbe Menge, andere Aufzählung", blocks: EX_B },
        Example { title: "Gleichheit und Ungleichheit", blocks: EX_C },
    ],
    misconceptions: &[
        Misconception {
            title: "Reihenfolge oder Duplikate erzeugen eine neue Menge",
            text: r#"In einer Liste wäre (1,2) etwas anderes als (2,1). In einer Menge nicht. {1,2,2} ist keine «Menge mit drei Plätzen», sondern die Menge {1,2}."#,
        },
        Misconception {
            title: "Element und einelementige Menge verwechseln",
            text: r#"1 ist eine Zahl. {1} ist eine Menge mit einem Element. Deshalb ist 1 ∈ {1} wahr, aber 1 = {1} falsch. Diese Unterscheidung wird bei Teilmengen noch wichtiger."#,
        },
    ],
    summary: SUMMARY,
    exercises: EXERCISES,
};
