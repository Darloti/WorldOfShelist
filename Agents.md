# Arbeitsregeln fuer Agents

## Projektziel

Das Spiel World of Shelist ist ein datengetriebenes Fantasy-Simulationsspiel. Das technische Projekt und Cargo-Crate heissen `WorldOfShelist`. Zuerst entsteht eine vollstaendige, reproduzierbare und beobachtbare Welt-Simulation. Spielmodi wie Roguelike, Siedlungsbau, Koenigreich und History greifen spaeter auf dieselbe Welt und dieselben Simulationsregeln zu.

Die verbindliche Projektdefinition steht in `docs/PROJECT-VISION.md`. Die Roadmap besteht aus dem kurzen Index `docs/ROADMAP.md` und je einer Datei unter `docs/roadmap/`.

## Technische Leitplanken

- Sprache ist Rust, die erste grafische Ausgabe verwendet Macroquad.
- Der Simulationskern darf weder Macroquad noch andere Rendering- oder Eingabe-Abhaengigkeiten kennen.
- Zeitfortschritt wird ueber explizite Ticks oder Zeitabschnitte gesteuert und nicht implizit aus der Render-Framerate abgeleitet.
- Zufall wird ueber einen Seed und benannte RNG-Stroeme reproduzierbar gemacht.
- Zustandsaenderungen sollen als validierte Commands oder Events nachvollziehbar sein.
- Datenformate, Definitionen und Regeln werden von Laufzeitcode getrennt, soweit dies ohne unnoetige Abstraktion moeglich ist.
- Oeffentliche Crate-APIs bleiben klein. Interne Module kapseln fachliche Teilaufgaben; ein Orchestrator koordiniert sie.
- Performance wird gemessen. Keine vorzeitige ECS-, Multithreading- oder Streaming-Komplexitaet ohne Profiling-Bedarf.
- Persistenz und Replay sind Kernanforderungen, keine spaete Zusatzfunktion.

## Zielstruktur

Die aktuelle Anwendung ist ein einzelnes Starter-Crate. Bei wachsendem Code soll sie schrittweise in einen Cargo-Workspace uebergehen, voraussichtlich mit:

- `world_core`: IDs, Zeit, Geometrie, gemeinsame Domänentypen und Fehler
- `world_data`: Definitionen, Templates, Balancing-Daten und Laden/Validieren
- `worldgen`: deterministische Welterzeugung
- `simulation`: Systeme, Scheduler, Commands, Events und Simulationszustand
- `world_rendering`: Macroquad-Adapter fuer Weltansicht und Debug-Visualisierung
- `game_modes`: Regeln und Aktionen der einzelnen Spielmodi
- `app`: Zusammenschalten von Konfiguration, Input, Simulation, Rendering und Savegames

Diese Namen sind Vorschlaege, keine Verpflichtung. Abhaengigkeiten sollen von oben nach unten zeigen: UI/Adapter -> Orchestrierung -> Fachlogik -> Kern. Fachlogik darf nicht von konkreten Adaptern nach oben zeigen.

## Arbeitsablauf

0. Gespraeche ueber Regeln, Architektur, Datenvertraege oder andere fachliche
   Entscheidungen beziehen sich zunaechst ausschliesslich auf die
   Dokumentation dieser Entscheidung. Eine beschlossene Entscheidung darf in
   den betroffenen Dokumenten festgehalten werden. Code oder Content darf nur
   angelegt oder veraendert werden, wenn der Nutzer dies ausdruecklich und
   konkret beauftragt; aus einem Gespraech oder einer fachlichen Entscheidung
   darf keine Implementierung abgeleitet werden.
   Wenn der Nutzer darum bittet, durch die Phasenabschnitte gefuehrt zu
   werden, erfolgt die Begleitung in kleinen, einzeln erklaerten Schritten
   statt als umfangreiche Aufgabenliste. Jeder Schritt wird mit einer
   einfachen Begruendung versehen, warum er sinnvoll ist. Bei Rust-Code
   werden die relevanten Teile ausfuehrlich erklaert, insbesondere was sie
   tun und warum sie in Rust so umgesetzt sind. Die Erklaerungstiefe kann
   spaeter im Dialog gemeinsam verringert werden.
1. Kontext sparsam laden: Zuerst nur den Roadmap-Index lesen, falls die
   zustaendige Phase noch nicht bekannt ist. Danach ausschliesslich die
   benoetigte Phasendatei unter `docs/roadmap/` lesen. `PROJECT-VISION.md`,
   `SYSTEM-CONCEPT.md` und andere Phasendateien nur oeffnen, wenn der konkrete
   Task eine darin enthaltene Entscheidung oder Definition zwingend benoetigt.
   Bestehende Module und weitere Dateien ebenfalls nur lesen, wenn sie fuer
   die konkrete Aenderung erforderlich sind. Keine vollstaendigen Verzeichnis-,
   Dokumentations- oder Codebestandsaufnahmen ohne konkreten Bedarf. Von
   benoetigten grossen Dateien nur die relevanten Abschnitte lesen; keine
   vorsorglichen Gesamtdateien in den Session-Kontext laden.
2. ToDo- und Checklistenpunkte duerfen erst abgehakt werden, wenn sie wirklich
   vollstaendig abgeschlossen, fachlich geprueft und falls erforderlich
   dokumentiert sind. Teilfortschritte bleiben offen und werden als solche
   benannt.
   Wenn der Nutzer nach den ToDos einer Phase fragt, ist ausschliesslich die
   zugehoerige Phasendatei zu lesen und deren Inhalt woertlich aufzulisten;
   keine Aufteilung,
   Konkretisierung oder sonstige Eigeninterpretation ergaenzen.
3. Die kleinste fachlich vollstaendige Aenderung umsetzen.
4. Neue Regeln zuerst in reinen Rust-Tests pruefen; Macroquad nur fuer Integrations- oder Darstellungstests verwenden.
5. Bei jeder Simulationserweiterung Seed, Tick, erzeugte Events und relevante Zustandsaenderungen sichtbar machen.
6. Dokumentation aktualisieren, wenn Architektur, Datenvertrag oder Roadmap betroffen sind.
7. `cargo fmt --check`, `cargo check` und passende `cargo test` ausfuehren. Bei grafikbezogenen Aenderungen zusaetzlich manuell starten. Wenn die zu pruefenden Aenderungen vom Nutzer stammen, diese Befehle nicht automatisch ausfuehren, sondern erst nach ausdruecklicher Aufforderung.
8. Bei Auflistungen von Entscheidungen, Fragen oder ToDos nummerierte Punkte im
   Format `1.`, `2.`, `3.` verwenden, damit der Nutzer direkt auf einzelne
   Punkte Bezug nehmen kann.

## Qualitaetskriterien

- Ein gleicher Seed, eine gleiche Konfiguration und gleiche Eingaben erzeugen dasselbe Ergebnis.
- Kein System veraendert fremden Zustand unkontrolliert; Abhaengigkeiten sind im Orchestrator erkennbar.
- Events sind fachlich benannt, versionierbar und enthalten genug Kontext fuer Logs und Replay.
- Fehler werden behandelt und nicht durch `unwrap` an externen Datenstellen versteckt.
- Jede neue persistierte Struktur bekommt eine Migrations- oder Versionsstrategie.
- Tests pruefen Invarianten, Grenzfaelle und deterministische Wiederholung, nicht nur happy paths.

## Nicht-Ziele der ersten Phase

- Kein vollstaendiges Kampfsystem.
- Keine ausgearbeitete Benutzeroberflaeche.
- Keine riesige prozedurale Detailwelt, bevor ein kleiner Weltabschnitt stabil simuliert und gespeichert werden kann.
- Keine vier unabhaengigen Spiele. Alle Modi muessen denselben Weltzustand verwenden.
