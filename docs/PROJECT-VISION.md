# Projektdefinition: World of Shelist

## Projektidentitaet

- Technischer Projekt- und Cargo-Name: `WorldOfShelist`
- Sichtbarer Spieletitel: `World of Shelist`

## Technische Mindestanforderungen

- Rust-Edition: `2024`
- Minimale Rust-Version (MSRV): `1.85`
- Verbindliche Erstplattform: `Windows x86_64` mit der `MSVC`-Toolchain
- Fuer den Build werden Rust/Cargo sowie ein kompatibler C/C++-Linker der MSVC-Toolchain benoetigt.
- Weitere Plattformen werden erst verbindlich zugesagt, wenn sie eingerichtet und getestet sind.

## Verbindliche Qualitaetsbefehle

- Formatierung pruefen: `cargo fmt --check`
- Build pruefen: `cargo check`
- Linting pruefen: `cargo clippy --all-targets --all-features -- -D warnings`
- Tests ausfuehren: `cargo test --all-targets --all-features`

Clippy-Warnungen gelten mit `-D warnings` als Fehler und muessen behoben oder
gezielt im Code begruendet unterdrueckt werden.

## Sprache und Internationalisierung

- Quellcode, oeffentliche Projektdokumentation und sichtbare technische
  Schnittstellen werden auf Englisch erstellt.
- Die Dokumente unter `docs/` sind interne Arbeitsdokumente und bleiben auf
  Deutsch.
- Die initiale Spielsprache ist Englisch mit dem Sprachcode `en`.
- Sichtbare Texte werden ueber stabile Uebersetzungsschluessel bezogen und
  nicht direkt in Simulations- oder UI-Logik eingebettet.
- `en` ist die initiale Fallback-Sprache. Weitere Sprachen koennen spaeter
  ergaenzt werden, ohne die Fachlogik zu aendern.
- Die i18n-Basis soll auf dem Fluent-Oekosystem aufbauen, voraussichtlich mit
  `fluent-bundle` und `unic-langid`. Die konkrete Einbettung von Sprachdateien
  wird beim Aufbau der Anwendung festgelegt.

## Datengetriebene Engine und Mods

World of Shelist wird als datengetriebene und hochgradig modbare Simulation
entwickelt. Das Grundspiel ist kein Sonderfall der Engine, sondern wird als
`core-mod` ueber dieselbe Mod-Schnittstelle geladen wie spaetere Erweiterungen.

- Die Engine stellt generische Strukturen und Mechanismen fuer Weltzustand,
  Definitionen, Validierung, Zeit, deterministischen Zufall, Commands, Events,
  Persistenz und Ausfuehrung bereit.
- Das `core-mod` liefert die initialen Definitionen, Inhalte und Regeln des
  Grundspiels.
- Regeln von der Weltgenerierung ueber Ressourcen und Beduerfnisse bis zu
  Commands und Kampfberechnungen sollen nach Moeglichkeit als versionierte,
  validierte Daten beschrieben werden.
- Systeme interpretieren diese Daten und sollen keine grundspiel-spezifischen
  Sonderpfade benoetigen.
- Datenformate, Mod-Abhaengigkeiten, Defaults und Konfliktregeln werden als
  Teil des Mod-Vertrags festgelegt.
- Unvermeidbarer nativer Code bleibt auf generische Laufzeitmechanismen,
  Dateninterpretation, Sicherheitsgrenzen und gemessene Performancepfade
  beschraenkt.

Das Grundspiel und Mods muessen denselben Determinismus-, Validierungs-,
Versions- und Persistenzregeln folgen. Ein Savegame muss daher auch die
verwendeten Mod-Versionen und deren Datenvertrag nachvollziehbar festhalten.

## Namespaced-IDs und Mod-Aufloesung

Jedes fachlich adressierbare Ding erhaelt eine stabile, genau dreiteilige
`namespaced-id`. Diese Identitaet bleibt in Mod-Dateien, Logs, Fehlern, Events,
Debug-Ausgaben und Werkzeugen lesbar. Das verbindliche Format ist:

```text
<namespace>:<kind>:<name>
```

Die drei Segmente haben immer diese Bedeutung:

1. `namespace`: Mod-Namespace, dem das Ding gehoert.
2. `kind`: Kategorie oder fachliche Art des Dings.
3. `name`: konkrete, innerhalb der Kategorie benannte Identitaet.

Jedes Segment ist erforderlich und nicht leer. IDs mit weniger oder mehr als
drei Segmenten sind ungueltig.

Beispiele:

```text
core:item:wood
core:worldgen:overworld
```

Mods koennen Inhalte anderer Mods gezielt erweitern, teilweise ueberschreiben,
deaktivieren oder neue Inhalte hinzufuegen. Ueberschreibungen werden nicht
implizit durch Dateireihenfolge entschieden, sondern ueber einen expliziten,
deterministischen Mod-Aufloesungsvertrag mit Namespace-, Versions- und
Abhaengigkeitspruefung. Teilweise Aenderungen muessen auf validierten
Feldpfaden oder definierten Merge-Regeln beruhen; eine Deaktivierung wird als
ausdruecklicher Zustand behandelt.

Zur Laufzeit duerfen namespaced-IDs auf kompakte numerische IDs abgebildet
werden. Diese numerischen IDs sind lediglich interne Referenzen fuer Code,
Speicher und Performance. Das Woerterbuch zwischen beiden Formen wird
versioniert im Welt- beziehungsweise Save-Kontext gefuehrt und muss jederzeit
beide Richtungen unterstuetzen:

```text
namespaced-id <-> runtime-id
```

Fehlt eine Rueckuebersetzung, ist ein Save, Event, Fehler oder Debug-Eintrag
nicht ausreichend erklaerbar und die Daten sind abzulehnen oder als
unaufgeloest zu kennzeichnen. Die textuelle Identitaet darf daher nicht durch
eine numerische ID ersetzt werden.

## Crate- und Modulkonventionen

Fachlich eigenstaendige Bereiche werden als kleine Library-Crates organisiert.
Ein Crate soll in sich stimmig, unabhaengig testbar und bei Bedarf auch in
anderen Spielen wiederverwendbar sein. Module dienen innerhalb eines Crates
der weiteren Gliederung und werden nicht ohne fachlichen Grund als eigene
Crates ausgelagert.

Die folgende Struktur ist eine vorlaeufige Startstruktur und keine vollstaendige
oder abschliessende Liste. Weitere fachliche Crates koennen hinzukommen;
bestehende Crates koennen bei wachsendem Verstaendnis geteilt oder
zusammengelegt werden. Die grundlegende Abhaengigkeitsrichtung ist:

```text
world_core <- world_data <- worldgen
world_core <- world_data <- simulation
worldgen + simulation <- app
```

- `world_core` enthaelt gemeinsame, adapterfreie Grundbegriffe wie IDs, Zeit,
  Geometrie und Fehler.
- `world_data` enthaelt Definitionen, Konfigurationen und deren Validierung.
- `worldgen` beschaeftigt sich ausschliesslich mit deterministischer
  Welterzeugung.
- `simulation` beschaeftigt sich ausschliesslich mit dem Fortschreiben des
  Weltzustands.
- `app` verbindet Konfiguration, Eingabe, Simulation und spaetere Adapter.
- Rendering und Eingabe bleiben Adapter und werden nicht von `world_core`,
  `world_data`, `worldgen` oder `simulation` abhaengig gemacht.

Im aktuellen Starter-Crate bleiben die Quellen unter `src/`. Der Einstieg in
die Anwendung liegt in `src/main.rs`; fachliche Bibliothekslogik wird beim
Aufteilen in Crates nicht in `main.rs` eingebettet. Innerhalb eines Crates
gelten Rust-Konventionen fuer `snake_case` bei Modulen, Funktionen und
Variablen sowie `PascalCase` bei Typen und Traits. Oeffentliche APIs werden
bewusst klein gehalten.

## Kurzfassung

World of Shelist ist eine datengetriebene Fantasy-Welt, in der Geografie, Ressourcen, Kreaturen, Bewohner, Siedlungen, Zivilisationen und Artefakte durch Regeln und Ereignisse entstehen und sich ueber Zeit veraendern. Der Spieler betritt keine statische Levelsammlung, sondern eine bereits lebendige Welt. Seine Handlungen sind Eingaben in dieselbe Simulation, die auch autonome Akteure, Wirtschaft, Politik, Konflikte und Umwelt fortschreiben.

## Produktkern

Der wichtigste Wert ist nicht die Menge prozedural erzeugter Inhalte, sondern die Glaubwuerdigkeit ihrer Zusammenhaenge. Ein Artefakt soll eine Herkunft haben, eine Siedlung soll aus konkreten Beduerfnissen entstanden sein, und ein Krieg soll spaetere Grenzen, Familiengeschichten und Ressourcenlagen beeinflussen. Diese Kausalitaet wird durch Ereignisse, Beziehungen und Zeit sichtbar und speicherbar.

## Spielerlebnis

Die Welt wird zuerst erstellt, validiert, gespeichert und fuer eine Weile simuliert. Erst danach wird ein Spielmodus aktiviert. Der gleiche Weltzustand kann als:

- Roguelike: einzelne Figur, Erkundung, Kampf, Beute, Handwerk und persoenliche Beziehungen
- Siedlungsbau: Bauplanung, Bevoelkerung, Aufgaben, Versorgung und Produktionsketten
- Koenigreich: Herrschaft, Gesetze, Diplomatie, Krieg, Handel und langfristige Entwicklung
- History-Modus: die Weltgeschichte, Chroniken, Biografien, Kartenveraenderungen und Kausalzusammenhaenge betrachten

gespielt werden. Diese Modi sind Perspektiven und Regelpakete auf einer gemeinsamen Simulation. Sie sollen einzeln, nacheinander oder kombiniert aktiviert werden koennen.

## Systemgrenzen

Die Welt-Simulation ist der Kern. Rendering, Eingabe, Audio, UI und konkrete Spielmodi sind Adapter oder Konsumenten. Worldgen erzeugt einen initialen Zustand; Simulation veraendert ihn; Events erklaeren die Veraenderungen; Persistenz speichert Zustand, Konfiguration und Historie. Kein Spielmodus darf eine eigene Parallelwelt oder eigene Zeitlogik einfuehren.

## Architekturthese

Ein stabiler Kern aus IDs, Definitionen, Zustandsdaten, Commands, Events und Zeit ist wichtiger als eine rein technische Crate-Anzahl. Fachlich klar erkennbare Grenzen werden jedoch frueh als eigene Crates umgesetzt, damit die Trennung nicht auf eine spaetere, unbestimmte Phase verschoben wird. Ein Cargo-Workspace wird daher eingefuehrt, sobald die erste solche Crate-Grenze umgesetzt wird. Weitere Crates folgen, wenn sie fachliche Eigenstaendigkeit, Testbarkeit oder Wiederverwendbarkeit verbessern. Innerhalb jedes Crates arbeiten spezialisierte Module, deren Zusammenspiel ein Orchestrator mit klarer Reihenfolge und Fehlerbehandlung steuert.

## Architekturentscheidungen fuer Phase 1

Phase 1 beginnt als Cargo-Workspace mit den fachlichen Crates `world_core`,
`world_data`, `worldgen`, `simulation` und `app`. Die Crates werden nur dort
getrennt, wo eine fachliche Grenze, eigenstaendige Testbarkeit oder
Wiederverwendbarkeit besteht.

`world_core` bleibt adapterfrei und enthaelt die generischen Grundbegriffe des
Weltmodells, insbesondere IDs, Zeit, Geometrie und Fehler. Die Verwaltung von
Mods und Definitionen gehoert nicht in `world_core`.

`world_data` laedt und validiert das `core-mod` sowie weitere Mods. Es prueft
Abhaengigkeiten, Versionen und Datenvertraege, loest Mods deterministisch auf
und bildet lesbare namespaced IDs auf kompakte Runtime-IDs ab. Ein eigener
Mod-Crate ist fuer Phase 1 nicht erforderlich und bleibt eine spaetere Option,
falls die Mod-Verwaltung eine eigenstaendige fachliche Grenze bildet.

Die Daten fuer Phase 1 werden von Beginn an datengetrieben entwickelt. Die
erste Iteration erzeugt zunaechst die Geografie der Welt. Weitere Definitionen
und Laufzeitdaten werden anschliessend schrittweise ergaenzt.

Der erste Simulations-Tick umfasst genau einen Tag. Die erste Persistenzstufe
ist ein versionierter JSON-Snapshot.

Das erste historische Ereignis ist `InitialRaceSpawn`. Der Eventtyp tritt
mehrfach auf, mindestens einmal pro Rasse. Jede Rasse wird dabei in Gruppen von
jeweils 100 Individuen an ihren initialen Weltpositionen angesiedelt. Die
Ereignisse bilden den nachvollziehbaren Ausgangspunkt fuer spaetere
Siedlungs- und Zivilisationsgruendungen.

ECS, Multithreading, Streaming und vergleichbare technische Komplexitaet werden
fuer Phase 1 nicht vorab entschieden. Sie werden erst bei einem nachgewiesenen
Bedarf durch Messungen erneut bewertet.

## Umsetzungsumfang und Reihenfolge fuer Phase 1

Die Umsetzung erfolgt in einem ersten vertikalen, datengetriebenen Schnitt in
folgender Reihenfolge:

1. Der Cargo-Workspace und die minimalen Crate-Grenzen werden eingerichtet.
2. `world_core` liefert IDs, Seed, Tageszeit, Tick, Koordinaten, Regionen und
   generische Fehler fuer die eigenen Grundtypen.
3. `world_data` laedt das `core-mod`, validiert dessen Daten und stellt die
   Definitionen fuer Welt, Rassen und Geografie bereit.
4. `worldgen` erzeugt daraus deterministisch die geografische Welt mit ihrer
   Weltkonfiguration, Oberwelt und den initialen Regionen.
5. Nach der geografischen Erzeugung wird ein eigener Initialisierungs-Command
   fuer die Erstbevoelkerung verarbeitet. Dieser verteilt die Rassen in
   Gruppen von jeweils 100 Individuen auf die Welt und erzeugt die zugehoerigen
   `InitialRaceSpawn`-Events.
6. `simulation` verarbeitet anschliessend den ersten Tages-Tick und fuehrt den
   Weltzustand sowie das Event-Log deterministisch fort.
7. Ein versionierter JSON-Snapshot wird gespeichert, geladen und in einem
   semantischen Roundtrip-Test verglichen.
8. `app` verbindet die Crates und stellt zunaechst nur eine Debug-Ausgabe fuer
   Seed, Tick, Weltzustand und Events bereit. Eine ausgearbeitete
   Macroquad-Oberflaeche ist nicht Teil dieses ersten Schnitts.

Der Arbeitsname des Initialisierungs-Commands ist `InitializeRacePopulation`.
Die genaue oeffentliche Command-API wird bei der Umsetzung von Phase 1
festgelegt.

## Offene Architekturfragen

- Die genaue Aufteilung der Module innerhalb der Phase-1-Crates ist noch nicht
  festgelegt.
- Die konkreten Datenformate und Merge-Regeln des Mod-Vertrags werden mit der
  ersten Mod-Ladeimplementierung weiter spezifiziert.
- Der Zeitpunkt und die genaue Reihenfolge, in der weitere Welt-Daten nach der
  geografischen Erzeugung befuellt werden, bleiben fuer die Umsetzung von
  Phase 1 offen.

## Erfolgskriterien fuer den ersten Meilenstein

Eine kleine Welt kann mit einem Seed erzeugt werden. Sie besitzt nachvollziehbare Regionen, Ressourcen, Akteure und mindestens eine Siedlung. Mehrere Simulationstakte erzeugen deterministische Events und veraendern den Zustand. Die Welt kann geladen, gespeichert und in einer einfachen Macroquad-Debugansicht betrachtet werden. Ein zweiter Lauf mit denselben Eingaben liefert dasselbe Ergebnis.

## Hauptrisiken

- Der Umfang der vier Spielmodi kann den Simulationskern ueberladen.
- Zu detaillierte Agentensimulation kann die Laufzeit und Debugbarkeit frueh zerstoeren.
- Unkontrollierter Zufall verhindert Replays und erschwert Fehleranalyse.
- Eine zu fruehe ECS- oder Multithreading-Entscheidung kann Datenmodell und APIs verkomplizieren.
- Prozedurale Inhalte koennen formal vielfaeltig, aber kausal belanglos sein.

## Gegenmassnahmen

Mit kleinen Szenarien, festen Seeds, expliziten Invarianten, Event-Logs, Snapshot-Tests und Messdaten arbeiten. Agenten koennen in unterschiedlichen Detailstufen simuliert werden; nur der relevante Ausschnitt wird hochaufgeloest. Der erste vertikale Schnitt muss Erzeugung, Ticken, Beobachtung und Speichern gemeinsam beweisen.
