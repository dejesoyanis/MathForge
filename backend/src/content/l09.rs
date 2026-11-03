use super::model::*;

const THEORY: &[Block] = &[
    Block::Heading("Relationen als Mengen von Paaren"),
    Block::Paragraph(r#"Eine Relation zwischen $A$ und $B$ ist eine Menge von geordneten Paaren, genauer eine Teilmenge des kartesischen Produkts. Eine Relation auf einer Menge $A$ ist eine Teilmenge von $A \times A$."#),
    Block::Math(r#"R \subseteq A \times A."#),
    Block::Paragraph(r#"Statt $(x,y) \in R$ schreibt man oft $x\,R\,y$. Für den Einstieg bleiben wir bei der Mengenform. Alles, was du über Teilmengen weisst, gilt auch hier: Vereinigung, Schnitt und Gleichheit von Relationen sind Vereinigung, Schnitt und Gleichheit ihrer Paarmengen."#),
    Block::Heading("Drei Eigenschaften"),
    Block::Paragraph(r#"Sei $R \subseteq A \times A$. Die Eigenschaften beziehen sich auf diese Trägermenge $A$."#),
    Block::Bullets(&[
        r#"$R$ ist reflexiv auf $A$, wenn für jedes $x \in A$ gilt $(x,x) \in R$."#,
        r#"$R$ ist symmetrisch, wenn aus $(x,y) \in R$ stets $(y,x) \in R$ folgt."#,
        r#"$R$ ist transitiv, wenn aus $(x,y) \in R$ und $(y,z) \in R$ stets $(x,z) \in R$ folgt."#,
    ]),
    Block::Paragraph(r#"Reflexivität hängt an $A$. Fehlt auch nur ein Diagonalelement $(a,a)$ für ein $a \in A$, ist $R$ auf $A$ nicht reflexiv. Symmetrie und Transitivität sind Implikationen. Wenn es gar kein Paar gibt, das die Voraussetzung erfüllt, ist die Implikation wahr. Die leere Relation $R = \emptyset$ auf einer nicht leeren Menge $A$ ist deshalb symmetrisch und transitiv, aber nicht reflexiv. Daraus folgt auch: Symmetrie und Transitivität zusammen erzwingen nicht die Reflexivität."#),
    Block::Paragraph(r#"Wir führen hier noch keine Äquivalenzrelationen als eigenes Kapitel ein. Eine Relation, die alle drei Eigenschaften hat, ist der Begriff, auf den die nächste Vertiefung aufbauen würde. Aufgaben bleiben bei kleinen, vollständig angegebenen Paarmengen."#),
];

const EX_A: &[Block] = &[
    Block::Paragraph(r#"Sei $A = \{1,2\}$ und $R = \{(1,1),(2,2)\}$. Für beide Elemente von $A$ liegt das Diagonalelement in $R$, also ist $R$ reflexiv. Zu jedem Paar ist das gespiegelte Paar dasselbe Diagonalelement, also ist $R$ symmetrisch. Die einzigen Ketten bleiben auf der Diagonale, also ist $R$ transitiv."#),
];

const EX_B: &[Block] = &[
    Block::Paragraph(r#"Sei $A = \{1,2,3\}$ und $S = \{(1,2),(2,3)\}$. $S$ ist nicht reflexiv, weil $(1,1)$ fehlt. $S$ ist nicht symmetrisch, weil $(2,1)$ fehlt. $S$ ist nicht transitiv, weil $(1,2)$ und $(2,3)$ in $S$ liegen, $(1,3)$ aber nicht."#),
];

const EX_C: &[Block] = &[
    Block::Paragraph(r#"Sei $A = \{1,2,3\}$ und $T = \{(1,2),(2,3),(1,3)\}$. Die Kette $(1,2)$, $(2,3)$ verlangt $(1,3)$, und dieses Paar liegt in $T$. Weitere Ketten mit einer mittleren Komponente gibt es nicht. $T$ ist transitiv. Symmetrisch ist $T$ nicht, weil $(2,1)$ fehlt. Reflexiv ist $T$ ebenfalls nicht."#),
];

const SUMMARY: &[Block] = &[
    Block::Paragraph(r#"Eine Relation auf $A$ ist eine Teilmenge von $A \times A$. Reflexivität fordert alle Paare $(x,x)$ für $x \in A$. Symmetrie dreht Paare um. Transitivität schliesst Ketten. Die leere Relation auf einer nicht leeren Menge zeigt, dass Symmetrie und Transitivität ohne Reflexivität vorkommen."#),
];

const EXERCISES: &[Exercise] = &[
    Exercise {
        id: "l09e01",
        lesson_id: "l09",
        difficulty: Difficulty::Einstieg,
        prompt: r#"Wahr oder falsch? Die Relation $R = \{(1,1),(2,2)\}$ auf $A = \{1,2\}$ ist reflexiv."#,
        hint: r#"Prüfe $(x,x)$ für jedes Element von $A$, nicht nur für ein Beispiel."#,
        explanation: r#"Wahr. Für $x = 1$ und für $x = 2$ liegt $(x,x)$ in $R$. Mehr Elemente hat $A$ nicht."#,
        kind: ExerciseKind::TrueFalse { correct: true },
    },
    Exercise {
        id: "l09e02",
        lesson_id: "l09",
        difficulty: Difficulty::Einstieg,
        prompt: r#"Wahr oder falsch? Die Relation $R = \{(1,2),(2,1)\}$ auf $A = \{1,2\}$ ist symmetrisch."#,
        hint: r#"Zu jedem vorhandenen Paar muss das gespiegelte Paar ebenfalls vorhanden sein."#,
        explanation: r#"Wahr. Das Spiegelbild von $(1,2)$ ist $(2,1)$ und umgekehrt. Beide liegen in $R$. Reflexiv ist $R$ nicht, weil $(1,1)$ und $(2,2)$ fehlen. Danach ist hier nicht gefragt."#,
        kind: ExerciseKind::TrueFalse { correct: true },
    },
    Exercise {
        id: "l09e03",
        lesson_id: "l09",
        difficulty: Difficulty::Aufbau,
        prompt: r#"Wahr oder falsch? Die Relation $R = \{(1,2),(2,3),(1,3)\}$ auf $A = \{1,2,3\}$ ist transitiv."#,
        hint: r#"Suche alle Paare von Paaren, deren Ende und Anfang zusammenpassen."#,
        explanation: r#"Wahr. Die einzige Kette mit verschiedenen Paaren ist $(1,2)$ zusammen mit $(2,3)$. Das verlangte Paar $(1,3)$ liegt in $R$. Von $3$ geht kein Paar aus, und $(1,3)$ hat kein anschliessendes Paar mit erster Komponente $3$."#,
        kind: ExerciseKind::TrueFalse { correct: true },
    },
    Exercise {
        id: "l09e04",
        lesson_id: "l09",
        difficulty: Difficulty::Vertiefung,
        prompt: r#"Sei $R = \emptyset$ eine Relation auf $A = \{1,2\}$. Welche Beschreibung trifft zu?"#,
        hint: r#"Eine Implikation ohne erfüllte Voraussetzung ist wahr. Reflexivität ist keine solche Implikation."#,
        explanation: r#"Die leere Relation ist symmetrisch und transitiv, weil es kein Paar gibt, dessen Spiegelbild oder Fortsetzung fehlen könnte. Sie ist nicht reflexiv, weil $(1,1) \notin R$ und $1 \in A$. Deshalb trifft die dritte Aussage zu."#,
        kind: ExerciseKind::SingleChoice {
            options: &[
                r#"$R$ ist reflexiv."#,
                r#"$R$ ist nicht symmetrisch."#,
                r#"$R$ ist symmetrisch und transitiv, aber nicht reflexiv."#,
                r#"$R$ ist transitiv, aber nicht symmetrisch."#,
            ],
            correct_index: 2,
        },
    },
    Exercise {
        id: "l09e05",
        lesson_id: "l09",
        difficulty: Difficulty::Vertiefung,
        prompt: r#"Wahr oder falsch? Jede symmetrische und transitive Relation ist reflexiv."#,
        hint: r#"Betrachte die leere Relation auf $\{1\}$."#,
        explanation: r#"Falsch. $R = \emptyset$ auf $A = \{1\}$ ist symmetrisch und transitiv, aber nicht reflexiv, weil $(1,1)$ fehlt. Der häufige Fehlschluss nimmt an, zu jedem $x$ gebe es irgendein $y$ mit $(x,y) \in R$, und baut daraus $(x,x)$. Diese Existenz ist nicht gegeben."#,
        kind: ExerciseKind::TrueFalse { correct: false },
    },
];

pub static LESSON: Lesson = Lesson {
    id: "l09",
    order: 9,
    title: "Relationen",
    description: "Relationen als Teilmengen von A × A: reflexiv, symmetrisch, transitiv.",
    difficulty: Difficulty::Vertiefung,
    objectives: &[
        r#"Du liest eine Relation auf einer kleinen Menge als Menge geordneter Paare."#,
        r#"Du prüfst Reflexivität, Symmetrie und Transitivität vollständig, nicht nur an einem Paar."#,
        r#"Du verwendest die leere Relation als Gegenbeispiel zur Behauptung, Symmetrie und Transitivität ergäben Reflexivität."#,
    ],
    theory: THEORY,
    examples: &[
        Example { title: "Die Gleichheitsrelation auf zwei Elementen", blocks: EX_A },
        Example { title: "Eine kurze Kette ohne Abschluss", blocks: EX_B },
        Example { title: "Dieselbe Kette, transitiv geschlossen", blocks: EX_C },
    ],
    misconceptions: &[
        Misconception {
            title: "Reflexivität nur für vorkommende Elemente prüfen",
            text: r#"Liegt (1,2) in R und ist A = {1,2,3}, müssen trotzdem (1,1), (2,2) und (3,3) geprüft werden. 3 kann nicht ausgespart werden, nur weil es in keinem anderen Paar vorkommt."#,
        },
        Misconception {
            title: "Aus symmetrisch und transitiv automatisch reflexiv schliessen",
            text: r#"Der Standardscheinbeweis setzt voraus, dass jedes x zu mindestens einem y in Relation steht. Die leere Relation auf einer nicht leeren Menge erfüllt diese Voraussetzung nicht und ist das Gegenbeispiel."#,
        },
    ],
    summary: SUMMARY,
    exercises: EXERCISES,
};
