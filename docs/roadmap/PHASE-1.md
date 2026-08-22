# Phase 1: Deterministischer Weltkern

Phase 1 baut den ersten datengetriebenen, deterministischen Weltkern. Die Arbeit erfolgt in kleinen Paketen. Ein Paket gilt erst als abgeschlossen, wenn sein Teil-Ergebnis reproduzierbar geprueft wurde.

## Checkliste

### A. Workspace und Crate-Grenzen

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

**Pruefbares Teil-Ergebnis:** `cargo check --workspace` laeuft erfolgreich; die Abhaengigkeiten entsprechen der dokumentierten Richtung und der Kern laedt keine Rendering- oder Eingabeabhaengigkeit.

### B. Gemeinsame Grundtypen in `world_core`

- [X] `WorldId` als typisierte ID einfuehren
- [X] `RegionId` als typisierte ID einfuehren
- [X] `ChunkId` als typisierte ID einfuehren
- [X] `EntityId` als typisierte ID einfuehren
- [X] `EventId` als typisierte ID einfuehren
- [X] `WorldSeed` modellieren
- [ ] Weltzeit in Millisekunden mit einem Tages-Tick modellieren
- [ ] Koordinatentyp definieren
- [ ] Grundlegendes Regionenmodell definieren
- [ ] Namespaced-ID-Format validieren
- [ ] Namespaced IDs in Runtime-IDs abbilden
- [ ] Runtime-IDs zurueck in lesbare namespaced IDs abbilden
- [ ] Generische Fehler fuer die eigenen Grundtypen definieren
- [ ] Fehler fuer Verletzungen der Kerninvarianten definieren
- [ ] Reine Rust-Tests fuer gueltige und ungueltige IDs schreiben
- [ ] Reine Rust-Tests fuer Zeit, Koordinaten und Ueberlaeufe schreiben

**Pruefbares Teil-Ergebnis:** Reine Rust-Tests akzeptieren gueltige Grundtypen, lehnen ungueltige IDs, Koordinaten, Zeitwerte und Ueberlaeufe ab und pruefen, dass ein Tages-Tick die Weltzeit genau einmal fortschreibt.

### C. Datenvertrag und `core-mod` in `world_data`

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

**Pruefbares Teil-Ergebnis:** Gueltige Testdaten werden geladen und validiert; mindestens je ein Test fuer fehlende Pflichtdaten, ungueltige Daten, inkompatible Versionen und eine ungueltige Mod-Abhaengigkeit schlaegt mit einem stabilen Fehler fehl.

### D. Geografische Welterzeugung in `worldgen`

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

**Pruefbares Teil-Ergebnis:** Ein Test erzeugt mit identischer Konfiguration und identischem Seed zweimal semantisch denselben geografischen Weltzustand; ein anderer Seed erzeugt einen unterscheidbaren gueltigen Zustand.

### E. Erstbevoelkerung und `InitialRaceSpawn`

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

**Pruefbares Teil-Ergebnis:** Ein Test weist nach, dass der Command bei ungueltiger oder fehlender Geografie abgelehnt wird, bei gueltiger Geografie alle Rassen mit der richtigen Gruppengroesse anlegt und pro Rasse ein nachvollziehbares `InitialRaceSpawn`-Event im Event-Log hinterlaesst.

### F. Erster Simulations-Tick in `simulation`

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

**Pruefbares Teil-Ergebnis:** Reine Rust-Tests pruefen normalen Fortschritt, Pause, Einzelschritt, Ablehnung und deterministische Event-Reihenfolge. Ein Tages-Tick wird nie teilweise oder implizit mehrfach angewendet.

### G. Versionierter JSON-Snapshot und Zusammenschaltung

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

**Pruefbares Teil-Ergebnis:** Ein erzeugter Weltzustand kann gespeichert, geladen und semantisch identisch verglichen werden. Ein zweiter Lauf mit gleichen Eingaben erzeugt denselben Zustand und dieselben Events.

**Gesamt-Ergebnis und Exit-Kriterium:** Eine kleine Welt wird aus einem versionierten `core-mod`, einer Konfiguration und einem Seed geografisch erzeugt, anschliessend per `InitializeRacePopulation` mit Rassengruppen bevoelkert und um mindestens einen Tages-Tick fortgeschrieben. Die Welt besitzt einen nachvollziehbaren Zustand sowie `InitialRaceSpawn`-Events, kann als JSON gespeichert und wieder geladen werden, und ein zweiter Lauf mit denselben Eingaben erzeugt semantisch identische Ergebnisse. Ungueltige Daten und Commands werden mit stabilen Fehlern abgelehnt.
