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

Wenn in der Projektdokumentation von einem `Event` oder einem Ereignis die
Rede ist, ist damit grundsaetzlich ein historisches, fachliches Ereignis der
Welt gemeint. Technische Vorgaenge wie Ticks, Command-Eingang,
Systemausfuehrungen oder Debug-Meldungen sind keine Events im fachlichen Sinn
und werden ausdruecklich als technische Metadaten beziehungsweise
Protokolleintraege bezeichnet.
eintraege bezeichnet.

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

### Commands, validierte Zustandsaenderungen und Events

Ein Command beschreibt ausschliesslich eine Absicht. Er enthaelt die
angeforderte Aktion und ihren Eingabekontext, veraendert den Weltzustand aber
nicht direkt. Die ersten Commands der Kernsimulation sind:

- `InitializeRacePopulation`: die initialen Rassengruppen nach der
  geografischen Welterzeugung auf der Welt verteilen
- `AdvanceSimulation`: einen oder mehrere Ticks beziehungsweise bis zu einer
  Zielzeit fortschreiten
- `PauseSimulation`: die Verarbeitung weiterer Fortschritts-Commands pausieren
- `StepSimulation`: genau einen Tick verarbeiten und danach pausiert bleiben

Die fachliche Validierung wird nicht ausschliesslich im Orchestrator gebuendelt.
Entitaeten und fachliche Teilbereiche stellen geeignete Validatoren fuer ihre
eigenen Regeln bereit. Der Orchestrator prueft zusaetzlich den Command-Kontext,
ordnet die Validatoren und Systeme in definierter Reihenfolge an, stellt die
atomare Anwendung sicher und sammelt die resultierenden Events. Ein Command ist
nur dann erfolgreich, wenn alle relevanten Validierungen bestanden wurden.

Eine validierte Zustandsaenderung ist das Ergebnis eines erfolgreich geprueften
Commands und wird innerhalb des kontrollierten Orchestrator-Schritts auf den
Weltzustand angewendet. Sie ist keine historische Tatsache, solange sie nicht
erfolgreich angewendet wurde. Eine erfolgreiche Zustandsaenderung erzeugt ein
fachliches Event, wenn sie historisch relevant ist; rein technische oder
wirkungslos angenommene Commands erzeugen kein historisches Event.

Ein Event ist ein festgeschriebenes, vergangenes fachliches Ereignis und Teil
der append-only Weltgeschichte. Es wird nach seiner Annahme nicht inhaltlich
geaendert. Der erste historische Eventtyp der initialen Welterzeugung ist
`InitialRaceSpawn`. Weitere fruehe historische Eventtypen sind `CreatureSlain`,
`KingdomFounded` und `HistoricalFigureBorn`. Ihre zusaetzlichen Pflichtdaten
sind mindestens:

- `InitialRaceSpawn`: Rasse, Gruppengroesse, initiale Weltposition und
  Erzeugungskontext

- `CreatureSlain`: getoetete Kreatur und Ort des Todes
- `KingdomFounded`: gegruendetes Koenigreich und Gruendungsort
- `HistoricalFigureBorn`: geborene historische Figur und Geburtsort

`InitialRaceSpawn` ist ein wiederholbarer Eventtyp und tritt mindestens einmal
pro Rasse auf. Die Rassen werden in Gruppen von jeweils 100 Individuen auf die
Welt gesetzt. Die einzelnen Event-Instanzen erhalten jeweils eine eigene
`EventId`; spaetere Siedlungs- und Zivilisationsgruendungen koennen auf diese
Ereignisse und ihre betroffenen Entitaeten zurueckverweisen.

`InitialRaceSpawn` entsteht nicht als impliziter Nebeneffekt der reinen
Geografieerzeugung. Nach erfolgreicher geografischer Welterzeugung wird dafuer
`InitializeRacePopulation` verarbeitet. Der Command wird validiert und atomar
angewendet; erst danach werden die Rassengruppen als Weltzustand angelegt und
die zugehoerigen historischen Events in deterministischer Reihenfolge erzeugt.

Jedes historische Event enthaelt mindestens eine eigene `EventId`, den
Eventtyp, einen exakten Zeitpunkt der Weltzeit, die Ursache, die betroffenen
Entitaeten und die fuer den Eventtyp definierten Ereignisdaten. Die Ursache unterscheidet mindestens
Weltgenerierung, autonome Simulation, Spieler, System beziehungsweise
Orchestrator und Mod.

Ein Simulations-Tick ist kein historisches Event und wird nicht als allgemeines
Pflichtfeld des Event-Inhalts behandelt. Ein historisches Event kann optional
den erzeugenden Tick als technische Metadaten referenzieren, damit Debugging
und Replay den Ausfuehrungskontext nachvollziehen koennen. Ein Tick kann
mehrere historische Events erzeugen oder kein historisches Event erzeugen.

Abgelehnte Commands veraendern den Weltzustand nicht und erzeugen kein
fachliches Event. Sie werden ausschliesslich im Debug-Protokoll mit ihrem
Fehler erfasst; im normalen historischen Event-Log erscheinen sie nicht.

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

Die kanonische Weltzeit wird als nichtnegative Anzahl von Sekunden seit der
Welterzeugung gefuehrt. Der Startzeitpunkt ist Tag 1, Monat 1, Jahr 1 bei
Weltzeit `0`. Ein Tag hat 24 Stunden, eine Stunde 60 Minuten und eine Minute
60 Sekunden.

Der Kalender umfasst zehn normale Monate mit jeweils 20 Tagen sowie einen
elften Sondermonat mit drei Tagen. Ein Jahr hat damit 203 Tage oder
`17.539.200` Sekunden. Der Sondermonat ist als eigener Kalenderabschnitt
erkennbar und kann spaeter besondere Gefahren, Spawnregeln und Events
aktivieren. Diese Regeln gehoeren nicht zur grundlegenden Zeitrepraesentation.

Ein Simulations-Tick ist ein expliziter, kontrollierter Fortschrittsschritt,
in dem die zustaendigen Systeme in definierter Reihenfolge ausgefuehrt werden.
Ein Tick ist kein zusaetzlicher Zeittyp; er besitzt eine Dauer und verschiebt
die gemeinsame Weltzeit um diesen Betrag. Die Tick-Dauer darf vom
Simulationskontext abhaengen: Auf Koenigreichsebene kann ein Tick einen Tag,
auf Siedlungsebene eine Stunde und im Roguelike-Modus die Dauer einer
konkreten Aktion repraesentieren. Unterschiedliche Detailstufen verwenden
damit unterschiedlich grosse Fortschrittsschritte, erzeugen aber keine eigene
Zeitlogik und keine parallele Weltzeit.

Im Siedlungsmodus entsprechen zehn Echtzeitsekunden einer Ingame-Stunde. Ein
Siedlungstag dauert damit 240 Echtzeitsekunden. Im Koenigreichmodus entsprechen
zehn Echtzeitsekunden einem Ingame-Tag. Ein normaler Monat dauert 200
Echtzeitsekunden, der Sondermonat 30 Echtzeitsekunden und ein Jahr 2030
Echtzeitsekunden.

Im Roguelike-Modus sind Ticks aktionsbasiert. Die Bewegung von einem Tile zum
naechsten, ein Angriff und eine Interaktion wie das Faellen eines Baumes haben
jeweils eine eigene, aus Aktion, Geschwindigkeit, Zustand und Umgebung
deterministisch berechnete Dauer. Eine Aktion kann dadurch Sekunden oder
mehrere Minuten Weltzeit verbrauchen. Schnellreisen werden als Folge normaler
Reise- oder Bewegungsaktionen simuliert und nicht als unkontrollierter direkter
Zeitsprung behandelt.

Echtzeit in einem Spielmodus beschreibt die Bedienung und Darstellung, nicht
eine Kopplung an die Render-Framerate. Die Simulation bleibt explizit
tickbasiert und kann pausiert, beschleunigt oder im Einzelschritt ausgefuehrt
werden. Als Geschwindigkeiten sind mindestens `0.25x`, `0.5x`, `1x`, `2x`,
`10x`, `100x` und `maximum` vorgesehen. Bei einer Ueberlastung wird die
Simulation langsamer; es werden keine Ticks uebersprungen.

Ein Fortschrittsbefehl kann einen einzelnen Tick, mehrere Ticks oder die
Simulation bis zu einer Zielzeit anfordern. Jeder Tick wird vollstaendig
abgeschlossen, bevor der naechste beginnt. Eine Pause und ein Moduswechsel
werden an einer Tick-Grenze wirksam. Ein Einzelschritt verarbeitet genau einen
Tick und laesst die Simulation anschliessend pausiert. Commands waehrend einer
Pause werden angenommen und in stabiler Reihenfolge vorgemerkt.

Ein Tick beginnt bei einer bestimmten Weltzeit und endet bei der Weltzeit nach
seinem Fortschritt. Zustandsaenderungen werden innerhalb dieses kontrollierten
Schritts angewendet. Events erhalten neben ihrem Tick-Kontext einen exakten
Zeitpunkt in der Weltzeit; eine stabile Reihenfolge fuer Events mit gleicher
Weltzeit wird spaeter als Teil des Orchestrators festgelegt. Zeit wird je nach
Kontext in Jahren, Monaten, Tagen, Stunden, Minuten und Sekunden dargestellt.
#### Phase-0-Spezifikation der Tick-Fortschritts- und Grenzfalltests

Die Tests decken im ersten Umsetzungsschritt den Minimal-Kern und die
Welterzeugung mit einem einzigen Simulationskontext ab. Dieser Kontext
verwendet einen Tick von genau einem Tag. Spaetere Simulationskontexte erhalten
eigene Tests und koennen fachlich andere Tick-Dauern verwenden; es gibt daher
keine dauerhafte einheitliche Tick-Dauer fuer alle Simulationsebenen.

Die Fortschrittstests decken alle vorgesehenen Operationen ab:

- normaler Fortschritt um einen Tick
- Fortschritt um mehrere Ticks
- Fortschritt bis zu einer Zielzeit
- Pause
- Fortsetzen
- Einzelschritt
- Beschleunigung und Verlangsamung ueber die vorgesehenen Geschwindigkeiten
- Fortschritt im ersten Phase-1-Simulationskontext mit einem Tages-Tick

Die Grenzfalltests decken mindestens folgende Situationen ab:

- Weltzeit und Fortschritt vor dem ersten Tick
- Tick `0`
- ein einzelner Tick
- mehrere Ticks
- sehr grosse Schrittanzahlen
- die groesste darstellbare Weltzeit
- Zeit- und Tick-Ueberlauf
- negative oder anderweitig ungueltige Fortschrittswerte
- Fortschritt waehrend einer Pause
- Einzelschritt waehrend einer Pause
- Fortsetzen nach einer Pause
- wiederholtes Pausieren und Fortsetzen
- Fortschritt bis zu einer bereits erreichten Zielzeit
- Fortschritt bis zu einer ungueltigen oder nicht erreichbaren Zielzeit
- Wechsel des Simulationskontexts an einer Tick-Grenze

Ein ungueltiger Fortschritts-Command wird mit einem stabilen Fehler abgelehnt.
Die gesamte angeforderte Zustandsaenderung wird atomar zurueckgerollt; Weltzeit,
Weltzustand und historische Events bleiben unveraendert. Ein Fortschritt wird
nicht teilweise ausgefuehrt. Nach einer Ablehnung darf ein spaeterer,
unabhaengig gueltiger Command die Simulation fortsetzen. Die Ablehnung wird
mit ihrem Fehlercode im Debug-Protokoll erfasst, erzeugt aber kein historisches
Event.

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

Definitionen beschreiben allgemeine fachliche Typen und Regeln und sind keine
konkreten Weltentitaeten. Beispiele fuer die erste Phase sind Biome,
Ressourcenarten, Spezies und Ortstypen. Sie werden ueber versionierte
Datenquellen beziehungsweise das `core-mod` geladen.

Instanzen sind konkrete Auspraegungen dieser Definitionen in einer bestimmten
Welt. Dazu gehoeren beispielsweise eine konkrete Welt, Region,
Ressourcenlagerstaette, Siedlung oder ein Point of Interest. Instanzen besitzen
stabile IDs und werden im Save gespeichert, sofern sie fuer den Weltzustand,
die Geschichte oder eine spaetere Referenz relevant sind.

Der Laufzeitstatus umfasst alle veraenderlichen Werte und den aktuellen
Simulationsfortschritt. Dazu gehoeren insbesondere die aktuelle Weltzeit,
aktuelle Mengen, aktive Prozesse, Lebenszyklusstatus und laufende
Beziehungen. Der Laufzeitstatus wird getrennt von Definitionen und der
unveraenderlichen Identitaet einer Instanz behandelt und im Save gespeichert.

Unveraenderte Details, die aus Seed, Weltkonfiguration und Erzeugungskontext
deterministisch rekonstruiert werden koennen, muessen nicht dauerhaft
gespeichert werden. Sobald solche Details relevant veraendert oder historisch
referenziert wurden, ist der persistierte Zustand massgeblich.

Die Trennung gilt zunaechst als fachlicher Datenvertrag und wird bei der
Implementierung des Minimal-Kerns in Rust-Strukturen abgebildet. Definitionen
werden dabei nicht als Instanzen dupliziert; Instanzen referenzieren die
zugehoerigen Definitionen.

### Versionierung und Migration

Jeder Save besitzt eine explizite technische Formatversion im
`manifest.json`. Alte Saves werden beim Laden grundsaetzlich automatisch auf
die aktuelle unterstuetzte Version migriert. Migrationen funktionieren nur
vorwaerts, also von einer aelteren auf eine neuere Formatversion.

Eine Migration veraendert den urspruenglichen Save nicht. Sie erstellt eine
neue Save-Kopie und legt vor der Migration zusaetzlich eine Sicherung des
urspruenglichen Saves an. Der urspruengliche Save bleibt dadurch als
Rueckfalloption erhalten.

Fehlen Daten, entscheidet die jeweilige Datenart ueber das Vorgehen. Fuer
fachlich eindeutig ableitbare oder optionale Werte duerfen definierte
Defaults verwendet werden. Fehlen Pflichtdaten ohne sicheren Default, wird
die Migration mit einem verstaendlichen Fehler abgebrochen.

Unbekannte zusaetzliche Felder werden beim Laden als Fehler abgelehnt. Dadurch
werden Tippfehler und nicht unterstuetzte Daten nicht stillschweigend ignoriert
und koennen die Reproduzierbarkeit nicht unbemerkt veraendern.

Als inkompatibel gelten insbesondere geaenderte Datentypen ohne eindeutige
Umwandlung, entfernte Pflichtfelder ohne sinnvollen Ersatz, geaenderte ID- oder
Referenzvertraege, nicht unterstuetzte Save-Formatversionen sowie fehlende
oder inkompatible Mod-Daten. In solchen Faellen wird ein verstaendlicher
Fehler ausgegeben. Sofern der Zustand noch sicher lesbar ist, darf der Save
zusaetzlich schreibgeschuetzt geladen werden; eine Simulation oder ein
Ueberschreiben ist dann nicht erlaubt.

Save-Migrationen behandeln das technische Save-Format und die gespeicherte
Datenstruktur. Mod-Versionen und deren Datenvertraege werden separat geprueft
und nicht durch eine allgemeine Save-Migration repariert. Nicht mehr direkt
unterstuetzte Saves sollen spaeter ueber ein separates Import- oder
Upgrade-Werkzeug migriert werden koennen.

## 2. Datengetriebene Inhalte

### Datenquellen und Dateiformate

Das erste Datenformat ist JSON. Es wird wegen seiner Menschenlesbarkeit fuer
Debug-Daten, Testdaten und die ersten Save-Prototypen verwendet. Die
Serialisierung und Deserialisierung erfolgt ueber `serde`, damit die fachlichen
Rust-Strukturen nicht an ein einzelnes Dateiformat gebunden werden.

Der Datenvertrag wird so gestaltet, dass spaeter ein binaeres und
komprimierbares Format ergaenzt oder fuer Produktions-Saves verwendet werden
kann. Die Wahl des Speicherformats darf daher nicht Teil der fachlichen
Zustandsmodelle oder der Simulationslogik werden. Formatversionen werden
explizit mitgefuehrt.

Geladene Daten werden gegen ein definiertes Schema validiert. Unbekannte
Felder, fehlende Pflichtfelder, ungueltige Typen, ungueltige Werte und
inkompatible Formatversionen werden als Ladefehler behandelt.

Ganzzahlen, Seeds und typisierte IDs muessen in JSON und in spaeteren
Alternativformaten ohne Praezisionsverlust abgebildet werden. Debug- und
Testdaten werden nicht als eigene versionierte Dateien im Repository
vorgegeben; ihre Ablage und Erzeugung bleibt Teil der jeweiligen Test- oder
Werkzeugstruktur.

### Schema, Defaults und Validierung

#### Phase-0-Spezifikation ungueltiger Konfiguration und Daten

Der Validierungstest umfasst alle extern geladenen oder persistierten Daten:

- `WorldConfig` und `WorldSeed`
- Weltgroesse und Ausdehnung
- Generator-Version
- Mod-IDs, Mod-Versionen und Datenvertraege
- Definitionen
- Save-Struktur und Formatversion
- Runtime-ID-Woerterbuch
- serialisierte Commands und Events, soweit sie geladen werden

Der Test behandelt mindestens fehlende Pflichtfelder, leere oder formal
ungueltige IDs, ungueltige oder inkompatible Versionen, widerspruechliche
Werte, ungueltige Wertebereiche, fehlende Mod-Abhaengigkeiten, doppelte IDs,
ungueltige Referenzen, ungueltige Seeds sowie beschaedigte oder unvollstaendige
Daten. Unbekannte Felder werden strikt abgelehnt.

Eine ungueltige Konfiguration oder ein ungueltiger Datensatz wird vollstaendig
abgelehnt. Vor der Welterzeugung wird keine Welt erstellt. Bei einem bereits
bestehenden Weltzustand bleibt dieser durch atomaren Rollback unveraendert;
Weltzeit, Weltzustand und historische Events werden nicht teilweise geaendert.
Die Ablehnung verwendet einen stabilen Fehlercode und benennt den betroffenen
Datentyp sowie, sofern vorhanden, den Feldpfad. Sie erzeugt kein historisches
Event.

Die Validierung erfolgt vor Welterzeugung und Simulation. Aeltere
unterstuetzte Versionen werden nur ueber ausdruecklich definierte,
deterministische Migrationen akzeptiert. Unbekannte oder inkompatible
Versionen werden abgelehnt. Eine Migration aendert die Quelldaten nicht,
sondern erzeugt eine neue validierte Version mit Sicherung der Quelle.

### Namensgeneratoren und Sprachdaten
### Tags, Kategorien und Abhaengigkeiten
### Modding und Erweiterungspunkte
### Balancing und Konfigurationsprofile

### Core-Mod und generische Engine-Vertraege

## 3. Weltgenerierung

### Seed- und Zufallsstrategie

Der Welt-Seed ist Bestandteil der Weltkonfiguration und wird intern als
`u64` gespeichert. Jeder `u64`-Wert ist gueltig, einschliesslich `0`.

Eine Welt kann entweder mit einem numerischen Seed oder mit einem frei
eingegebenen Text-Seed erzeugt werden. Text-Seeds werden deterministisch und
plattformunabhaengig in einen `u64`-Wert umgewandelt. Der eingegebene Text wird
zusaetzlich unveraendert in der Weltkonfiguration gespeichert, damit er in
Saves, Logs und Debug-Ausgaben nachvollziehbar und teilbar bleibt. Der
numerische Seed wird ebenfalls immer gespeichert und angezeigt; bei einem
Text-Seed wird der Originaltext zusaetzlich angezeigt.

Wird kein Seed angegeben, erzeugt das System einen automatisch bestimmten
`u64`-Seed. Automatisch erzeugte Seeds besitzen keinen Text-Seed und werden
ausschliesslich numerisch gespeichert und angezeigt.

Eine Aenderung des Seeds erzeugt eine neue Welt. Die Kombination aus
Weltkonfiguration und numerischem Seed muss ausreichen, um die Welterzeugung
reproduzierbar zu wiederholen. Die verwendete Umwandlung von Text in `u64`
muss deshalb als stabiler, versionierter und plattformunabhaengiger Vertrag
behandelt werden.

Zufallsentscheidungen verwenden benannte und voneinander unabhaengige
RNG-Stroeme. Fuer den ersten Kernumfang werden die fachlichen Stroeme
`system:rng:worldgen`, `system:rng:weather`, `system:rng:population`,
`system:rng:events`, `system:rng:loot` und `system:rng:names` vorgesehen.
Jeder Stream besitzt genau eine fachliche Verantwortlichkeit. Ein zusaetzlicher
Zufallsaufruf in einem Stream darf die Ergebnisse anderer Streams nicht
veraendern.

Der Namespace `system` ist fuer engineinterne Zufallsentscheidungen reserviert.
Mods duerfen keine `system:*`-ID definieren, ueberschreiben oder referenzieren.
Fachliche Zufallsstroeme des Grundspiels verwenden den Namespace `core`, zum
Beispiel `core:rng:worldgen`; Stroeme anderer Mods verwenden deren eigenen
Namespace. Die Mod-Aufloesung weist unzulaessige Verwendung des reservierten
`system`-Namespaces zurueck.

Streams werden deterministisch aus Welt-Seed, vollstaendiger Stream-ID und
einer versionierten RNG-Algorithmuskennung abgeleitet. Unterstreams duerfen
spaeter aus einer Stream-ID und einem stabilen Kontext wie Region-ID,
Entity-ID oder Event-ID abgeleitet werden. Unbekannte Streams werden nicht
implizit erzeugt, sondern als Fehler behandelt.

Fuer die erste Version wird kein globaler RNG-Zustand persistiert. Gleicher
Seed, gleiche Konfiguration, gleiche Eingaben und gleiche Eventreihenfolge
muessen dieselben Zufallsentscheidungen erzeugen. Die verwendeten Stream-IDs
und die RNG-Algorithmusversion werden im Welt- beziehungsweise Save-Kontext
nachvollziehbar festgehalten. Wichtige Zufallsentscheidungen sollen mit
Stream-ID und Kontext debugbar sein, ohne jeden einzelnen Zufallswert zu
protokollieren.
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

Commands werden am Orchestrator entgegengenommen und in stabiler Reihenfolge
verarbeitet. Die Verarbeitung eines Commands folgt im ersten Orchestrator
dieser Reihenfolge:

1. Command aus der Eingangsqueue nehmen
2. Command-Kontext pruefen
3. Validatoren der betroffenen Entitaeten und Fachbereiche aufrufen
4. Zustandsaenderungen vorbereiten
5. Zustandsaenderungen atomar anwenden
6. daraus entstandene historische Events erzeugen
7. historische Events in stabiler Reihenfolge an das Event-Log uebergeben
8. technische Debug- und Ausfuehrungsdaten protokollieren

Fachliche Validatoren pruefen ihre jeweiligen Invarianten; der Orchestrator
prueft den gemeinsamen Kontext, koordiniert die Reihenfolge und verwirft den
gesamten Zustandswechsel, falls eine Validierung fehlschlaegt. Die
Fehlerdarstellung verwendet stabile fachliche Fehlercodes. Abgelehnte Commands
werden nur fuer Debugging protokolliert und nicht als historische Events
gespeichert. Ein formal gueltiger, aber wirkungsloser Command, etwa das
Pausieren einer bereits pausierten Simulation, aendert den Zustand nicht und
erzeugt ebenfalls kein historisches Event.

### Events, Eventtypen und Event-Log

Das Event-Log enthaelt ausschliesslich erfolgreich angewendete, unveraenderliche
fachliche Ereignisse. Der Orchestrator vergibt Event-IDs, stellt Weltzeit,
Ursache und betroffene Entitaeten sicher und fuehrt Events in einer
deterministischen Reihenfolge. Der erste historische Eventtyp der initialen
Welterzeugung ist `InitialRaceSpawn`; spaetere fruehe Eventtypen sind
`CreatureSlain`, `KingdomFounded` und `HistoricalFigureBorn`. Ihre
ereignisspezifischen Daten werden gemeinsam mit dem jeweiligen Eventtyp
definiert. Ein reiner Command-Eingang, ein Tick oder eine Ablehnung ist kein
historisches Event.
### Kerninvarianten des Phase-0-Kerns

Die folgenden Invarianten begrenzen den ersten testbaren Kernumfang. Sie gelten
unabhaengig von spaeteren Regeln fuer Population, Ressourcen, Biome oder Kampf:

- Eine `WorldConfig` enthaelt mindestens einen gueltigen `WorldSeed`.
- Ein `WorldSeed` bleibt fuer die Lebensdauer einer Welt unveraendert.
- Ein `Tick` ist gueltig darstellbar und kann nicht unbeabsichtigt rueckwaerts
  laufen.
- Ein normaler Tick-Fortschritt erhoeht die Simulationszeit genau einmal.
- Eine pausierte Simulation veraendert ihren Weltzustand nicht.
- Ein Einzelschritt verarbeitet genau einen kontrollierten Tick.
- Typisierte IDs sind nur in ihrem vorgesehenen fachlichen Kontext verwendbar.
- IDs sind innerhalb ihres jeweiligen Gueltigkeitsbereichs eindeutig.
- Events besitzen immer eine eindeutige Event-ID und eine gueltige Weltzeit.
  Ein optional gespeicherter Tick-Kontext muss gueltig sein.
- Events enthalten Ursache und betroffene Entitaeten, sofern fachlich
  erforderlich.
- Referenzen auf Entitaeten und Beziehungen zeigen nur auf gueltige oder
  ausdruecklich entfernte Objekte.
- Ein ungueltiger Command veraendert den Weltzustand nicht.
- Bei gleichem Seed, gleicher Konfiguration und gleichen Eingaben entstehen
  derselbe Zustand und dieselben Events.

### Event-Abonnenten und Reaktionen
### Determinismus und Replay

Determinismus wird fachlich beziehungsweise semantisch bewertet. Bei gleicher
Weltkonfiguration, gleichem Seed, gleichen Mod-Versionen, gleichen Commands,
gleichen Eingabezeitpunkten und gleicher Eingabereihenfolge muessen mindestens
der fachlich relevante Weltzustand, stabile IDs und historische Events
identisch sein. Command- und Event-Reihenfolgen sind strikt deterministisch.

Gleicher Seed und gleicher RNG-Kontext muessen plattformuebergreifend dieselben
Zufallsentscheidungen liefern. Systemzeit, externe Zufallsquellen und
unkontrollierte Thread- oder Ausfuehrungsreihenfolgen sind innerhalb der
Simulation nicht zulaessig. Abgelehnte Commands und Fehler muessen mit
demselben fachlichen Fehlercode und in derselben Reihenfolge reproduzierbar
sein.

#### Phase-0-Spezifikation des Determinismus-Tests

Der Determinismus-Test deckt den vollstaendigen fuer den jeweiligen Lauf
relevanten Umfang ab: Minimal-Kern, Welterzeugung und die vorgesehenen
Simulationskontexte. In zwei identischen Testlaeufen bleiben alle folgenden
Eingaben unveraendert:

- vollstaendige Weltkonfiguration
- Welt-Seed
- Mod-Versionen und Datenvertraege
- Commands
- Eingabezeitpunkte
- Eingabereihenfolge

Die Ergebnisse werden semantisch verglichen. Der Vergleich umfasst:

- den fachlich relevanten Weltzustand
- stabile IDs
- historische Events
- Event-Reihenfolge
- abgelehnte Commands und Fehlercodes
- relevante Debug-Informationen zu Zufallsentscheidungen

Der Test wird in drei Umfaengen spezifiziert:

1. Welterzeugung ohne anschliessenden Tick-Fortschritt
2. Welterzeugung und anschliessend 100 Ticks
3. Welterzeugung und anschliessend 10.000 Ticks

Jeder Umfang muss bei identischen Eingaben semantisch identische Ergebnisse
erzeugen.

Ein Determinismus-Test zaehlt Fortschrittsschritte des jeweiligen
Simulationskontexts und nicht eine globale, einheitliche Tick-Dauer. Ein
Siedlungsschritt entspricht einer Ingame-Stunde, ein Koenigreichsschritt einem
Ingame-Tag und ein Roguelike-Schritt einer abgeschlossenen Aktion. Die Dauer
eines Roguelike-Schritts ist aktionsabhaengig und kann beispielsweise bei
Bewegung, Angriff oder Baumfaellen unterschiedlich ausfallen. Nach dem Test
muss die Weltzeit der Summe aller verarbeiteten Schritt- beziehungsweise
Aktionsdauern entsprechen.

Der erste Testumfang umfasst mindestens 100 Fortschrittsschritte je Kontext
sowie einen Langzeittest mit 10.000 Fortschrittsschritten. Zusaetzlich wird ein
Save nach einer definierten Schrittzahl geladen und mit einem ununterbrochenen
Lauf verglichen. Groessere und kleinere Schrittweiten duerfen nur dann als
fachlich gleichwertig gelten, wenn im groesseren Schritt keine relevanten
Zwischenereignisse oder historischen Zustandsaenderungen verloren gehen.
Zeitraeume mit solchen Zwischenereignissen duerfen nicht unzulaessig
zusammengefasst werden.

Unterschiedliche Seeds werden mit identischer Weltkonfiguration verglichen.
Nur der Seed darf sich zwischen den Testlaeufen aendern. Jeder unterschiedliche
Seed muss nicht zwingend ein einzigartiges Ergebnis erzeugen; bei ausreichender
Stichprobe muessen jedoch fachlich unterscheidbare Ergebnisse entstehen.

Der Test verwendet mindestens die numerischen Seeds `0`, `1`, `885372` und
`u64::MAX` sowie die Text-Seeds `world of shelist` und `the power of rng`.
Getestet werden die Welterzeugung, 100 Fortschrittsschritte und 1.000
Fortschrittsschritte in den Kontexten Weltgenerierung, Siedlung, Koenigreich
und Roguelike. Verglichen werden alle fachlich relevanten Ergebnisse,
insbesondere Weltzustand, stabile IDs, historische Events und relevante
Zufallsauswirkungen.

Jeder Testlauf muss trotz Seed-Unterschieden alle Welt-, ID- und Event-
Invarianten einhalten. Festgestellte Unterschiede werden mit Seed,
Simulationskontext, Testumfang, betroffenem Weltzustand und relevanten Events
dokumentiert.
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

#### Phase-0-Spezifikation der Debug-Ausgabe

Die Debug-Ausgabe wird sowohl als menschenlesbarer Text als auch als
strukturiertes JSON definiert. Beide Darstellungen verwenden dieselben
fachlichen Daten und dieselbe deterministische Reihenfolge. Die Ausgabe kann
an mehrere Ziele geleitet werden: mindestens Konsole und Datei. Die
Konsolenausgabe ist unabhaengig von der Dateiausgabe abschaltbar.

Die Ausgabemenge wird ueber ein konfigurierbares Mindest-Loglevel gesteuert.
Die Level sind `error`, `warn`, `info`, `debug` und `trace`. Ein Eintrag wird
ausgegeben, wenn sein Level mindestens so hoch ist wie das fuer sein Ziel
konfigurierte Mindestlevel. Konsole und Datei koennen jeweils eigene
Mindestlevel verwenden. Der Default ist `info` fuer beide Ziele; die Konsole
kann zusaetzlich unabhaengig deaktiviert werden. Dadurch sind Fehler und
Warnungen ohne weitere Konfiguration sichtbar, waehrend technische Debug- und
Trace-Daten standardmaessig ausgeblendet bleiben.

Ein Debug-Eintrag enthaelt nur stabile und fuer die Reproduktion relevante
Daten. Systemzeit, zufaellige technische IDs und andere nichtdeterministische
Metadaten werden nicht als Vergleichsdaten verwendet. Die Feldnamen und die
Reihenfolge der Eintraege bleiben fuer semantische Vergleiche stabil.

Ein Eintrag zur Welterzeugung oder zu einer Zufallsentscheidung enthaelt alle
relevanten Seed-Daten, insbesondere den Welt-Seed, seine Eingabeform, die
normalisierte Darstellung, die RNG-Algorithmusversion sowie verwendeten
RNG-Stream und RNG-Kontext, sofern diese fuer die Entscheidung relevant sind.

Ein Tick-Eintrag enthaelt alle relevanten Tick-Daten: Simulationskontext,
Tick-Nummer, Weltzeit vor und nach dem Tick, Tick-Dauer, verarbeitete Commands,
Eingabezeitpunkte und Eingabereihenfolge sowie die ausgefuehrte
Systemreihenfolge, soweit diese Daten vorhanden sind.

Ein Event-Eintrag enthaelt alle relevanten Event-Daten: Event-ID, Eventtyp,
Tick-Kontext, exakten Zeitpunkt, Ursache, betroffene Entitaeten,
ereignisspezifische Daten und die stabile Reihenfolge innerhalb des Ticks.

Normale Tick-Eintraege werden nur ausgegeben, wenn der Tick mindestens ein
historisches Event erzeugt. Fehler und abgelehnte Commands werden unabhaengig
von erzeugten Events sofort ausgegeben. Sie enthalten mindestens Fehlercode,
Fehlerart, betroffenen Datentyp oder Command, Feldpfad beziehungsweise
Entitaetsreferenz sowie Seed- und Tick-Kontext, sofern vorhanden.

### LOD, Culling und Streaming
### Animationen, Partikel und Effekte
### Audio und Musik
### Eingabe, Bindings und Accessibility

## 15. Persistenz und Werkzeuge

### Save-Struktur

Ein Save ist ein eigener Ordner und keine einzelne Datei. Die erste
Darstellung ist menschenlesbar und verwendet JSON. Das Save-Layout wird von
der konkreten Serialisierung getrennt, damit spaeter binaere und
komprimierbare Speicherformate verwendet werden koennen, ohne die fachliche
Aufteilung des Weltzustands zu aendern.

Im obersten Save-Ordner liegt ein `manifest.json`. Das Manifest beschreibt
mindestens die Save-Formatversion, die Weltidentitaet, den Seed, die aktuelle
Zeit, verwendete Versionen und die enthaltenen Datenbereiche. Es erlaubt
Werkzeugen, die Dateien eines Saves zu erkennen, ohne die gesamte Welt laden
zu muessen.

Die Daten werden nach fachlicher Zustaendigkeit, Groesse, Aenderungshaeufigkeit
und Zugriffsmuster aufgeteilt. Fuer den ersten Entwurf sind unter anderem
folgende Bereiche vorgesehen:

- `world`: Geografie, Regionen und Regionsressourcen einschliesslich
  unterirdischer beziehungsweise anderer geografischer Daten
- `aggregates`: aggregierte Daten wie Bevoelkerungszahlen, regionale
  Ressourcenmengen und Tierbestaende
- `entities`: aktuell instanzierte Entitaeten
- `player`: spielerbezogene Daten, sofern der Spielstand einen Spieler besitzt
- `settlements`: Zustaende und Metadaten von Siedlungen
- `civilisations`: Zustaende und Metadaten von Zivilisationen
- `maps`: fuer Darstellung und lokale Verarbeitung benoetigte Kartendaten von
  Tiles bis zu relevanten Objekten

Diese Liste ist nicht abschliessend. Weitere fachliche Bereiche wie Items,
Fraktionen, Beziehungen, Prozesse, Definitionen oder Geschichte koennen bei
sinnvoller fachlicher oder technischer Abgrenzung eigene Dateien oder
Unterordner erhalten. Die Save-Struktur wird deshalb erweiterbar entworfen;
Punkt 3.E legt keine vollstaendige spaetere Dateiliste fest.

Events werden in einem eigenen `events`-Ordner gespeichert. Die Event-Historie
wird in aufeinanderfolgenden Dateien segmentiert. Die maximale Anzahl von
Events je Datei ist konfigurierbar, zum Beispiel 1.000 oder 10.000. Die
Segmentierung darf weder Reihenfolge noch Vollstaendigkeit der Event-Historie
veraendern.

Kartendaten und Entitaeten werden getrennt gespeichert. Entitaeten werden
nicht in die Kartendateien eingebettet, sondern ueber ihre IDs und
Positionsdaten mit den Karten verknuepft. Alle Save-Bereiche muessen fuer
Werkzeuge und spaetere Suchfunktionen grundsaetzlich durchsuchbar bleiben.
Konkrete Suchkriterien werden je Fachbereich spaeter festgelegt. Fuer
Entitaeten muessen mindestens die Entity-ID und Koordinaten als Suchkriterien
unterstuetzt werden; Kombinationen wie Entity-ID und Koordinate muessen
moeglich sein. Ob dafuer sortierte Daten, Indexdateien oder ein separates
Suchverzeichnis verwendet werden, bleibt eine Implementierungsentscheidung.

### Save-Snapshots und Save-Slots

### Minimaler Serialize-/Deserialize-Test

Fuer Phase 0 wird zunaechst nur der fachliche Umfang eines spaeter in 0.G
umzusetzenden Roundtrip-Tests festgelegt. Der Test verwendet einen minimalen
logischen Save mit `manifest`, Formatversion, Seed, Zeit, einem kleinen
Weltzustand, mindestens einer konkreten Entity und mindestens einem
historischen Event.

Der minimale Test serialisiert diesen Save und deserialisiert ihn wieder. Der
geladene Zustand muss semantisch identisch mit dem Ausgangszustand sein. Der
Vergleich erfolgt nicht anhand byte-identischen JSON-Texts, sondern anhand
seiner fachlichen Werte.

Die Aufteilung in mehrere Save-Dateien und Event-Segmente, Tests fuer
ungueltige Pflichtdaten sowie Tests fuer unbekannte Felder gehoeren nicht zum
minimalen Roundtrip-Test. Sie werden spaeter in den Qualitaets- und
Implementierungstests behandelt. Die konkreten Rust-Strukturen, Speicher- und
Ladefunktionen werden erst in 0.G umgesetzt.

### Event-Log und Replay-Dateien

Jedes fachliche Event ist ein historisches Ereignis und wird dauerhaft im Save
gespeichert. Die Event-Historie ist vollstaendig und append-only. Ein Snapshot,
Checkpoint oder Save-Ladevorgang darf historische Events weder ersetzen noch
loeschen. Event-Dateien duerfen zur Platzersparnis komprimiert oder in ein
Archiv verschoben werden; eine Bereinigung oder Entfernung historischer Events
ist nicht zulaessig.

Runtime-interne Vorgaenge, technische Debug-Meldungen, Warnungen und
abgelehnte Commands sind keine historischen Events. Sie werden getrennt im
Debug- beziehungsweise Diagnoseprotokoll behandelt und gehoeren nicht in die
dauerhafte fachliche Event-Historie.

Gespeicherte fachliche Events enthalten ausreichend Daten, um sowohl Replay
als auch History zu unterstuetzen. Die Segmentierung in mehrere Event-Dateien
darf ihre Reihenfolge, Vollstaendigkeit und eindeutige Zuordnung nicht
veraendern.
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

Die folgenden Fragen sind fuer Phase 1 noch nicht entscheidend und bleiben
bewusst offen:

### Welche Agenten werden voll und welche aggregiert simuliert?
### Ist die Welt endlich, unendlich oder streamingfaehig?
### Wie stark veraendert Magie Natur- und Gesellschaftsregeln?
### Welche Informationen darf der Spieler sehen?
### Welche Regeln muessen modifizierbar sein?
### Welche technischen Grenzen gelten fuer spaetere Plattformen?
