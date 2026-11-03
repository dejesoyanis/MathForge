# Architektur

MathForge ist ein Monorepo mit zwei Prozessen. Der Browser spricht nur JSON mit der Rust-API. Mengenoperationen, der Parser und die Aufgabenbewertung existieren einmal, im Backend.

## Komponenten

- `frontend/`: Next.js App Router, TypeScript, Tailwind CSS, KaTeX. Seiten: Übersicht, Lektion, Mengenlabor, Prüfung.
- `backend/`: Axum, Tokio, Serde, rusqlite. Eine SQLite-Verbindung hinter einem Mutex, ausreichend für den lokalen Einzelbenutzer.
- `backend/src/content/`: versionierte Lektionen und die Prüfung. Lösungen stehen nur hier.
- `backend/src/domain/`: `IntSet`, Parser, Bewertung.
- `backend/src/db.rs`: Schema und Persistenz.
- `backend/data/mathforge.db`: Laufzeitdaten, nicht versioniert.

## Datenfluss

1. Die Übersicht lädt `GET /api/v1/lessons` und `GET /api/v1/progress`.
2. Eine Lektion lädt `GET /api/v1/lessons/:id`. Der Payload enthält Theorie, Beispiele, Hinweise und Aufgabenstellungen, aber keine Lösungen und keine richtigen Indizes.
3. `POST /api/v1/exercises/:id/submit` bewertet in `domain::grade`. Erst die Antwort enthält Musterlösung und Feedback. Ungültige Mengeneingaben liefern HTTP 400 und werden nicht als Versuch gespeichert.
4. Das Labor sendet die Rohstrings an `POST /api/v1/lab/evaluate` und zeigt die zurückgegebenen LaTeX-Formeln an. Es rechnet nicht selbst.
5. Eine Prüfung entsteht mit `POST /api/v1/exam/start`. Solange sie offen ist, enthält `GET /api/v1/exam/:id` keine Auswertung. `POST /api/v1/exam/:id/submit` schreibt Score, Antworten und Musterlösungen in einer SQLite-Anweisung.

## API `/api/v1`

Fehler haben immer die Form `{ "error": { "code", "message", "details?" } }`.

| Methode | Pfad | Zweck |
| --- | --- | --- |
| GET | `/health` | Status, Dienstname, Version |
| GET | `/lessons` | Liste ohne Lösungen |
| GET | `/lessons/:id` | Lektion ohne Lösungen |
| POST | `/lessons/:id/complete` | Abschluss, wenn jede Übung einen Versuch hat |
| GET | `/progress` | Zähler, nächste Lektion, letzte Versuche inklusive freigegebener Musterlösung |
| DELETE | `/progress` | Lokalen Fortschritt löschen |
| POST | `/exercises/:id/submit` | Versuch bewerten und speichern |
| POST | `/lab/evaluate` | Mengenoperationen für A und B |
| POST | `/exam/start` | Neuen Versuch anlegen oder den offenen zurückgeben |
| GET | `/exam/attempts` | Liste |
| GET | `/exam/:id` | Fragen, nach Abschluss auch Auswertung |
| POST | `/exam/:id/submit` | Alle zehn Antworten bewerten |

Antwortkörper einer Übung:

```json
{ "answer": { "type": "set", "value": "{1,2,3}" } }
```

Weitere `type`-Werte: `true_false` (`value` boolesch), `single_choice` (`index` ab 0), `numeric` (JSON-Ganzzahl), `free_text` (`value`, optional `self_assessment`: `correct`, `partial`, `incorrect`).

## Bewertung

Automatisch bewertete Aufgaben setzen `correct` auf `true` oder `false`. Freitext setzt `auto_graded` auf `false` und `correct` immer auf `null`, auch wenn eine Selbsteinschätzung mitgeschickt wird. Die Selbsteinschätzung liegt in einem eigenen Feld und zählt in `self_assessed_correct`, nicht in `exercises_correct`.

«Richtige Antworten» ist die Anzahl der Übungen, deren **letzter** automatisch bewerteter Versuch korrekt ist. Es werden keine Prozentwerte berechnet. Eine Lektion gilt als abgeschlossen, wenn jede ihrer Übungen mindestens einmal beantwortet wurde und die Nutzerin den Abschluss auslöst. Nicht jede Antwort muss richtig sein.

Der Prüfungsscore ist die Anzahl korrekter automatisch bewerteter Aufgaben geteilt durch deren Anzahl, ausgedrückt als «n von m». Diese Prüfung enthält zehn solche Aufgaben und keinen Freitext. Der Nenner ist deshalb 10. Der Hinweistext kommt vom Backend (`score_note`).

## Parser

Grammatik und Grenzen stehen in `docs/MATH_CONTENT.md`. Der Vergleich zweier Mengen läuft über `BTreeSet<i64>`, nicht über Rohstrings. Duplikate fallen weg, die Ausgabe ist aufsteigend sortiert. Die leere Menge wird kanonisch als `∅` zurückgegeben.

## Persistenz

Tabellen: `exercise_attempts`, `lesson_progress`, `exam_attempts`. Zeitstempel sind UTC in RFC 3339. Prüfungsabgaben schreiben Score und JSON in einem `UPDATE`. Ein Neustart der API öffnet dieselbe Datei wieder.

Übungsversuche zählen nicht als Prüfungsaufgaben und umgekehrt.

## Sicherheit

- Bind nur an Loopback.
- CORS nur für konfigurierte Origins.
- Request-Body maximal 32 KiB.
- SQL ausschliesslich mit gebundenen Parametern.
- Kein `unwrap` auf Nutzereingaben. Parser- und JSON-Fehler werden zu HTTP 400.
- KaTeX läuft mit `trust: false`. Formeln rendert nur der Lektionsinhalt. Freitext der Nutzerin bleibt Text.
- Lösungen sind nicht im JavaScript-Bundle und nicht im Aufgaben-GET.

## Entscheidungen

- Inhalte als Rust-Statics statt JSON, damit Typen und Tests die Lösungen beim Kompilieren sehen und der Browser sie nicht importieren kann.
- Eine SQLite-Verbindung statt eines Pools. Der Mutex serialisiert Zugriffe.
- Schema per `CREATE TABLE IF NOT EXISTS` statt eines Migrationsframeworks. Es gibt genau eine lokale Datei und keine Altbestände.
- Kein Docker. Zwei Befehle genügen.
- Prüfung ohne Freitext, damit der Score vollständig automatisch und nachvollziehbar ist. Beweisaufgaben bleiben in den Lektionen und verlangen eine Selbsteinschätzung.
- Zwischeneingaben einer offenen Prüfung werden nicht pro Tastendruck gespeichert. Der abgeschlossene Versuch schon.
