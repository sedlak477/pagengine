# Pagengine

Pagengine ist eine Traock-Engine zum Analysieren von Tarockspielen und um optimale Spielzüge zu finden.

Zur Beschreibung von Spielen wird das [Tarockaustauschformat (TAF)](/Rock Dateiformat) und das [ROCK Dateiformat](/Rock Dateiformat) verwendet.

## Build

```cmd
cargo build
```

## Tests

```cmd
cargo test
```

## Funny Tarock Zahlen

### Es gibt ca. $3.4×10^{11}$ mögliche Hände

Die tatsächliche Zahl ist: $\binom{54}{12} = 343,006,888,770$.

Das entspricht etwa der Anzahl an Sternen in der Milchstraße.

### Es gibt ca. $1.2×10^{35}$ verschiedene Tarockspiele

Die tatsächliche Zahl ist: $\binom{54}{12} \cdot \binom{42}{12} \cdot \binom{30}{12} \cdot \binom{18}{12} \cdot \binom{6}{3} \cdot \binom{3}{3} = 121,805,714,747,949,970,276,376,111,645,280,000$.

Jedes Sandkorn auf der Erde müsste ein Tarockspiel pro Sekunde für das gesamte Alter unseres Universums spielen, um alle möglichen Tarockspiele zu spielen.
