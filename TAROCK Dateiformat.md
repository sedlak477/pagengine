# TAROCK Dateiformat

Eine Tarockdatei ist eine UTF-8 kodierte Textdatei, die TAF Zeichenketten in jeder Zeile beinhält.

## Begriffsdefinitionen

| Begriff | Definition |
| ------- | ---------- |
| Tarockspiel | Ein Tarockspiel startet wenn "Mein Spiel" gesagt wurde und endet mit dem letzen Stich. |
| Tarockrunde | Eine Tarockrunde startet mit dem ersten Tarockspiel und endet nachdem das letzte Tarockspiel gespielt wurde und der Endstand ausgezählt wurde. |
| Spielzug | 

## Tarockaustauschformat (TAF)

Das Tarockaustauschformat (TAF) ist eine Zeichenkette, die den Zustand eines Tarockspiels zur Gänze kodiert.

Sie setzt sich aus fünf Gruppen zusammen die durch Leerzeichen voneinander getrennt sind:

```text
<Karten> <Spiel> <Ansagen> <Kleinen stechen die Großen> <Spritzen>
```

Beispiel für den Ausgangszustand eines Rufers aus Sicht von Spieler:in 1:

1. Talon und Karten von Spieler:in 1 sind bekannt.
2. Spieler:in 1 spielt einen Rufer, ruft den Kreuz König und nimmt den 1. Talon (der mit dem König drinnen).
3. Spieler:in 1 sagt den Pagat und die Könige an. Spieler:in 2 sagt die Trull an.
4. Kleinen stechen die Großen wird nicht gespielt.
5. Spieler:in 2 spritzt Rufer von Spieler:in 1.

```text
XKX10X9/H4H3H2#HkXkT1T3T5T6K1K2K3K4KBKP#X10X9HD/............/#............/#............/# R1XK-1 1K/T// - R12
```

Im Folgenden wird auf die einzelnen Gruppen genauer eingegangen.

### Karten

Die *Karten* Gruppe umfasst den Zustand aller Karten in den Händen der Spieler:innen und auf dem Tisch.  
Sie ist nach folgendem Schema aufgebaut:

```text
<Talon 1>/<Talon 2>#<Hand 1>/<Stiche 1>#<Hand 2>/<Stiche 2>#<Hand 3>/<Stiche 3>#<Hand 4>/<Stiche 4>#<Laufender Stich>
```

- *Talon* 1 & 2 beschreiben die Karten die im oberen Talon (1) und im unteren Talon (2) liegen.
- *Hand* 1, 2, 3 & 4 beschreiben die Karten die Spieler:in 1 - 4 in ihren Händen halten.
- *Stich* 1, 2, 3 & 4 beschreiben die Karten die Spieler:in 1 - 4 in ihren Stichstapeln vor sich liegen haben.
- *Laufender Stich* beschreibt die Karten die in der Mitte des Tisches liegen während eines Stiches.

Talon, Spieler:innen und der laufende Stich werden durch ein `#` Symbol getrennt.
Die Hand und die Stiche der Spieler:innen werden je durch ein `/` Symbol getrennt.
Der obere und untere Talon wird ebenfalls durch ein `/` Symbol getrennt.

#### Einzelne Karten

Eine einzele Karte wird durch ihre Farbe und Wert angegeben.  
Jede Farbe wird durch einen Buchstaben repräsentiert:
| Farbe | Buchstabe |
| --- | --- |
| Herz | H |
| Karo | K |
| Pick | P |
| Kreuz | X |
| Tarock | T |

Bei Karten mit Zahlenwerten wird die Zahl als Wert verwendet.  
Bube, Kavall, Dame, König und Gstieß werden wiefolgt repräsentiert:
| Wert | Buchstabe |
| --- | --- |
| Bube | B |
| Kavall | P |
| Dame | D |
| König | K |
| Gstieß | 22 |

Unbekannte Karten werden durch ein `.` Punkt Symbol notiert.  
Sowohl Groß- als auch Kleinschreibung der Buchstaben ist konform.

Beispiele:

- Herz König: `HK`
- Tarock 14er: `T14`
- Kreuz 9er: `X9`
- Kreuz Kavall: `xk`
- Karo 1er: `k1`
- Unbekannte Karte im Talon: `.`

### Spiel

Die *Spiel* Gruppe umfasst Informationen über das gespielte Spiel, inkl. gerufene Könige falls zutreffend.  
Sie ist nach folgendem Schema aufgebaut:

```text
<Typ><Spieler:in><Gerufener König><Mitspieler:in><Talon>
```

- *Typ* gibt den Spieltypen an (Rufer, Solorufer, Dreier, ...).
- *Spieler:in* gibt die Person oder Personen an die das Spiel spielen. Mögliche Optionen sind 1 - 4 im Bezug auf die Spieler:innen angegeben bei den [Karten](#karten).
- *Gerufener König* gibt den gerufenen König bei Rufer spielen an. Bei Spielen wo kein König gerufen wird, wird `-` notiert.
- *Mitspieler:in* gibt die Person an die den gerufenen König ausgespielt hat. Falls nicht anwendbar oder nicht bekannt, wird `-` notiert.
- *Talon* gibt an ob der obere (1) oder untere (2) Talon genommen wurde. Falls nicht anwendbar wird `-` notiert. Bei Sechserbock wird `12` notiert.

Jedem Spieltyp wird folgende Kurzbezeichung für die Notation zugewiesen:

| Spieltyp | Kurzbezeichnung |
| --- | --- |
| Trischaken | T |
| Rufer | R |
| Sechserbock | S |
| Picc-, Zwicc-, Driccolo | P1, P2, P3 |
| Solorufer | SR |
| Picc-, Zwicc-, Driccolo bei | PB1, PB2, PB3 |
| Bettler | B |
| Besserrufer | BR |
| Picc-, Zwicc-, Driccolo ouvert | PO1, PO2, PO3 |
| Dreier | D |
| Bettler ouvert | BO |
| Pagatdreier | PD |
| Solodreier | SD |
| Solopagatdreier | SPD |

Bei *Picc-, Zwicc-, Driccolo bei* spielen mehrere Personen das Spiel.
Für die Notation werden Spieltyp und Spieler:in für jede spielende Person wiederholt, e.g. Piccolo bei für Spieler:in 1 und Driccolo bei für Spieler:in 2: `PB11PB32`.

Beispiele:

- Spieler:in 2 spielt Dreier und nimmt unteren Talon: `D2--2`
- Spieler:in 4 spielt Rufer mit Pick König und nimmt oberen Talon: `R4pk-1`
- Spieler:in 1 und Spieler:in 4 spielen Piccolo bei, Spieler:in 1 mit 2 Stichen und Spieler:in 4 mit 3 Stichen: `PB21PB34---`
- Spieler:in 4 sagt Trischaken an: `T4---`
- Spieler:in 4 spielt Zwiccolo ouvert: `PO24---`

### Ansagen

Die *Ansagen* Gruppe umfasst Informationen über etwaige Ansagen die gemacht wurden.
Sie umfasst **nicht** gespritzte Ansagen.  
Sie ist nach folgendem Schema aufgebaut:

```text
<Ansagen 1>/<Ansagen 2>/<Ansagen 3>/<Ansagen 4>
```

- *Ansagen* 1, 2, 3 & 4 geben je die Ansagen von Spieler:in 1 - 4 an.

Jede Ansage wird durch ein Zeichen notiert.
Bei mehreren Ansagen werden die Zeichen aneinandergekettet.

| Ansage | Zeichen |
| --- | --- |
| Pagat | 1 |
| Uhu | 2 |
| Pelikan | 3 |
| Quappil | 4 |
| Trull | T |
| König Ultimo | U |
| Könige | K |
| Valat | V |

Beispiele:

- Niemand sagt etwas an: `///`
- Spieler:in 2 sagt Pagat und Trull an: `/1T//`
- Spieler:in 1 sagt Pagat und Spieler:in 4 sagt Könige an: `1///K`

### Kleinen stechen die Großen

Die *Kleinen stechen die Großen* Gruppe gibt an ob in dem Spiel "Die Kleinen stechen die Großen" gilt und wird mit dem Buchstaben `J` notiert.
Falls es nicht angesagt wurde oder nicht anwendbar ist, wird `-` notiert.

### Spritzen


