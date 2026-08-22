# Phase 0: Fundament und Entscheidungen

- Projektvision und Systeminventar etablieren.
- Rust-Konventionen, Teststrategie und deterministische RNG-Strategie festlegen.
- Cargo-Workspace frueh einfuehren, sobald die erste fachliche Crate-Grenze sinnvoll umgesetzt wird; die Aufteilung soll nicht bis zu einer spaeten Wachstums- oder Stabilitaetsschwelle aufgeschoben werden.
- Zeitmodell, ID-Strategie, Koordinatenmodell und Persistenzversion definieren.

**Exit-Kriterium:** Die fachlichen und technischen Grundlagen fuer den Minimal-Kern sind dokumentiert, widerspruchsfrei entschieden und als Umsetzungsvertrag fuer Phase 1 abgrenzbar.

## Checkliste

### A. Projektgrundlagen

- [x] Projektname und sichtbarer Spieletitel dokumentieren: `WorldOfShelist` und `World of Shelist`
- [x] Rust-Version, Edition und minimale Build-Anforderungen festlegen
- [x] Formatierungs-, Lint- und Testbefehle verbindlich festlegen
- [x] Verzeichnis- und Modulkonventionen fuer das aktuelle Starter-Crate dokumentieren
- [x] Entscheidungskriterium fuer die spaetere Aufteilung in Cargo-Crates festlegen

### B. Fachliche Kernbegriffe

- [x] Definition von Welt, Weltzustand und Weltkonfiguration festlegen
- [x] Zentrale Entitaeten und ihre Lebensdauer erfassen
- [x] Typisierte IDs fuer Welt, Region, Chunk, Entitaet und Event definieren
- [x] Beziehungen zwischen Entitaeten als eigenes Konzept beschreiben
- [x] Koordinaten- und Regionsmodell fuer die erste kleine Welt festlegen
- [x] Einheiten und Mengen definieren, zum Beispiel Nahrung, Gewicht und Entfernung

### C. Zeit und Determinismus

- [x] Erste Zeiteinheit und Bedeutung eines Simulations-Ticks festlegen
- [x] Regeln fuer Tick-Fortschritt, Pause und Einzelschritt definieren
- [x] Welt-Seed als Teil der Weltkonfiguration modellieren
- [x] Benannte RNG-Stroeme und ihre Verantwortlichkeiten festlegen
- [x] Verhalten bei gleicher Konfiguration, gleichem Seed und gleichen Eingaben definieren
- [x] Verhalten bei unterschiedlichen Seeds als Testfall festlegen

### D. Zustandsaenderungen und Ereignisse

- [x] Unterschied zwischen Command, validierter Zustandsaenderung und Event dokumentieren
- [x] Erste Commands fuer die Kernsimulation benennen
- [x] Erste fachliche Events und ihre Pflichtdaten definieren
- [x] Event-ID, Tick, Ursache und betroffene Entitaeten festlegen
- [x] Validierungs- und Fehlerstrategie fuer ungueltige Commands festlegen
- [x] Reihenfolge und Verantwortlichkeit des ersten Simulations-Orchestrators definieren

### E. Daten und Persistenz

- [x] Trennung zwischen Definitionen, Instanzen und Laufzeitstatus festlegen
- [x] Erstes Datenformat fuer Debug- und Testdaten auswaehlen
- [x] Save-Struktur mit Formatversion, Seed, Zeit und Weltzustand skizzieren
- [x] Strategie fuer Save-Migrationen und inkompatible Daten festlegen
- [x] Entscheidung treffen, welche Events dauerhaft gespeichert werden
- [x] Anforderungen fuer den minimalen Serialize-/Deserialize-Test festlegen

### F. Qualitaet und Beobachtbarkeit

- [x] Kerninvarianten der Welt als Testfaelle formulieren
- [x] Determinismus-Test mit identischen Eingaben spezifizieren
- [x] Tick-Fortschritts- und Grenzfalltests definieren
- [x] Testfall fuer ungueltige Konfiguration oder Daten definieren
- [x] Debug-Ausgabe fuer Seed, Tick und erzeugte Events definieren
- [x] Fehlerdarstellung fuer Entwickler und spaetere Logs festlegen

### G. Abschluss von Phase 0

- [x] Architekturentscheidungen und offene Fragen dokumentieren
- [x] Phase-0-Dokumentation auf Widersprueche und fehlende Entscheidungen pruefen
- [x] Umsetzungsumfang und Reihenfolge fuer Phase 1 festlegen
- [x] Entscheidung fuer Phase 1: Weltkern erweitern oder zuerst Workspace aufteilen
