# MathForge

Lokale Lernplattform für Mengenlehre auf Universitätsniveau. Das Frontend ist eine Next.js-App, die Bewertung und das Mengenlabor laufen in einer Rust-API. Fortschritt liegt in einer SQLite-Datei auf diesem Rechner. Es gibt keine Anmeldung, keine Cloud und keinen KI-Dienst.

Die Oberfläche ist Deutsch (Schweizer Schreibweise).

## Voraussetzungen

- Node.js 20 oder neuer (getestet mit Node 24) und npm
- Rust stable mit Cargo (getestet mit Rust 1.99)
- Ein C-Compiler für das gebündelte SQLite (`rusqlite` mit Feature `bundled`). Auf Windows genügt die Visual-Studio-Buildumgebung, die mit Rust mitinstalliert wird.

## Start unter Windows (PowerShell)

Zwei Terminals, jeweils im Repository-Root.

Backend:

```powershell
cd backend
cargo run
```

Frontend:

```powershell
cd frontend
npm install
npm run dev
```

- Oberfläche: http://localhost:3000
- API: http://127.0.0.1:3001

Die API bindet standardmässig nur an `127.0.0.1`.

## Start unter Unix

```bash
cd backend
cargo run
```

```bash
cd frontend
npm install
npm run dev
```

Dieselben Adressen wie oben.

## Konfiguration

Backend, alle Variablen optional:

| Variable | Standard | Bedeutung |
| --- | --- | --- |
| `MATHFORGE_HOST` | `127.0.0.1` | Nur `127.0.0.1`, `localhost` oder `::1` |
| `MATHFORGE_PORT` | `3001` | API-Port |
| `MATHFORGE_DB` | `backend/data/mathforge.db` | SQLite-Datei, Ordner wird angelegt |
| `MATHFORGE_CORS_ORIGINS` | `http://localhost:3000,http://127.0.0.1:3000` | Kommagetrennte erlaubte Frontend-Origins |

PowerShell:

```powershell
$env:MATHFORGE_PORT = "3001"
$env:MATHFORGE_CORS_ORIGINS = "http://localhost:3000,http://127.0.0.1:3000"
```

Unix:

```bash
export MATHFORGE_PORT=3001
export MATHFORGE_CORS_ORIGINS="http://localhost:3000,http://127.0.0.1:3000"
```

Frontend:

| Variable | Standard |
| --- | --- |
| `NEXT_PUBLIC_API_URL` | `http://127.0.0.1:3001` |

Die Variable wird beim `next build` eingebacken. Für die Entwicklung reicht der Standard. Vorlage: `frontend/.env.example`.

## Datenbank

Beim ersten Start legt die API das Schema selbst an (`CREATE TABLE IF NOT EXISTS`). Es gibt keine separate Migration. Seed-Inhalte (Lektionen, Aufgaben, Lösungen) liegen im Rust-Code unter `backend/src/content/` und sind nicht Teil der Datenbank.

Fortschritt zurücksetzen:

- In der Übersicht auf «Fortschritt zurücksetzen», oder
- `DELETE http://127.0.0.1:3001/api/v1/progress`, oder
- API beenden und `backend/data/mathforge.db` löschen.

## Tests und Build

Backend:

```powershell
cd backend
cargo test
```

Frontend:

```powershell
cd frontend
npm run typecheck
npm run lint
npm test
npm run build
```

Unter Unix dieselben Befehle in einer POSIX-Shell.

Produktionsstart des Frontends nach dem Build: `npm run start` im Ordner `frontend` (Port 3000).

## Bedrohungsmodell

Es gibt keine Authentisierung. Fortschritt, Antworten und Prüfungsversuche sind Daten eines einzelnen lokalen Benutzers. Die API ist für eine vertrauenswürdige lokale Nutzung gedacht und darf nicht offen ins Netz gestellt werden. `MATHFORGE_HOST` lehnt andere Adressen ab.

## Inhalt

Zehn Lektionen von Mengengleichheit bis zu Funktionen als Relationen, dazu ein Mengenlabor und eine Prüfung mit zehn automatisch bewerteten Aufgaben. Details: `docs/MATH_CONTENT.md`. Architektur: `docs/ARCHITECTURE.md`. Abschluss des One-Shot-Auftrags: `docs/ONE_SHOT_REPORT.md`.

## Troubleshooting

- **«Das Backend ist nicht erreichbar»:** Im zweiten Terminal muss `cargo run` laufen und `MathForge API hört auf http://127.0.0.1:3001` ausgeben. `GET http://127.0.0.1:3001/api/v1/health` liefert `{"status":"ok",...}`.
- **Port belegt:** Anderen `MATHFORGE_PORT` setzen und `NEXT_PUBLIC_API_URL` beim Frontend-Start gleichziehen. Next.js-Port: `npm run dev -- --port 3000`.
- **CORS-Fehler im Browser:** Die Adresszeile muss `http://localhost:3000` oder `http://127.0.0.1:3000` sein, oder der Origin muss in `MATHFORGE_CORS_ORIGINS` stehen. `localhost` und `127.0.0.1` sind verschiedene Origins.
- **Datenbank gesperrt:** Nur eine API-Instanz pro Datei starten.
- **`cargo` findet kein SQLite:** Das Crate `rusqlite` baut SQLite mit. Ein fehlender C-Linker ist die übliche Ursache, nicht eine fehlende Systembibliothek.
- **Leere Prüfung nach Neuladen vor dem Abschicken:** Zwischeneingaben der Prüfung werden erst beim Abschicken gespeichert. Übungsversuche sind sofort persistiert.
