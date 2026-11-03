# Mathematischer Inhalt

Inhaltliche Quelle: `backend/src/content/`. Version `1` (`CONTENT_VERSION`). Die Aufgabenlösungen stehen nur dort. Dieser Text beschreibt Konventionen und Grenzen, nicht die Musterlösungen im Einzelnen.

## Lektionen

1. **Mengen, Elemente und Gleichheit.** Aufzählung, \(\in\), Extensionalität. Reihenfolge und Duplikate ändern die Menge nicht.
2. **Teilmengen und die leere Menge.** \(\subseteq\), \(\subsetneq\), \(\emptyset\) gegen \(\{\emptyset\}\), \(\in\) gegen \(\subseteq\).
3. **Vereinigung, Schnitt und Differenz.** \(\cup\), \(\cap\), \(\setminus\), symmetrische Differenz \(\triangle\).
4. **Grundmenge, Komplement und De Morgan.** Komplement nur relativ zu einem genannten \(U\). Beide De-Morgan-Gesetze.
5. **Potenzmengen.** \(\mathcal{P}(A)\) und \(|\mathcal{P}(A)| = 2^{|A|}\) für endliches \(A\), einschliesslich \(\mathcal{P}(\emptyset) = \{\emptyset\}\).
6. **Kartesische Produkte.** Geordnete Paare, \(A \times B\), \(|A \times B| = |A|\cdot|B|\).
7. **Mengengesetze und Elementbeweise.** Kommutativität, Assoziativität, Distributivität, Absorption. Ein Beispiel illustriert oder widerlegt, es beweist keine Allaussage. Eine Freitextaufgabe verlangt den Elementbeweis von \(A \cap B \subseteq A\).
8. **Kardinalität und Inklusion–Exklusion.** \(|A \cup B| = |A| + |B| - |A \cap B|\) für zwei endliche Mengen.
9. **Relationen.** \(R \subseteq A \times A\), reflexiv auf \(A\), symmetrisch, transitiv. Die leere Relation auf einer nicht leeren Menge ist symmetrisch und transitiv, aber nicht reflexiv.
10. **Funktionen als Relationen.** Definitionsmenge, Zielmenge, Bild. Jede Funktion ist eine Relation. Die Umkehrung gilt nicht. Eine Freitextaufgabe verlangt diese Begründung mit Gegenbeispiel.

Jede Lektion hat Lernziele, Theorie mit formalen Definitionen, mindestens zwei ausgerechnete Beispiele, typische Denkfehler, mindestens vier Übungen und eine Zusammenfassung. Die Prüfung stellt je eine automatisch bewertete Aufgabe aus jeder Lektion, mit anderen Zahlen als die Übungen.

Übungsaufgaben: 51. Davon 24 Wahr/Falsch, 7 Einzelauswahl, 9 Mengeneingaben, 9 Zahlen, 2 Freitextbegründungen. Prüfungsaufgaben: 10, alle automatisch bewertet (Wahr/Falsch, Menge oder Zahl).

## Notation

- Teilmenge inklusiv: \(\subseteq\). Echte Teilmenge: \(\subsetneq\). Das Zeichen \(\subset\) wird nicht verwendet, weil es in der Literatur beides bedeuten kann.
- Element: \(\in\). Ganze Zahlen behandeln wir als Elemente, nicht als Mengen. Aussagen wie \(2 \subseteq \{1,2\}\) werden nicht gebildet.
- Leere Menge: \(\emptyset\). Die Menge, deren einziges Element die leere Menge ist: \(\{\emptyset\}\). Diese beiden sind verschieden.
- Differenz: \(\setminus\). Symmetrische Differenz: \(\triangle\).
- Komplement: \(\complement_U(A)\) oder \(A^{c}\), immer mit genannter Grundmenge \(U\) und \(A \subseteq U\).
- Potenzmenge: \(\mathcal{P}(A)\). Kardinalität: \(|A|\).
- Geordnetes Paar: \((a,b)\). Gleichheit komponentenweise. Das Paar ist nicht die Menge \(\{a,b\}\).
- Relation auf \(A\): Teilmenge von \(A \times A\).
- Funktion \(f: A \to B\): Relation, die jedem \(a \in A\) genau ein \(b \in B\) zuordnet. Das Bild \(f(A)\) ist eine Teilmenge der Zielmenge.

Beispiele und Aufgaben verwenden endliche, explizit angegebene Mengen. Komprehensionen ohne Grundbereich kommen nicht vor. Die Kuratowski-Kodierung von Paaren wird nur als Ausblick erwähnt und nicht als Antwortformat verlangt.

## Automatische Bewertung

Der Parser akzeptiert ausschliesslich endliche Mengen ganzer Zahlen:

```text
menge    ::= "∅" | "{" [ eintrag ("," eintrag)* ] "}"
eintrag  ::= ganzzahl
ganzzahl ::= "-" ? ziffer+
```

Leerzeichen um Klammern, Kommas und Zahlen sind erlaubt. `{}` und `∅` sind die leere Menge. Führende Nullen sind erlaubt und werden als Zahlwert gelesen. Duplikate werden entfernt. Die kanonische Ausgabe ist aufsteigend sortiert, die leere Menge als `∅`.

Grenzen: höchstens 512 Zeichen, höchstens 64 aufgezählte Einträge, jede Zahl im Bereich von `i64`. Abgelehnt werden unter anderem leere Eingaben, nachgestellte Kommas, Dezimalzahlen, ein führendes `+`, Buchstaben, verschachtelte Klammern und Text nach der schliessenden Klammer.

Verschachtelte Mengen, Paare, Relationen und Potenzmengen werden nicht von diesem Parser gelesen. Aufgaben dazu sind Wahr/Falsch, Einzelauswahl oder eine Anzahl. Die Oberfläche behauptet nicht, der Parser könne beliebige verschachtelte Mengen.

Wahr/Falsch, Einzelauswahl, Integer-Mengen und ganze Zahlen bewertet das Backend. Freitext bekommt eine Musterlösung und eine Selbsteinschätzung (korrekt, teilweise, nicht korrekt). `correct` bleibt dabei leer. Eine Selbsteinschätzung ist kein formal geprüfter Beweis.

Der Prüfungsscore lässt Freitext weg. In der ausgelieferten Prüfung gibt es keinen Freitext, der Nenner ist die Zahl der Prüfungsaufgaben. «Richtige Antworten» auf der Übersicht zählt nur den letzten korrekten automatischen Versuch einer Übung und ignoriert Selbsteinschätzungen.

## Was dieser Kurs nicht leistet

Keine unendlichen Mengen, keine allgemeine Inklusion–Exklusion für mehr als zwei Mengen, keine Äquivalenzrelationen als eigener Satz, keine Injektivität oder Surjektivität über den Hinweis hinaus, dass das Bild in der Zielmenge liegt. Elementbeweise werden angezeigt und selbst eingeschätzt, nicht maschinell geprüft.
