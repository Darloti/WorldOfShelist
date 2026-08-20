# Projektdefinition: World of Shelist

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

Ein stabiler Kern aus IDs, Definitionen, Zustandsdaten, Commands, Events und Zeit ist wichtiger als fruehe Crate-Anzahl. Crates werden dann getrennt, wenn sie fachliche Grenzen oder Testbarkeit verbessern. Innerhalb jedes groesseren Crates arbeiten spezialisierte Module, deren Zusammenspiel ein Orchestrator mit klarer Reihenfolge und Fehlerbehandlung steuert.

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
