use super::model::*;

const THEORY: &[Block] = &[
    Block::Heading("Grundmenge und Komplement"),
    Block::Paragraph(r#"Das Komplement ist nur relativ zu einer Grundmenge eindeutig. Ist $U$ die Grundmenge und $A \subseteq U$, so ist das Komplement von $A$ in $U$ die Menge aller Elemente von $U$, die nicht in $A$ liegen."#),
    Block::Math(r#"\complement_U(A) = U \setminus A = \{ x \in U \mid x \notin A \}."#),
    Block::Paragraph(r#"Wir schreiben dafür auch $A^{c}$, aber nur, nachdem $U$ genannt ist. Ohne Grundmenge wäre unklar, ob zum Beispiel das Komplement von $\{1\}$ die Zahl $2$ enthält. In diesem Kurs gilt bei jeder Komplementaufgabe: $A \subseteq U$, und $U$ ist eine endliche Menge ganzer Zahlen."#),
    Block::Paragraph(r#"Aus der Definition folgen $\complement_U(U) = \emptyset$, $\complement_U(\emptyset) = U$ und $\complement_U(\complement_U(A)) = A$, sofern $A \subseteq U$."#),
    Block::Heading("Die Gesetze von De Morgan"),
    Block::Paragraph(r#"Für $A \subseteq U$ und $B \subseteq U$ vertauschen Komplement und die beiden Verknüpfungen Vereinigung und Schnitt, wobei aus «oder» ein «und» wird und umgekehrt."#),
    Block::Math(r#"\complement_U(A \cup B) = \complement_U(A) \cap \complement_U(B)"#),
    Block::Math(r#"\complement_U(A \cap B) = \complement_U(A) \cup \complement_U(B)"#),
    Block::Paragraph(r#"Ein Element liegt genau dann nicht in der Vereinigung, wenn es in keiner der beiden Mengen liegt, also im Schnitt der Komplemente. Es liegt genau dann nicht im Schnitt, wenn es in mindestens einem Komplement liegt. Das ist der Inhalt der beiden Gleichungen. Ein einzelnes Zahlenbeispiel illustriert sie, ersetzt aber nicht diese Überlegung für beliebige $A$ und $B$."#),
];

const EX_A: &[Block] = &[
    Block::Paragraph(r#"Sei $U = \{1,2,3,4,5,6\}$ und $A = \{1,2,6\}$. Dann ist $\complement_U(A) = \{3,4,5\}$. Die Zahl $7$ gehört nicht zum Komplement, weil sie nicht zu $U$ gehört."#),
];

const EX_B: &[Block] = &[
    Block::Paragraph(r#"Sei $U = \{1,2,3,4\}$, $A = \{1,2\}$ und $B = \{2,3\}$. Dann ist $A \cup B = \{1,2,3\}$ und $\complement_U(A \cup B) = \{4\}$. Andererseits ist $\complement_U(A) = \{3,4\}$ und $\complement_U(B) = \{1,4\}$, der Schnitt dieser Komplemente ist $\{4\}$. Beide Seiten des ersten De-Morgan-Gesetzes stimmen überein."#),
];

const EX_C: &[Block] = &[
    Block::Paragraph(r#"Mit denselben Mengen ist $A \cap B = \{2\}$ und $\complement_U(A \cap B) = \{1,3,4\}$. Die Vereinigung der Komplemente ist $\{3,4\} \cup \{1,4\} = \{1,3,4\}$. Auch das zweite De-Morgan-Gesetz trifft hier zu. Würde man eine andere Grundmenge wählen, etwa $\{1,2,3,4,5\}$, änderten sich die Komplemente."#),
];

const SUMMARY: &[Block] = &[
    Block::Paragraph(r#"Komplemente setzen eine genannte Grundmenge $U$ voraus. Für $A \subseteq U$ ist $\complement_U(A) = U \setminus A$. De Morgan: Das Komplement einer Vereinigung ist der Schnitt der Komplemente, das Komplement eines Schnitts ist die Vereinigung der Komplemente."#),
];

const EXERCISES: &[Exercise] = &[
    Exercise {
        id: "l04e01",
        lesson_id: "l04",
        difficulty: Difficulty::Einstieg,
        prompt: r#"Sei $U = \{0,1,2,3,4\}$ und $A = \{0,2,4\}$. Bestimme $\complement_U(A)$."#,
        hint: r#"Welche Elemente von $U$ liegen nicht in $A$?"#,
        explanation: r#"$U$ enthält $0,1,2,3,4$. Davon liegen $0,2,4$ in $A$. Übrig bleiben $1$ und $3$. Also ist $\complement_U(A) = \{1,3\}$."#,
        kind: ExerciseKind::Set { correct: &[1, 3] },
    },
    Exercise {
        id: "l04e02",
        lesson_id: "l04",
        difficulty: Difficulty::Aufbau,
        prompt: r#"Sei $U = \{1,2,3,4,5\}$, $A = \{1,2\}$ und $B = \{2,3\}$. Bestimme $\complement_U(A \cup B)$."#,
        hint: r#"Bilde zuerst die Vereinigung, danach das Komplement relativ zu $U$."#,
        explanation: r#"$A \cup B = \{1,2,3\}$. In $U$ fehlen dann genau $4$ und $5$. Also ist $\complement_U(A \cup B) = \{4,5\}$. Nach De Morgan ist das zugleich $\complement_U(A) \cap \complement_U(B) = \{3,4,5\} \cap \{1,4,5\} = \{4,5\}$."#,
        kind: ExerciseKind::Set { correct: &[4, 5] },
    },
    Exercise {
        id: "l04e03",
        lesson_id: "l04",
        difficulty: Difficulty::Aufbau,
        prompt: r#"Wahr oder falsch? Das Komplement einer Menge ist auch ohne Angabe einer Grundmenge eindeutig bestimmt."#,
        hint: r#"Überlege, ob $2$ zum Komplement von $\{1\}$ gehören soll."#,
        explanation: r#"Falsch. Ohne Grundmenge ist nicht festgelegt, welcher Vorrat von Elementen ausserhalb von $A$ überhaupt betrachtet wird. $\complement_{\{1,2\}}(\{1\}) = \{2\}$, aber $\complement_{\{1,2,3\}}(\{1\}) = \{2,3\}$. Die Ergebnisse sind verschieden."#,
        kind: ExerciseKind::TrueFalse { correct: false },
    },
    Exercise {
        id: "l04e04",
        lesson_id: "l04",
        difficulty: Difficulty::Vertiefung,
        prompt: r#"Wahr oder falsch? Für alle Mengen $A,B \subseteq U$ gilt $\complement_U(A \cap B) = \complement_U(A) \cup \complement_U(B)$."#,
        hint: r#"Ein Element liegt genau dann nicht in beiden Mengen, wenn es in mindestens einem Komplement liegt."#,
        explanation: r#"Wahr. Das ist das zweite Gesetz von De Morgan. $x \notin A \cap B$ bedeutet, dass $x \notin A$ oder $x \notin B$ gilt, also $x \in \complement_U(A) \cup \complement_U(B)$, sofern $x \in U$."#,
        kind: ExerciseKind::TrueFalse { correct: true },
    },
    Exercise {
        id: "l04e05",
        lesson_id: "l04",
        difficulty: Difficulty::Vertiefung,
        prompt: r#"Sei $U = \{1,2,3,4\}$, $A = \{1,4\}$ und $B = \{4\}$. Bestimme $\complement_U(A \cap B)$."#,
        hint: r#"Der Schnitt ist klein. Bilde ihn zuerst."#,
        explanation: r#"$A \cap B = \{4\}$. Das Komplement von $\{4\}$ in $U$ ist $\{1,2,3\}$. Zur Kontrolle: $\complement_U(A) \cup \complement_U(B) = \{2,3\} \cup \{1,2,3\} = \{1,2,3\}$."#,
        kind: ExerciseKind::Set { correct: &[1, 2, 3] },
    },
];

pub static LESSON: Lesson = Lesson {
    id: "l04",
    order: 4,
    title: "Grundmenge, Komplement und De Morgan",
    description: "Komplemente nur relativ zu U und die beiden De-Morgan-Gesetze.",
    difficulty: Difficulty::Aufbau,
    objectives: &[
        r#"Du bildest $\complement_U(A)$ nur nach Angabe der Grundmenge und prüfst $A \subseteq U$."#,
        r#"Du formulierst beide De-Morgan-Gesetze und wendest sie auf endliche Grundmengen an."#,
        r#"Du begründest De Morgan mit einer Elementbedingung, nicht nur mit einem Beispiel."#,
    ],
    theory: THEORY,
    examples: &[
        Example { title: "Komplement in einer sechselementigen Grundmenge", blocks: EX_A },
        Example { title: "De Morgan für die Vereinigung", blocks: EX_B },
        Example { title: "De Morgan für den Schnitt", blocks: EX_C },
    ],
    misconceptions: &[
        Misconception {
            title: "Komplement ohne Grundmenge",
            text: r#"«Alles, was nicht in A liegt» ist keine Menge, mit der wir hier rechnen. Erst U legt fest, welche Kandidaten existieren. In jeder Aufgabe und im Labor, sobald von Komplementen die Rede ist, muss U dastehen."#,
        },
        Misconception {
            title: "De Morgan nur als Merkformel",
            text: r#"Die Formel vertauscht nicht nur Zeichen. Sie übersetzt «nicht (oder)» in «(nicht) und (nicht)» und «nicht (und)» in «(nicht) oder (nicht)». Wer das an einem Element x nachvollzieht, braucht die Formel nicht auswendig zu raten."#,
        },
    ],
    summary: SUMMARY,
    exercises: EXERCISES,
};
