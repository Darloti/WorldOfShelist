# Projekt-Roadmap

Die Roadmap ist nach validierbaren Ergebnissen geordnet, nicht nach rein technischen Komponenten. Jede Phase endet mit einem spielbaren oder messbaren Beweis, dass die vorherige Grundlage funktioniert.

## Phase 0: Fundament und Entscheidungen

- Projektvision und Systeminventar etablieren.
- Rust-Konventionen, Teststrategie und deterministische RNG-Strategie festlegen.
- Cargo-Workspace frueh einfuehren, sobald die erste fachliche Crate-Grenze
  sinnvoll umgesetzt wird; die Aufteilung soll nicht bis zu einer spaeten
  Wachstums- oder Stabilitaetsschwelle aufgeschoben werden.
- Zeitmodell, ID-Strategie, Koordinatenmodell und Persistenzversion definieren.

**Exit-Kriterium:** Die fachlichen und technischen Grundlagen fuer den
Minimal-Kern sind dokumentiert, widerspruchsfrei entschieden und als
Umsetzungsvertrag fuer Phase 1 abgrenzbar.

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

- [x] Unterschied zwischen Command, validierter Zustandsaenderung und Event dokumentieren
- [x] Erste Commands fuer die Kernsimulation benennen
- [x] Erste fachliche Events und ihre Pflichtdaten definieren
- [x] Event-ID, Tick, Ursache und betroffene Entitaeten festlegen
- [x] Validierungs- und Fehlerstrategie fuer ungueltige Commands festlegen
- [x] Reihenfolge und Verantwortlichkeit des ersten Simulations-Orchestrators definieren

#### E. Daten und Persistenz

- [x] Trennung zwischen Definitionen, Instanzen und Laufzeitstatus festlegen
- [x] Erstes Datenformat fuer Debug- und Testdaten auswaehlen
- [x] Save-Struktur mit Formatversion, Seed, Zeit und Weltzustand skizzieren
- [x] Strategie fuer Save-Migrationen und inkompatible Daten festlegen
- [x] Entscheidung treffen, welche Events dauerhaft gespeichert werden
- [x] Anforderungen fuer den minimalen Serialize-/Deserialize-Test festlegen

#### F. Qualitaet und Beobachtbarkeit

- [x] Kerninvarianten der Welt als Testfaelle formulieren
- [x] Determinismus-Test mit identischen Eingaben spezifizieren
- [x] Tick-Fortschritts- und Grenzfalltests definieren
- [x] Testfall fuer ungueltige Konfiguration oder Daten definieren
- [x] Debug-Ausgabe fuer Seed, Tick und erzeugte Events definieren
- [x] Fehlerdarstellung fuer Entwickler und spaetere Logs festlegen

#### G. Abschluss von Phase 0

- [x] Architekturentscheidungen und offene Fragen dokumentieren
- [x] Phase-0-Dokumentation auf Widersprueche und fehlende Entscheidungen pruefen
- [x] Umsetzungsumfang und Reihenfolge fuer Phase 1 festlegen
- [x] Entscheidung fuer Phase 1: Weltkern erweitern oder zuerst Workspace aufteilen

## Phase 1: Deterministischer Weltkern

Phase 1 baut den ersten datengetriebenen, deterministischen Weltkern. Die
Arbeit erfolgt in kleinen Paketen. Ein Paket gilt erst als abgeschlossen, wenn
sein Teil-Ergebnis reproduzierbar geprueft wurde.

### Phase-1-Checkliste

#### A. Workspace und Crate-Grenzen

- [X] Cargo-Workspace einrichten
- [X] Crate `world_core` im Workspace registrieren
- [X] Crate `world_data` im Workspace registrieren
- [X] Crate `worldgen` im Workspace registrieren
- [X] Crate `simulation` im Workspace registrieren
- [X] Crate `app` im Workspace registrieren
- [X] Abhaengigkeitsrichtung von `world_core` nach oben dokumentieren
- [X] Rueckverweise auf hoehere Schichten verhindern
- [X] Minimalen oeffentlichen Einstieg fuer jedes Crate einrichten
- [X] Kompilierbares Testziel fuer jedes Crate einrichten

**Pruefbares Teil-Ergebnis:** `cargo check --workspace` laeuft erfolgreich; die
Abhaengigkeiten entsprechen der dokumentierten Richtung und der Kern laedt
keine Rendering- oder Eingabeabhaengigkeit.

#### B. Gemeinsame Grundtypen in `world_core`

- [ ] `WorldId` als typisierte ID einfuehren
- [ ] `RegionId` als typisierte ID einfuehren
- [ ] `ChunkId` als typisierte ID einfuehren
- [ ] `EntityId` als typisierte ID einfuehren
- [ ] `EventId` als typisierte ID einfuehren
- [ ] `WorldSeed` modellieren
- [ ] Weltzeit mit einem Tages-Tick modellieren
- [ ] Koordinatentyp definieren
- [ ] Grundlegendes Regionenmodell definieren
- [ ] Namespaced-ID-Format validieren
- [ ] Namespaced IDs in Runtime-IDs abbilden
- [ ] Runtime-IDs zurueck in lesbare namespaced IDs abbilden
- [ ] Generische Fehler fuer die eigenen Grundtypen definieren
- [ ] Fehler fuer Verletzungen der Kerninvarianten definieren
- [ ] Reine Rust-Tests fuer gueltige und ungueltige IDs schreiben
- [ ] Reine Rust-Tests fuer Zeit, Koordinaten und Ueberlaeufe schreiben

**Pruefbares Teil-Ergebnis:** Reine Rust-Tests akzeptieren gueltige Grundtypen,
lehnen ungueltige IDs, Koordinaten, Zeitwerte und Ueberlaeufe ab und pruefen,
dass ein Tages-Tick die Weltzeit genau einmal fortschreibt.

#### C. Datenvertrag und `core-mod` in `world_data`

- [ ] Version des JSON-Datenformats festlegen
- [ ] JSON-Struktur fuer Weltkonfiguration festlegen
- [ ] JSON-Struktur fuer Mod-Metadaten festlegen
- [ ] JSON-Struktur fuer Rassendefinitionen festlegen
- [ ] JSON-Struktur fuer geografische Definitionen festlegen
- [ ] Weltkonfiguration als validierten Laufzeittyp modellieren
- [ ] `core-mod` aus einem definierten Datenpfad laden
- [ ] Mod-Identitaet validieren
- [ ] Mod-Version validieren
- [ ] Mod-Abhaengigkeiten validieren
- [ ] Mod-Datenvertrag validieren
- [ ] Definition fuer Weltgroesse modellieren
- [ ] Definition fuer Rassen modellieren
- [ ] Definition fuer Regionen modellieren
- [ ] Definition fuer geografische Parameter modellieren
- [ ] Namespaced IDs der geladenen Definitionen deterministisch aufloesen
- [ ] Fehlende Pflichtfelder ablehnen
- [ ] Unbekannte Felder ablehnen
- [ ] Ungueltige Werte ablehnen
- [ ] Inkompatible Versionen ablehnen
- [ ] Test fuer gueltige Daten schreiben
- [ ] Tests fuer die definierten Lade- und Validierungsfehler schreiben

**Pruefbares Teil-Ergebnis:** Gueltige Testdaten werden geladen und validiert;
mindestens je ein Test fuer fehlende Pflichtdaten, ungueltige Daten,
inkompatible Versionen und eine ungueltige Mod-Abhaengigkeit schlaegt mit einem
stabilen Fehler fehl.

#### D. Geografische Welterzeugung in `worldgen`

- [ ] RNG-Grundlage aus dem Welt-Seed erzeugen
- [ ] Benannte RNG-Stream-Identitaeten definieren
- [ ] Benannten RNG-Stream `worldgen` einrichten
- [ ] Benannten RNG-Stream `population` einrichten
- [ ] Deterministische RNG-Streams von anderen Streams trennen
- [ ] Weltgroesse aus der Weltkonfiguration uebernehmen
- [ ] Oberwelt erzeugen
- [ ] Geografischen Weltzustand modellieren
- [ ] Regionen mit stabilen IDs erzeugen
- [ ] Regionen an gueltige Oberweltkoordinaten binden
- [ ] Geografische Grenzen validieren
- [ ] Eindeutigkeit geografischer IDs validieren
- [ ] Reproduzierbarkeit der geografischen Koordinaten testen
- [ ] Generator-Metadaten speichern
- [ ] Unabhaengigkeit der benannten RNG-Streams testen
- [ ] Test fuer identische Konfiguration und identischen Seed schreiben
- [ ] Test fuer unterschiedliche Seeds schreiben

**Pruefbares Teil-Ergebnis:** Ein Test erzeugt mit identischer Konfiguration
und identischem Seed zweimal semantisch denselben geografischen Weltzustand;
ein anderer Seed erzeugt einen unterscheidbaren gueltigen Zustand.

#### E. Erstbevoelkerung und `InitialRaceSpawn`

- [ ] `InitializeRacePopulation` als Command modellieren
- [ ] Eingabekontext des Commands definieren
- [ ] Gueltige geografische Welterzeugung als Voraussetzung validieren
- [ ] Wiederholte Erstbevoelkerung als ungueltigen Zustand behandeln
- [ ] Gruppengroesse 100 als Daten- oder Regelwert festlegen
- [ ] Eine Gruppe fuer jede definierte Rasse erzeugen
- [ ] Position jeder Gruppe deterministisch bestimmen
- [ ] `InitialRaceSpawn` als Eventtyp modellieren
- [ ] Pro Rasse mindestens eine Event-Instanz erzeugen
- [ ] Jeder Event-Instanz eine eigene `EventId` geben
- [ ] Weltzeit, Ursache, Position und Rassenreferenz in jedem Event speichern
- [ ] Zustandsaenderung atomar anwenden
- [ ] Event-Erzeugung atomar mit der Zustandsaenderung anwenden
- [ ] Deterministische Reihenfolge der Rassen festlegen
- [ ] Deterministische Reihenfolge der Events testen
- [ ] Test fuer Ablehnung vor geografischer Welterzeugung schreiben
- [ ] Test fuer erfolgreiche Erstbevoelkerung schreiben

**Pruefbares Teil-Ergebnis:** Ein Test weist nach, dass der Command bei
ungueltiger oder fehlender Geografie abgelehnt wird, bei gueltiger Geografie
alle Rassen mit der richtigen Gruppengroesse anlegt und pro Rasse ein
nachvollziehbares `InitialRaceSpawn`-Event im Event-Log hinterlaesst.

#### F. Erster Simulations-Tick in `simulation`

- [ ] Weltzustand und Command-Verarbeitung verbinden
- [ ] Weltzustand mit Weltzeit, Geografie und Event-Log modellieren
- [ ] Append-only Event-Log anbinden
- [ ] Einen kontrollierten Tages-Tick verarbeiten
- [ ] `AdvanceSimulation` fuer den ersten Simulationskontext unterstuetzen
- [ ] `PauseSimulation` fuer den ersten Simulationskontext unterstuetzen
- [ ] `StepSimulation` fuer den ersten Simulationskontext unterstuetzen
- [ ] Ungueltige Commands ablehnen
- [ ] Zustand bei abgelehnten Commands unveraendert lassen
- [ ] Event-Log bei abgelehnten Commands unveraendert lassen
- [ ] Seed fuer Debugging ausgeben
- [ ] Tick fuer Debugging ausgeben
- [ ] Zustandsaenderungen fuer Debugging ausgeben
- [ ] Events fuer Debugging ausgeben
- [ ] Tests fuer normalen Fortschritt und Pause schreiben
- [ ] Tests fuer Einzelschritt und Ablehnung schreiben
- [ ] Test fuer deterministische Event-Reihenfolge schreiben

**Pruefbares Teil-Ergebnis:** Reine Rust-Tests pruefen normalen Fortschritt,
Pause, Einzelschritt, Ablehnung und deterministische Event-Reihenfolge. Ein
Tages-Tick wird nie teilweise oder implizit mehrfach angewendet.

#### G. Versionierter JSON-Snapshot und Zusammenschaltung

- [ ] Snapshot-Formatversion speichern
- [ ] Weltkonfiguration im Snapshot speichern
- [ ] Seed im Snapshot speichern
- [ ] Weltzeit im Snapshot speichern
- [ ] Weltzustand im Snapshot speichern
- [ ] Event-Log im Snapshot speichern
- [ ] Snapshot laden
- [ ] Inkompatible Snapshot-Versionen ablehnen
- [ ] Fehlende Snapshot-Pflichtdaten ablehnen
- [ ] Minimalen semantischen Serialize-/Deserialize-Roundtrip testen
- [ ] `app` mit Datenladung verbinden
- [ ] `app` mit Welterzeugung verbinden
- [ ] `app` mit Erstbevoelkerung verbinden
- [ ] `app` mit einem Tages-Tick verbinden
- [ ] Textuelle Debug-Ausgabe fuer Seed bereitstellen
- [ ] Textuelle Debug-Ausgabe fuer Tick bereitstellen
- [ ] Textuelle Debug-Ausgabe fuer Regionen bereitstellen
- [ ] Textuelle Debug-Ausgabe fuer Events bereitstellen

**Pruefbares Teil-Ergebnis:** Ein erzeugter Weltzustand kann gespeichert,
geladen und semantisch identisch verglichen werden. Ein zweiter Lauf mit
gleichen Eingaben erzeugt denselben Zustand und dieselben Events.

**Gesamt-Ergebnis und Exit-Kriterium:** Eine kleine Welt wird aus einem
versionierten `core-mod`, einer Konfiguration und einem Seed geografisch
erzeugt, anschliessend per `InitializeRacePopulation` mit Rassengruppen
bevoelkert und um mindestens einen Tages-Tick fortgeschrieben. Die Welt besitzt
einen nachvollziehbaren Zustand sowie `InitialRaceSpawn`-Events, kann als JSON
gespeichert und wieder geladen werden, und ein zweiter Lauf mit denselben
Eingaben erzeugt semantisch identische Ergebnisse. Ungueltige Daten und
Commands werden mit stabilen Fehlern abgelehnt.

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
