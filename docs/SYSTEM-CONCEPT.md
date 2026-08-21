# Systemkonzept und Systeminventar

Dieses Dokument ist ein Arbeitsgeruest. Die Ueberschriften definieren moegliche Systemgrenzen und werden in spaeteren Sitzungen einzeln befuellt. Ein System sollte erst implementiert werden, wenn Zweck, Eingaben, Ausgaben, Invarianten und Beobachtbarkeit beschrieben sind.

## 1. Kernmodell und Identitaet

### Weltzustand

Der Weltzustand ist der veraenderliche Zustand einer konkreten Welt zu
einem bestimmten Simulationszeitpunkt. Er umfasst alle erzeugten Instanzen,
ihre aktuellen Eigenschaften und ihre Beziehungen. Dazu gehoeren insbesondere
Geografie, Regionen, Chunks, Ressourcen, Bewohner, Kreaturen, Siedlungen,
Zivilisationen, Gegenstaende, Orte und laufende Prozesse.

Nicht jeder Bestandteil muss von Anfang an mit voller Detailtiefe erzeugt
werden. Die fachlichen Kategorien werden frueh definiert; konkrete regionale
und lokale Daten koennen deterministisch bei Bedarf erzeugt werden.

### Welt und Weltkonfiguration

`World` ist der uebergeordnete fachliche Container einer konkreten Welt. Er
verbindet mindestens:

- die stabile Identitaet der Welt
- die Weltkonfiguration
- den aktuellen Weltzustand
- die Weltgeschichte beziehungsweise das relevante Event-Log
- persistierte Detaildaten und Erzeugungsmetadaten

Die Weltkonfiguration enthaelt alle Eingaben, die fuer eine reproduzierbare
Erzeugung der Oberwelt und ihrer Metadaten erforderlich sind. Dazu gehoeren
mindestens:

- Welt-Seed
- Weltgroesse und Ausdehnung der Oberwelt
- Generator-Version
- generatorrelevante Mod-IDs, Mod-Versionen und Datenvertraege
- generatorrelevante Definitionen, zum Beispiel Biome, Klima und Ressourcen
- der feste Startzeitpunkt: Tag 1, Monat 1, Jahr 1

Die Oberwelt wird einmalig aus dieser Konfiguration erzeugt. Ihre Geografie
und die dabei festgelegten Regions-Metadaten sind danach Bestandteil des
Weltzustands. Eine Aenderung dieser Erzeugungsgrundlagen erzeugt eine neue
Welt oder erfordert eine ausdrueckliche, versionierte Migration.

### Kartenebenen und Detailerzeugung

Die Welt verwendet mehrere fachliche Detailstufen:

```text
Oberwelt
`-- Region
    `-- lokale Detailkarte
        `-- Point of Interest
            `-- optionale Instance
```

Die Oberwelt enthaelt die globale Geografie, Regionen und Regions-Metadaten.
Regions-Metadaten werden direkt erzeugt und koennen unter anderem Klima,
Biom, Ressourcenpotenzial und weitere geografische Eigenschaften beschreiben.
Eine lokale Detailkarte wird deterministisch erzeugt, wenn sie fuer eine
Detailansicht, Simulation oder Aktion benoetigt wird.

Noch nicht erzeugte Details duerfen keine widerspruechlichen Ergebnisse zu
den bereits festgelegten Oberwelt- und Regionsdaten liefern.

### Orte, Spawner und Instanzen

Ein `PointOfInterest` ist ein dauerhaft adressierbarer Ort mit fachlicher
Bedeutung. Beispiele sind eine Wolfshoehle, ein Daemonentor, eine Ruine oder
ein besonderer Ressourcenort. Ein Ort kann bei der Weltgenerierung oder spaeter
durch die Simulation entstehen. Seine Herkunft muss ueber ein erzeugendes
Event, beteiligte Entitaeten und den relevanten Erzeugungskontext
nachvollziehbar sein.

Ein Ort kann unter anderem enthalten:

- Position und fachlichen Ortstyp
- aktuellen Lebenszyklusstatus
- Spawner- oder Aktionsregeln
- Referenzen auf beteiligte Entitaeten
- eine optionale Referenz auf eine Instanz

Ein `Spawner` ist eine Regel oder ein Zustand eines Ortes, aus dem spaeter
Entitaeten, Gruppen oder weitere Events entstehen koennen. Dadurch koennen
beispielsweise Wolfsrudel eine Wolfshoehle durch Migration entstehen lassen
oder ein Magier ein Daemonentor durch ein Ritual erzeugen.

Eine `Instance` ist eine optionale, separat betretbare Detailwelt, die an einen
Ort oder einen anderen fachlichen Uebergang gebunden ist. Sie besitzt eine
eigene lokale Geografie, eigene Entitaeten und einen eigenen lokalen Zustand.
Eine Instanz muss nicht gemeinsam mit dem Ort erzeugt werden, sondern kann bei
Bedarf deterministisch erzeugt und anschliessend persistiert werden.

Vor der ersten Erzeugung ist die Instanz durch ihren Erzeugungskontext
beschrieben. Nach ihrer Erzeugung ist bei unveraendertem Zustand weiterhin die
deterministische Regeneration moeglich. Sobald sie betreten oder veraendert
wurde, ist der persistierte Zustand massgeblich.

Die Spielmodi greifen mit unterschiedlicher Detailtiefe auf diese Daten zu:

- Der Roguelike-Modus kann eine Instanz betreten und lokal simulieren.
- Der Siedlungsmodus behandelt einen Ort als Spawner, Gefahrenquelle,
  Ressourcenquelle oder Aktionspunkt, ohne die Instanzkarte zu laden.
- Der Koenigreich-Modus behandelt einen Ort als strategischen oder politischen
  Punkt, ohne seine interne Karte zu simulieren.
- Der History-Modus zeigt Ort, Herkunft und Ereignisse; die Instanzkarte ist
  nur bei historisch relevanten Ereignissen erforderlich.

Die Spielmodi erzeugen keine parallelen Welten. Sie verwenden denselben
Weltzustand und waehlen nur eine passende Detailstufe und Zugriffsperspektive.

### Herkunft und Lebensdauer

Dynamisch erzeugte Orte und Instanzen sind fachliche Objekte mit eigenem
Lebenszyklus. Ein Ort kann beispielsweise geplant, aktiv, ruhend, entdeckt,
versiegelt, zerstoert oder entfernt sein. Die tatsaechlichen Zustaende werden
je Ortstyp validiert und nicht als unkontrollierte freie Strings gefuehrt.

Die Erzeugung eines Ortes oder einer Instanz ist eine nachvollziehbare
Zustandsaenderung und wird nicht nur als direkte Feldaenderung behandelt. Sie
erzeugt ein fachliches Event mit mindestens Tick, eigener Event-ID,
Objekt-ID, Ort beziehungsweise Position und Ursache. Ursachen koennen unter
anderem Weltgenerierung, Migration, ein Simulationsereignis, eine
Akteurshandlung, ein Spieler-Command oder ein Mod-Command sein.

Fuer dieselbe Weltkonfiguration, denselben Seed, dieselben Eingaben und
dieselbe Eventreihenfolge muessen dynamisch entstandene Orte und Instanzen
identisch entstehen. Zufallsentscheidungen verwenden dafuer benannte,
deterministische RNG-Stroeme.

### Entitaeten und stabile IDs

Die Welt unterscheidet zwischen fachlichen Definitionen, konkreten Entitaeten
und aggregierten Zustandsdaten. Eine Entitaet erhaelt eine eigene stabile ID,
wenn sie dauerhaft referenzierbar, historisch relevant oder fuer eine lokale
Simulation wichtig ist. Nicht jede simulierte Einheit wird deshalb als
Einzelobjekt materialisiert.

Definitionen sind keine konkreten Weltentitaeten. Eine `SpeciesDefinition`
beschreibt beispielsweise, was ein Elf ist. Eine aggregierte Population oder
ein konkreter historisch relevanter Charakter beschreibt dagegen, welche Elfen
in einer bestimmten Welt existieren. Dasselbe Prinzip gilt fuer Biome,
Gegenstandstypen, Ortstypen und Gebaeudearten.

Die zentralen Weltentitaeten sind:

- `World`: konkrete Weltinstanz mit Konfiguration, Zustand und Geschichte
- `Region`: dauerhafter geografischer Bereich der Oberwelt
- `LocalMap`: erzeugte lokale Detailkarte einer Region
- `PointOfInterest`: fachlich relevanter Ort innerhalb einer Region
- `Instance`: optionale betretbare Detailkarte, die an einen Ort gebunden ist
- `ResourceDeposit`: Ressourcenvorkommen innerhalb einer Region
- `Species`: definierte Spezies oder Rasse
- `Character`: historisch oder lokal relevanter individueller Akteur
- `Group`: relevante Gruppe, zum Beispiel ein Wolfsrudel oder eine Reisegruppe
- `Settlement`: Siedlung mit eigenem Zustand und eigener Geschichte
- `Civilization`: Zivilisation, die entstehen, wachsen, zerfallen und neu
  gegruendet werden kann
- `Faction`: relevante politische, soziale oder ideologische Fraktion
- `Organization`: konkrete Organisation innerhalb oder ausserhalb einer
  Zivilisation
- `Item`: konkreter Gegenstand mit lokaler oder spielerischer Relevanz
- `Artifact`: historisch bedeutender Gegenstand mit eigener Herkunft
- `Building`: erzeugtes und relevantes Gebaeude innerhalb einer Siedlung oder
  lokalen Karte
- `Route`: dauerhafte oder historisch relevante Verkehrsverbindung
- `Event`: unveraenderlicher Eintrag der Weltgeschichte
- `Relationship`: relevante Beziehung zwischen zwei oder mehreren Entitaeten

Ressourcenvorkommen sind an eine Region gebunden. Eine lokale Detailkarte
oder eine Instanz kann deren konkrete lokale Auspraegung und Nutzung enthalten,
das regionale Vorkommen bleibt jedoch die uebergeordnete fachliche Referenz.

### Aggregation und historische Relevanz

Kreaturen, Bewohner, Gruppen und Rudel werden standardmaessig aggregiert
simuliert. Einzelne Entitaeten entstehen nur, wenn sie fuer die lokale
Simulation, einen aktiven Spielmodus oder die Geschichte relevant werden.
Historisch relevante Figuren und Gruppen erhalten dann eine eigene stabile ID
und koennen in Events und Beziehungen referenziert werden. Nicht relevante
Individuen bleiben Teil aggregierter Populationen oder Gruppenzustaende.

Dasselbe Prinzip gilt fuer Gruppen und Rudel: Ein Rudel kann als aggregierte
Population bestehen und bei einem relevanten Ereignis als konkrete `Group`
materialisiert werden. Die Materialisierung darf keine widerspruechliche
Doppelzaehlung erzeugen; die aggregierte Menge wird dabei entsprechend
angepasst.

Gegenstaende werden nur solange als eigene `Item`-Entitaeten gefuehrt, wie sie
in einer lokalen Simulation oder fuer den Spieler relevant sind. Ein
historisch bedeutender Gegenstand kann zum `Artifact` aufgewertet werden. Die
Aufwertung erhaelt die Identitaet des Gegenstands und fuehrt seine Herkunft
fort. Beispiel: Ein Schwert wird nach dem Toeten eines Drachen zu einem
Artefakt, weil dieses Ereignis seine historische Bedeutung veraendert.

Nicht mehr relevante Gegenstaende muessen nicht dauerhaft als aktiver
Laufzeitzustand erhalten bleiben. Sobald ein Gegenstand jedoch historisch
relevant wird, wird er als individuelle Entitaet materialisiert oder dauerhaft
als historische Referenz gefuehrt. Seine Herkunft wird durch die ausloesenden
Events nachvollziehbar gehalten.

Gebaeude werden als Entitaeten gefuehrt, sobald sie erzeugt und fuer die
Simulation, Geschichte oder einen Spielmodus relevant sind. Wege werden als
`Route` gefuehrt, wenn sie dauerhaft, genutzt oder historisch relevant sind.

### Lebensdauer und Identitaet

Regionen bleiben dauerhaft bestehen. Lokale Detailkarten und Instanzen koennen
bei Bedarf erzeugt und persistiert werden. Points of Interest, Siedlungen,
Gebaeude, Gegenstaende und Artefakte koennen geschaffen, veraendert, verlassen
oder zerstoert werden. Ein Verlassen oder eine temporaere Inaktivitaet beendet
die Identitaet nicht. Eine spaetere Wiederbesiedlung oder Reaktivierung bezieht
sich daher weiterhin auf dieselbe Entitaet, sofern kein neues fachliches
Objekt gegruendet wird.

Zivilisationen koennen zerfallen. Eine spaetere Neugruendung ist eine neue
Zivilisationsentitaet, auch wenn sie auf derselben Kultur, Bevoelkerung oder
historischen Tradition aufbaut. Fraktionen und Organisationen folgen derselben
Regel, sofern die Neugruendung fachlich eine neue Identitaet darstellt.

Gegenstaende und Artefakte koennen geschaffen und zerstoert werden. Die
Zerstoerung beendet die aktive Verfuegbarkeit, aber die historische Referenz
auf ein relevantes Artefakt oder einen relevanten Gegenstand kann in Events
und Beziehungen erhalten bleiben.

### Ereignisse und Beziehungen

Events existieren fortlaufend als append-only Weltgeschichte. Ein Event ist
eine unveraenderliche historische Wahrheit und darf nach seiner Annahme nicht
inhaltlich ueberschrieben werden. Events koennen je nach Eventtyp die damals
relevanten Entitaeten, Beziehungen, Orte, Positionen, Ursachen und
Zustandsaenderungen referenzieren.

Ein Event muss nicht jede aggregierte Einheit einzeln referenzieren. Es soll
aber alle Entitaeten festhalten, die fuer die historische Erklaerbarkeit des
Ereignisses relevant sind. Spaetere Darstellungen und Chroniken werden aus
diesen Events und den aktuellen beziehungsweise persistierten Entitaeten
abgeleitet.

Beziehungen werden nur zwischen relevanten Entitaeten als eigene fachliche
Strukturen gefuehrt. Sie koennen beispielsweise Verwandtschaft, Feindschaft,
Zugehoerigkeit, Besitz, politische Abhaengigkeit oder Kooperation darstellen.
Eine Beziehung besitzt einen eigenen Zustand und kann durch Events entstehen,
sich veraendern oder enden. Die historische Tatsache ihrer Entstehung oder
Aenderung bleibt im Event-Log erhalten.

Ein `Command` ist eine Eingabe oder angeforderte Aktion und noch kein Teil der
Weltgeschichte. Erst ein validierter und angewendeter Zustandswechsel erzeugt
ein unveraenderliches fachliches Event. Abgelehnte Commands koennen fuer
Diagnose und Logs aufgezeichnet werden, sind aber keine historischen Events.

Eine `LocalMap` beschreibt einen detaillierten Ausschnitt der Oberwelt und
bleibt an dessen Region gebunden. Eine `Instance` beschreibt dagegen einen
separaten betretbaren Raum oder Szenariobereich, der an einen Point of
Interest oder einen anderen fachlichen Uebergang gebunden ist. Beide koennen
dieselben technischen Mechanismen fuer deterministische Erzeugung und
Persistenz verwenden, bleiben aber fachlich unterschiedliche Kartentypen.

### Namespaced-IDs und Runtime-ID-Woerterbuch

Fachliche Definitionen und datengetriebene Inhalte werden ueber stabile,
genau dreiteilige `NamespacedId`-Werte adressiert, zum Beispiel
`core:species:wolf`. Konkrete Laufzeitentitaeten speichern dagegen nur ihre
passende typisierte numerische ID. Ein konkreter Wolf kann daher intern als
`EntityId(3305)` gespeichert werden und auf die Definition
`core:species:wolf` verweisen.

Das Runtime-ID-Woerterbuch ist Bestandteil des Welt- beziehungsweise
Save-Kontexts und bildet numerische IDs auf lesbare Namespaced-IDs ab. Es ist
fuer Debug-Ausgaben, Fehler, Events und Werkzeuge zustaendig; Laufzeitdaten
duerfen die lesbare ID nicht als Ersatz fuer die interne Referenz speichern.
Eine Rueckuebersetzung muss moeglich sein, damit Ausgaben wie
`entity#3305 (core:species:wolf)` erzeugt werden koennen. Nicht jede konkrete
Entitaet benoetigt eine eigene Namespaced-ID; in diesem Fall bleibt nur ihre
typisierte interne ID erhalten.

Die Vergabe typisierter IDs erfolgt fortlaufend und getrennt je ID-Typ. Die
Vergabereihenfolge ist Teil des deterministischen Erzeugungsablaufs. Fuer die
erste Kernimplementierung werden `WorldId`, `RegionId`, `ChunkId`, `EntityId`
und `EventId` verwendet. IDs werden als `u64`-Newtypes gefuehrt und fuer
Persistenz sowie Debug-Ausgaben direkt serialisierbar beziehungsweise
lesbar dargestellt.

### Komponenten und Aggregate

### Beziehungen und Referenzen

Beziehungen sind eigene fachliche Strukturen zwischen zwei oder mehr stabilen
Entitaeten. Die meisten Beziehungen verbinden zwei Entitaeten, das Modell muss
aber auch mehrteilige Beziehungen abbilden koennen. Beziehungen sind
grundsaetzlich gerichtet, sofern der Beziehungstyp keine ausdruecklich
ungerichtete Bedeutung besitzt.

Fuer den ersten Kernumfang werden folgende Beziehungstypen benoetigt:

- `CivilizationMembership`: Eine Entitaet gehoert einer Zivilisation an.
- `Residence`: Eine Entitaet hat einen Heimatort oder einen sesshaften
  Aufenthaltsort.
- `Ownership`: Eine Entitaet besitzt eine Struktur oder einen Gegenstand.

Eine `Residence` ist optional. Nomadenstaemme, wandernde Gruppen und
Abenteurer koennen ohne konkrete Heimat existieren. Ein Heimatort kann auf
eine `Settlement` oder einen `PointOfInterest` verweisen. Ein voruebergehender
Aufenthalt oder eine Reise erzeugt nicht automatisch eine `Residence`.

Jede Beziehung besitzt einen eigenen Lebenszyklus und kann entstehen, aktiv
sein, veraendert oder beendet werden. Der aktuelle Beziehungszustand wird im
Weltzustand gehalten. Entstehung, Aenderung und Beendigung werden durch
definierte, gespeicherte Events dokumentiert. Historische und Debug-Ansichten
werden aus diesen Events und dem aktuellen Zustand abgeleitet.

Beziehungen referenzieren die internen typisierten IDs der beteiligten
Entitaeten. Eine Beziehung darf nicht auf eine nicht existente Entitaet
verweisen. Unerlaubte Selbstbeziehungen und doppelte aktive Beziehungen
werden abgelehnt. Beziehungstypen koennen zusaetzliche Invarianten festlegen;
beispielsweise darf eine sesshafte Entitaet hoechstens eine aktive
Heimatort-Beziehung besitzen. Beginn und Ende eines Beziehungslebenszyklus
muessen zeitlich gueltig sein. Das Beenden einer Beziehung beendet ihre
aktive Verfuegbarkeit, entfernt aber nicht ihre historische Referenz.

### Koordinaten, Regionen und Chunks

Die erste Welt ist rechteckig und verwendet zweidimensionale, ganzzahlige
Koordinaten. Der Ursprung liegt in der linken oberen Ecke. Beide Koordinaten
sind nicht negativ; `x` waechst nach Osten und `y` nach Sueden. Eine
`WorldCellCoord` adressiert eine konkrete Zelle der Oberwelt und liegt im
Bereich `0..width` beziehungsweise `0..height` der Welt.

Es gibt drei fachlich getrennte Koordinatenpaare:

- `WorldCellCoord`: globale Koordinate einer konkreten Oberweltzelle
- `LocalMapCoord`: Koordinate innerhalb einer lokalen Detailkarte, gebunden an
  eine uebergeordnete `RegionId`; die zugehoerige Weltkoordinate kann zusaetzlich
  angezeigt oder abgeleitet werden
- `InstanceCoord`: eigenstaendige Koordinate innerhalb einer `Instance`, etwa
  fuer einen Dungeon, gebunden an deren `InstanceId`

Eine Weltkoordinate bezeichnet ausschliesslich eine konkrete Oberweltzelle
und nicht gleichzeitig eine Region. Regionen werden ueber ihre typisierte
`RegionId` und ihren aus Weltzellen bestehenden Bereich adressiert. Die
Regionsrasterkoordinate kann aus einer Weltzelle und der festen Regionsgroesse
abgeleitet werden. Chunks folgen demselben Prinzip und besitzen eine eigene
`ChunkId`.

Fuer die erste kleine Welt umfasst eine Region `16 x 16` Weltzellen. Ein
Chunk umfasst `4 x 4` Weltzellen, sodass eine Region vier mal vier Chunks
enthaelt. Regionen und ihre Metadaten werden bei der Welterzeugung angelegt.
Konkrete lokale Detailkarten werden dagegen erst bei Bedarf deterministisch
erzeugt und anschliessend persistiert, sofern ihr Zustand relevant geworden
ist.

Die Grundnachbarschaft umfasst die acht direkt oder diagonal angrenzenden
Zellen beziehungsweise Bereiche. An Weltgrenzen werden Nachbarn nicht
umgebrochen; ausserhalb der rechteckigen Welt existiert keine Koordinate.

### Zeit, Kalender und Zeitskalen
### Einheiten und Mengen

Mengen werden intern als nichtnegative Ganzzahlen in ihrer kleinsten
fachlichen Basiseinheit gespeichert. Bruchteile sind nicht zulaessig. Ein
Ueberlauf oder eine ungueltige Einheitenoperation wird abgelehnt. Physische
Bestaende duerfen nicht negativ werden; Schulden werden spaeter als eigene
fachliche Verpflichtung und nicht als negativer Geldbestand modelliert.

Fuer den ersten Kernumfang gelten folgende Mengen:

- `Mass`: Gramm als kleinste Basiseinheit
- `Count`: ganzzahlige Anzahl von Stuecken, Individuen oder Einheiten
- `Distance`: ganzzahlige Welt-, lokale oder Instanzzellen
- `FoodEnergy`: abstrakte ganzzahlige Energieeinheiten
- typisierte Ressourcenmengen: getrennte ganzzahlige Mengen je
  Ressourcendefinition

Nahrungsmittel werden nicht zu einem gemeinsamen Bestand zusammengefasst.
Konkrete Nahrungsdefinitionen wie Fleisch, Brot oder Beeren bleiben getrennte
typisierte Mengen. Eine Nahrungsdefinition besitzt einen abstrakten
ganzzahligen Energiegehalt und datengetriebene Vertraeglichkeitsregeln. Eine
Spezies oder Entitaet kann dadurch beispielsweise pflanzliche Nahrung
vertragen, Fleisch aber ablehnen. Beim Verbrauch wird nur kompatible Nahrung
auf den Energiebedarf angerechnet.

Zubereitungszustaende, Verarbeitung, Verderb, Haltbarkeit und Qualitaetsregeln
sind kein Bestandteil dieses ersten Einheitenmodells. Sie koennen spaeter als
optionale datengetriebene Eigenschaften und Prozesse ergaenzt werden, falls
dafuer ein konkreter fachlicher Bedarf entsteht.

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
