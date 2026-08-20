# Systemkonzept und Systeminventar

Dieses Dokument ist ein Arbeitsgeruest. Die Ueberschriften definieren moegliche Systemgrenzen und werden in spaeteren Sitzungen einzeln befuellt. Ein System sollte erst implementiert werden, wenn Zweck, Eingaben, Ausgaben, Invarianten und Beobachtbarkeit beschrieben sind.

## 1. Kernmodell und Identitaet

### Weltzustand
### Entitaeten und stabile IDs
### Namespaced-IDs und Runtime-ID-Woerterbuch
### Komponenten und Aggregate
### Beziehungen und Referenzen
### Koordinaten, Regionen und Chunks
### Zeit, Kalender und Zeitskalen
### Einheiten und Mengen
### Definitionen, Templates und Instanzen
### Versionierung und Migration

## 2. Datengetriebene Inhalte

### Datenquellen und Dateiformate
### Schema, Defaults und Validierung
### Namensgeneratoren und Sprachdaten
### Tags, Kategorien und Abhaengigkeiten
### Modding und Erweiterungspunkte
### Balancing und Konfigurationsprofile

### Core-Mod und generische Engine-Vertraege

## 3. Weltgenerierung

### Seed- und Zufallsstrategie
### Generator-Pipeline und Orchestrator
### Hoehen, Gelaende und Geologie
### Wasser, Fluesse, Seen und Kuesten
### Klima, Jahreszeiten und Wetter
### Biome, Flora und Fauna
### Ressourcen, Vorkommen und Erneuerung
### Regionen, Grenzen und natuerliche Barrieren
### Wege, Verkehr und Siedlungsstandorte
### Orte, Ruinen und Points of Interest
### Erstbevoelkerung und Weltgeschichte
### Generatorvalidierung und Reparaturschritte

## 4. Simulationslaufzeit

### Tick-Scheduler und Systemreihenfolge
### Globale, regionale und lokale Detailstufen
### Zeitfortschritt, Pause und Beschleunigung
### Commands und Validierung
### Events, Eventtypen und Event-Log
### Event-Abonnenten und Reaktionen
### Determinismus und Replay
### Fehlerbehandlung und Wiederanlauf
### Performancebudgets und Profiling

## 5. Umwelt und Natur

### Tageszeit und Beleuchtung
### Wetter und Naturkatastrophen
### Jahreszeiten und Erntezyklen
### Nahrungsketten und Populationen
### Krankheiten und Seuchen
### Feuer, Zerstoerung und Regeneration
### Magische oder uebernatuerliche Umweltveraenderungen

## 6. Bewohner und Kreaturen

### Spezies, Varianten und Koerpermodelle
### Attribute, Faehigkeiten und Traits
### Beduerfnisse, Ressourcen und Prioritaeten
### Gesundheit, Verletzung, Alter und Tod
### Beduerfnisse von Tieren und Monstern
### Tagesablaeufe und Rollen
### Bewegung, Reisen und Navigation
### Wahrnehmung, Wissen und Erinnerungen
### Ziele, Entscheidungen und Verhalten
### Familien, Abstammung und Vererbung
### Beziehungen, Gruppen und soziale Netzwerke
### Namens-, Kultur- und Identitaetsbildung

## 7. Zivilisation und Gesellschaft

### Stämme, Voelker und Kulturen
### Werte, Normen und Tabus
### Religionen, Kulte und Mythen
### Sprachen und Namensraeume
### Klassen, Berufe und soziale Schichten
### Institutionen und Organisationen
### Recht, Verbrechen und Bestrafung
### Migration, Integration und Minderheiten
### Bildung, Wissen und Technologie

## 8. Siedlungen und Infrastruktur

### Siedlungsgruendung und Standortlogik
### Gebaeude, Module und Bauplaene
### Zonen, Grundstuecke und Besitz
### Arbeitsauftraege und Bewohnerzuweisung
### Lager, Inventare und Ressourcenlogistik
### Nahrung, Wasser und Versorgung
### Handwerk und Produktionsketten
### Handel, Preise und Maerkte
### Strassen, Transport und Reisezeiten
### Wachstum, Verfall und Wiederaufbau

## 9. Politik und Koenigreiche

### Herrschaft, Legitimität und Nachfolge
### Territorien, Provinzen und Grenzen
### Gesetze, Steuern und Verwaltung
### Diplomatie, Vertraege und Beziehungen
### Fraktionen, Interessen und Machtgruppen
### Spionage, Intrigen und Desinformation
### Militaer, Armeen und Versorgung
### Krieg, Belagerung und Frieden
### Aufstaende, Revolutionen und Zerfall
### Imperien, Vasallen und Staatenbildung

## 10. Konflikt, Abenteuer und Interaktion

### Spielerfigur und kontrollierte Akteure
### Bewegung und lokale Zeit
### Wahrnehmung, Sicht und Nebel des Unwissens
### Kampf, Schaden und Statuswirkungen
### Beute, Inventar und Ausruestung
### Handwerk und Reparatur
### Quests, Auftraege und Geruechte
### Dialoge und soziale Aktionen
### Verbrechen, Ruf und Konsequenzen
### Tod, Nachfolge und Kampagnenfortsetzung

## 11. Artefakte und Geschichte

### Gegenstaende und materielle Eigenschaften
### Artefaktherstellung und Herkunft
### Magie, Verzauberungen und Kosten
### Besitzer, Vererbung und Verlust
### Relikte, Legenden und Bekanntheit
### Ereignisketten und historische Epochen
### Chronik, Biografien und Kausalitaetsgraph

## 12. Spielmodi und Regelpakete

### Modus-Schnittstelle und Aktivierung
### Roguelike-Regeln
### Siedlungsbau-Regeln
### Koenigreich-Regeln
### History-Regeln und Beobachterperspektive
### Gemeinsame Commands und Ressourcen
### Moduswechsel und Besitzwechsel
### Kombinierte Eingaben und Prioritaeten
### Siegbedingungen, Niederlage und Weltende

## 13. History-Modus und Weltchronik

### Chronikmodell und historische Epochen
### Ereignisfilter und Suchindex
### Historische Karten und Grenzveraenderungen
### Biografien, Dynastien und Genealogien
### Zivilisations- und Siedlungsgeschichten
### Artefaktherkunft und Besitzverlaeufe
### Kausalitaetsgraph und erklaerbare Zusammenhaenge
### Geruechte, Unsicherheit und widerspruechliche Quellen
### Zeitspruenge, Simulation bis Zielzeit und Beobachtungsmodus
### Zusammenfassungen fuer lange Zeitabschnitte

## 14. Rendering und Darstellung

### Macroquad-Anwendungsloop
### Weltkarte und Kamera
### Tile-, Chunk- und Layer-Rendering
### Darstellung von Entitaeten und Ereignissen
### UI, Panels und Informationshierarchie
### Debug- und Entwickleransichten
### LOD, Culling und Streaming
### Animationen, Partikel und Effekte
### Audio und Musik
### Eingabe, Bindings und Accessibility

## 15. Persistenz und Werkzeuge

### Save-Snapshots und Save-Slots
### Event-Log und Replay-Dateien
### Kompression, Checkpoints und Autosaves
### Save-Migration und Abwaertskompatibilitaet
### Import/Export und Debug-Dumps
### Weltgenerator- und Simulations-CLI
### Szenario- und Testdaten
### Telemetrie, Metriken und Diagnose
### Crash-Recovery und korrupte Saves

## 16. Architektur, Sicherheit und Betrieb

### Cargo-Workspace und Crate-Grenzen
### Orchestratoren und Abhaengigkeitsrichtung
### Threading, Parallelisierung und Synchronisation
### Speicherbudget und Datenlayout
### Deterministische Plattformunterschiede
### Testpyramide und Invarianten
### Fuzzing und Property-Based Tests
### Performance- und Langzeitsimulationstests
### Release, Packaging und Updates

## 17. Offene Designfragen

### Wie fein ist ein Tick in den jeweiligen Modi?
### Welche Agenten werden voll und welche aggregiert simuliert?
### Welche Ereignisse sind dauerhaft zu speichern?
### Ist die Welt endlich, unendlich oder streamingfaehig?
### Wie stark veraendert Magie Natur- und Gesellschaftsregeln?
### Welche Informationen darf der Spieler sehen?
### Welche Regeln muessen modifizierbar sein?
### Welche technischen Grenzen gelten fuer die erste Zielplattform?
