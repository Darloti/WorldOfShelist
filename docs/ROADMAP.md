# Projekt-Roadmap

Die Roadmap ist nach validierbaren Ergebnissen geordnet, nicht nach rein technischen Komponenten. Jede Phase endet mit einem spielbaren oder messbaren Beweis, dass die vorherige Grundlage funktioniert.

## Phase 0: Fundament und Entscheidungen

- Projektvision und Systeminventar etablieren.
- Rust-Konventionen, Teststrategie und deterministische RNG-Strategie festlegen.
- Cargo-Workspace frueh einfuehren, sobald die erste fachliche Crate-Grenze
  sinnvoll umgesetzt wird; die Aufteilung soll nicht bis zu einer spaeten
  Wachstums- oder Stabilitaetsschwelle aufgeschoben werden.
- Zeitmodell, ID-Strategie, Koordinatenmodell und Persistenzversion definieren.

**Exit-Kriterium:** Ein minimaler Kern kann einen Seed und einen Tick-Typ darstellen und wird durch automatisierte Tests abgesichert.

### Phase-0-Checkliste

#### A. Projektgrundlagen

- [x] Projektname und sichtbarer Spieletitel dokumentieren: `WorldOfShelist` und `World of Shelist`
- [x] Rust-Version, Edition und minimale Build-Anforderungen festlegen
- [x] Formatierungs-, Lint- und Testbefehle verbindlich festlegen
- [x] Verzeichnis- und Modulkonventionen fuer das aktuelle Starter-Crate dokumentieren
- [x] Entscheidungskriterium fuer die spaetere Aufteilung in Cargo-Crates festlegen

#### B. Fachliche Kernbegriffe

- [x] Definition von Welt, Weltzustand und Weltkonfiguration festlegen
- [x] Zentrale Entitaeten und ihre Lebensdauer erfassen
- [x] Typisierte IDs fuer Welt, Region, Chunk, Entitaet und Event definieren
- [x] Beziehungen zwischen Entitaeten als eigenes Konzept beschreiben
- [x] Koordinaten- und Regionsmodell fuer die erste kleine Welt festlegen
- [x] Einheiten und Mengen definieren, zum Beispiel Nahrung, Gewicht und Entfernung

#### C. Zeit und Determinismus

- [x] Erste Zeiteinheit und Bedeutung eines Simulations-Ticks festlegen
- [x] Regeln fuer Tick-Fortschritt, Pause und Einzelschritt definieren
- [x] Welt-Seed als Teil der Weltkonfiguration modellieren
- [x] Benannte RNG-Stroeme und ihre Verantwortlichkeiten festlegen
- [x] Verhalten bei gleicher Konfiguration, gleichem Seed und gleichen Eingaben definieren
- [x] Verhalten bei unterschiedlichen Seeds als Testfall festlegen

#### D. Zustandsaenderungen und Ereignisse

- [ ] Unterschied zwischen Command, validierter Zustandsaenderung und Event dokumentieren
- [ ] Erste Commands fuer die Kernsimulation benennen
- [ ] Erste fachliche Events und ihre Pflichtdaten definieren
- [ ] Event-ID, Tick, Ursache und betroffene Entitaeten festlegen
- [ ] Validierungs- und Fehlerstrategie fuer ungueltige Commands festlegen
- [ ] Reihenfolge und Verantwortlichkeit des ersten Simulations-Orchestrators definieren

#### E. Daten und Persistenz

- [ ] Trennung zwischen Definitionen, Instanzen und Laufzeitstatus festlegen
- [ ] Erstes Datenformat fuer Debug- und Testdaten auswaehlen
- [ ] Save-Struktur mit Formatversion, Seed, Zeit und Weltzustand skizzieren
- [ ] Strategie fuer Save-Migrationen und inkompatible Daten festlegen
- [ ] Entscheidung treffen, welche Events dauerhaft gespeichert werden
- [ ] Minimalen Serialize-/Deserialize-Test vorbereiten

#### F. Qualitaet und Beobachtbarkeit

- [ ] Kerninvarianten der Welt als Testfaelle formulieren
- [ ] Determinismus-Test mit identischen Eingaben erstellen
- [ ] Tick-Fortschritts- und Grenzfalltests erstellen
- [ ] Test fuer ungueltige Konfiguration oder Daten erstellen
- [ ] Debug-Ausgabe fuer Seed, Tick und erzeugte Events definieren
- [ ] Fehlerdarstellung fuer Entwickler und spaetere Logs festlegen

#### G. Implementierung des Minimal-Kerns

- [ ] `WorldSeed` implementieren
- [ ] `Tick` und grundlegendes Zeitmodell implementieren
- [ ] erste typisierte IDs implementieren
- [ ] minimale `World`- und `WorldConfig`-Strukturen implementieren
- [ ] deterministischen RNG-Wrapper implementieren
- [ ] minimale Event- und Command-Typen implementieren
- [ ] ersten Orchestrator mit einem kontrollierten Tick implementieren
- [ ] Tests fuer alle bisher festgelegten Invarianten implementieren

#### H. Abschluss von Phase 0

- [ ] `cargo fmt --check` erfolgreich ausfuehren
- [ ] `cargo check` erfolgreich ausfuehren
- [ ] `cargo test` erfolgreich ausfuehren
- [ ] Architekturentscheidungen und offene Fragen dokumentieren
- [ ] Phase-0-Exit-Kriterium anhand eines kleinen Testfalls nachweisen
- [ ] Entscheidung fuer Phase 1: Weltkern erweitern oder zuerst Workspace aufteilen

## Phase 1: Deterministischer Weltkern

- Welt-, Region-, Chunk-, Entity- und Event-IDs einfuehren.
- Weltzustand, Weltkonfiguration, Definitionen und Metadaten modellieren.
- Seedable RNG mit getrennten, benannten Stroemen implementieren.
- Commands, Events, Event-Log und grundlegende Invarianten definieren.
- JSON oder ein vergleichbares lesbares Format fuer Debug-Saves verwenden.

**Exit-Kriterium:** Gleicher Seed und gleiche Konfiguration erzeugen byte- oder semantisch-identische Ergebnisse; ungueltige Daten werden abgelehnt.

## Phase 2: Kleine Welt erzeugen

- Hoehen, Wasser, Klima und Biome in einer kleinen Karte erzeugen.
- Ressourcen und Lebensraeume aus Geografie ableiten, nicht rein zufaellig verteilen.
- Orte, Wege und potenzielle Siedlungsstandorte markieren.
- Generierung in nachvollziehbare, einzeln testbare Schritte teilen.
- Generatorstatistiken und einen Debug-Export ausgeben.

**Exit-Kriterium:** Mehrere Seeds ergeben unterscheidbare, gueltige Karten mit getesteten Nachbarschafts- und Ressourceninvarianten.

## Phase 3: Erste lebendige Simulation

- Zeit in grobe Simulationsschritte und spaeter feinere lokale Schritte aufteilen.
- Nahrung, Ressourcenverbrauch, einfache Population und Migration simulieren.
- Kreaturen und Bewohner als Agenten mit Beduerfnissen, Ort und einfachem Verhalten einfuehren.
- Siedlungen aus Beduerfnis, Standort und verfuegbaren Ressourcen entstehen lassen.
- Ereignisse fuer Geburt, Tod, Reise, Handel, Mangel, Gruendung und Konflikt erzeugen.

**Exit-Kriterium:** Eine erzeugte Welt veraendert sich ueber viele Ticks, bleibt innerhalb der Invarianten und erklaert wichtige Veraenderungen im Event-Log.

## Phase 4: Beobachtbarkeit und Weltansicht

- Macroquad als duennen Rendering-Adapter anschliessen.
- Karte, Zeit, Siedlungen, Agenten und letzte Events darstellen.
- Pause, Einzelschritt, Geschwindigkeiten und Seed-Anzeige bereitstellen.
- Debug-Overlays fuer Ressourcen, Population und Systemlaufzeiten ergaenzen.

**Exit-Kriterium:** Die Welt kann ohne Spielmodus betrachtet, pausiert und reproduzierbar untersucht werden.

## Phase 5: Persistenz, Replay und Skalierung

- Versionierte Saves mit Snapshot plus Event- oder Auditdaten einfuehren.
- Replay und Vergleich zweier Simulationen ermoeglichen.
- Chunking, LOD und Detailstufen erst anhand von Profiling einfuehren.
- Hintergrundsimulation fuer entfernte Regionen modellieren.
- Performancebudgets fuer Tick, Speicher und Save definieren.

**Exit-Kriterium:** Eine Welt kann gespeichert, geladen, fortgesetzt und bis zu einem definierten Budget simuliert werden.

## Phase 6: History-Modus und Weltchronik

- Ereignis- und Chronikansicht fuer die gesamte Weltgeschichte.
- Zeitleiste mit Filtern nach Regionen, Akteuren, Zivilisationen, Siedlungen und Ereignistypen.
- Historische Karten fuer Grenzen, Siedlungen, Wege und Umweltveraenderungen.
- Biografien, Dynastien, Artefaktherkunft und Beziehungen aus dem Event- und Beziehungsgraph ableiten.
- Kausalzusammenhaenge zwischen Ereignissen erklaerbar darstellen.
- Zeitspruenge und unterschiedliche Simulationsgeschwindigkeiten fuer reine Beobachtung anbieten.

**Exit-Kriterium:** Eine Welt kann ueber eine lange Zeit simuliert und anschliessend als nachvollziehbare Chronik mit historischen Karten und Akteursgeschichten betrachtet werden.

## Phase 7: Gemeinsame Interaktionsschicht

- Spieleraktionen als dieselben Commands modellieren wie autonome Aktionen.
- Sichtbarkeit, Wissen, Geruechte und Beziehungen zwischen Spieler und Welt integrieren.
- Aktivierbare Regelpakete fuer Simulation und Interaktion spezifizieren.

**Exit-Kriterium:** Ein Spieler kann in der Welt handeln, ohne Sonderpfade am Weltkern zu benoetigen.

## Phase 8: Roguelike-Modus

- Figur, Bewegung, Wahrnehmung, Kampf, Verletzung und Tod.
- Beute, Inventar, Handwerk, Quests und persoenliche Ereignisfolgen.
- Lokale Echtzeit- oder Zuglogik an den globalen Takt koppeln.

**Exit-Kriterium:** Eine Figur kann in einer laufenden Welt ein konsistentes Abenteuer erleben und ihre Folgen bleiben nachweisbar.

## Phase 9: Siedlungsbau-Modus

- Bauplaene, Bauregeln, Arbeitsauftraege und Bewohnerrollen.
- Lager, Produktion, Versorgung, Logistik und Automatisierung.
- Rekrutierung, Zufriedenheit, Krankheiten, Unfaelle und interne Konflikte.

**Exit-Kriterium:** Eine Siedlung kann gegruendet, erweitert und mit automatischen Produktionsketten betrieben werden.

## Phase 10: Koenigreich-Modus

- Herrschaftsstrukturen, Gesetze, Institutionen und Legitimität.
- Provinzen, Steuern, Handel, Diplomatie, Spionage und Kriegszustaende.
- Machtwechsel, Aufstaende, Vasallen und Untergang als Simulationsergebnisse.

**Exit-Kriterium:** Ein Koenigreich kann auf derselben Welt gegruendet oder uebernommen werden; politische Entscheidungen erzeugen langfristige Folgen.

## Phase 11: Kombinierte Kampagnen

- Moduswechsel ohne Weltreset.
- Gemeinsame Berechtigungen und Sichtweisen fuer Figur, Siedlung und Herrschaft.
- Kampagnenregeln, Sieg-/Endbedingungen und konfigurierbare Systempakete.
- Belastungstests mit langen Simulationen und vielen aktiven Entitaeten.

**Exit-Kriterium:** Roguelike, Siedlung, Koenigreich und History koennen in einer Kampagne nacheinander oder parallel relevant sein.

## Laufende Querschnittsarbeit

- Tests, Profiling, Replay-Vergleiche und Datenvalidierung
- UI/UX, Accessibility, Input und Controller-Unterstuetzung
- Audio, VFX, Lokalisierung und Modding
- Save-Migrationen, Fehlerberichte und Entwicklerwerkzeuge
- Balancing, Szenarien und qualitative Auswertung der emergenten Geschichten
