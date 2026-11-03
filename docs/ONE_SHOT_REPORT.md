# ONE_SHOT_REPORT — MATHFORGE-001

Stand: 3. November 2026. Dieser Bericht trennt ausgeführte Checks von Annahmen. Der Commit-Hash entsteht erst mit dem Commit, der diese Datei enthält, und steht deshalb in der Abschlussantwort an den Auftraggeber, nicht hier.

## Was implementiert ist

- Monorepo mit `frontend/`, `backend/`, `docs/`, Root-README und `.gitignore`.
- Next.js (App Router, TypeScript, Tailwind) und KaTeX mit `trust: false` und HTML+MathML.
- Rust-API (Axum, Tokio, Serde) auf `127.0.0.1:3001`, SQLite unter `backend/data/mathforge.db`.
- Zehn ausgearbeitete Lektionen auf Deutsch (Schweizer Schreibweise) mit Lernzielen, Theorie, formalen Definitionen, mindestens zwei Beispielen, Denkfehlern, Übungen und Zusammenfassung.
- 51 Übungen und 10 Prüfungsaufgaben. Lösungen fehlen im GET der Aufgaben und liegen nur im Backend.
- Bewertung in Rust: Wahr/Falsch, Einzelauswahl, Integer-Mengen, Zahlen. Freitext hat `correct: null` und eine Selbsteinschätzung.
- Mengenlabor über `POST /api/v1/lab/evaluate`.
- Fortschritt, Lektionsabschluss und Prüfungsversuche in SQLite. Reset über die Übersicht oder `DELETE /api/v1/progress`.

## Was nicht implementiert ist

- Anmeldung, Cloud, KI, Bezahlung, Telemetrie, Docker.
- Venn-Diagramme und Dark Mode.
- Parser für verschachtelte Mengen, Paare oder Relationen. Solche Aufgaben sind Auswahl, Wahr/Falsch oder Anzahl.
- Maschinelle Prüfung von Beweisen.
- Speichern von Prüfungs-Zwischeneingaben vor dem Abschicken.
- Mehr als ein lokaler Benutzer.

## Projektbaum

```text
README.md
.gitignore
backend/Cargo.toml
backend/Cargo.lock
backend/src/main.rs
backend/src/lib.rs
backend/src/error.rs
backend/src/db.rs
backend/src/api/
backend/src/domain/
backend/src/content/        Lektionen l01–l10 und Prüfung
backend/tests/api.rs
frontend/package.json
frontend/package-lock.json
frontend/app/               Übersicht, Lektion, Labor, Prüfung
frontend/components/
frontend/lib/
frontend/tests/
docs/ARCHITECTURE.md
docs/MATH_CONTENT.md
docs/ONE_SHOT_REPORT.md
```

`node_modules/`, `frontend/.next/`, `backend/target/` und `*.db` sind ignoriert.

## Entscheidungen

- Eine Quelle für Aufgaben und Lösungen: Rust-Statics. Das Frontend dupliziert keine Mengenoperation.
- \(\subseteq\) und \(\subsetneq\). \(\subset\) wird nicht verwendet.
- Score ohne Prozent. «Richtige Antworten» zählt nur den letzten korrekten automatischen Versuch. Selbsteinschätzung ist ein eigenes Feld.
- Die Prüfung hat zehn automatische Aufgaben, eine je Lektion, und keinen Freitext. Der Nenner ist 10. Der Hinweistext kommt von der API.
- Eine SQLite-Verbindung hinter einem Mutex. Schema beim Start mit `CREATE TABLE IF NOT EXISTS`.
- Abschluss einer Lektion verlangt einen Versuch pro Übung, nicht durchgehend richtige Antworten.
- Host ausserhalb von Loopback wird abgelehnt.

## Aufgaben

| Art | Übungen | Prüfung |
| --- | ---: | ---: |
| Wahr/Falsch | 24 | 5 |
| Einzelauswahl | 7 | 0 |
| Integer-Menge | 9 | 2 |
| Zahl | 9 | 3 |
| Freitext | 2 | 0 |
| Summe | 51 | 10 |

Lektionen 1–9 haben je 5 Übungen, Lektion 10 hat 6.

## Ausgeführte Checks

Alle Befehle liefen in diesem Auftrag. Ergebnis ist der Exit-Code und die Testzählung, nicht eine Schätzung.

| Befehl | Ergebnis |
| --- | --- |
| `cargo test` in `backend/` | Pass. 14 Unit-Tests, 5 API-Tests, 0 Doc-Tests. 0 fehlgeschlagen. |
| `npm run typecheck` in `frontend/` | Pass, Exit 0. |
| `npm run lint` in `frontend/` | Pass, Exit 0. |
| `npm test` in `frontend/` | Pass. 2 Dateien, 3 Tests. |
| `npm run build` in `frontend/` | Pass. Next.js 15.5.28, statische und dynamische Routen erzeugt. |
| `cargo run` und `npm run dev` | Pass. API meldet `MathForge API hört auf http://127.0.0.1:3001`. Next.js meldet `Ready` auf http://localhost:3000. |

`cargo run` gibt `dead_code`-Warnungen für Hilfsfunktionen aus, die nur Tests oder die gespeicherte Rohantwort nutzen. Der Build bricht daran nicht ab.

Browser-Smoke, http://localhost:3000, gegen die laufende API:

1. Übersicht zeigt 10 Lektionen, Lernstand 0 / 0 / 0 von 10, ohne Prozent.
2. «Weiterlernen» öffnet `/lessons/l01`.
3. Tab «Übungen»: erste Aussage als wahr abgeschickt. Antwort «Richtig», Musterlösung sichtbar, Zähler «1 von 5 Übungen bearbeitet». Vor dem Abschicken war keine Musterlösung im Baum.
4. Zurück auf der Übersicht: 1 bearbeitete Übung, 1 richtige Antwort, Lektion 1 «in Arbeit». `GET /api/v1/progress` lieferte dieselben Zahlen und `correct: true` für `l01e01`.
5. Mengenlabor mit `{-1,0,2}` und `{0,3}`: Mächtigkeiten 3, 2, Vereinigung 4, Schnitt 1, A ohne B 2, B ohne A 1, symmetrische Differenz 3. Teilmengenflags verneint, Mengen ungleich.
6. Danach `DELETE /api/v1/progress`. Erneutes GET: alle drei Zähler 0. Der ausgelieferte Datenbestand des Smoke-Tests ist damit geleert.

Im eingebetteten Browser hat ein Mausklick Links fokussiert; die Navigation lief über Enter auf dem fokussierten Link. Buttons (Tab, «Wahr», «Antwort prüfen», «Berechnen») reagierten auf den Klick. Das ist eine Beobachtung am Automationswerkzeug, kein nachgewiesener Fehler der Seite: die Links sind echte Anker und per Tastatur benutzbar.

## Bekannte Grenzen

- Kein Schutz der Fortschrittsdaten vor anderen lokalen Prozessen. Das ist das dokumentierte Einzelbenutzer-Modell.
- Offene Prüfung: Antworten stehen erst nach dem Abschicken in SQLite. Ein Reload davor verwirft die Eingaben, der Versuchsdatensatz bleibt offen.
- Zahlen im Frontend sind JSON-Zahlen. Die Aufgaben bleiben klein und exakt. Der Parser im Backend prüft `i64`, die Oberfläche ist nicht für den vollen 64-Bit-Rand gedacht.
- `cargo run` warnt vor ungenutzten Hilfsfunktionen. Tests und Laufzeit sind davon nicht betroffen.
- Die PowerShell-Konsole zeigt Umlaute in `Invoke-RestMethod` falsch an. Browser und API-UTF-8 sind in Ordnung; im Smoke-Test war der deutsche Text lesbar.

## Nächste Iteration, nicht umgesetzt

- Entwurfsspeicherung in der Prüfung.
- Dunkelmodus und eine einfache, korrekte Darstellung der Teilmengenrelation.
- Clippy und ein zweiter Browserlauf auf einer schmalen Viewport-Breite.
- Getrennte anonyme Profile, falls die Datei einmal mehr als eine Person nutzen soll. Das widerspricht dem jetzigen Modell und bräuchte eine bewusste Entscheidung.
