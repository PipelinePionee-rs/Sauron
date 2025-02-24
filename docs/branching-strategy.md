![Image of branching strategy](./images/image-243.png)


Vi har valgt at bruge en feature branching strategi, hvor vi har:
* 'main' branch der bruges til produktionskode
* 'dev' branch der bruges til udvikling og test af kode
* 'feature' branches, hvor vi bygger features, og integrerer dem ind i dev.

Denne strategi mindsker risikoen for konflikter, og er god hvis man arbejder flere på samme projekt. det kan også nemt skaleres op hvis der kommer flere udviklere på projektet.

Det er også smart at skilne mellem produktionskode, og kode der stadig er under udvikling.

Dog kan det være komplekst at arbejde i hvis man ikke er vant til det, og kan skabe mergekonflikter hvis en featurebranch har været under udvikling i længere tid.
