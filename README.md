# Rss-Reader

Medium RSS Reader je jednostavna aplikacija napisana u Rustu koja omogućava korisnicima da preuzimaju RSS feed sa Medium stranice, smeštaju ih u lokalnu SQLite bazu podataka i čitaju najnovije feedove. Ova aplikacija je dizajnirana kao početnički projekat i ima za cilj pružiti osnovno razumevanje programskog jezika Rust i njegovih biblioteka, kao i rad sa grafičkim sučeljem i bazom podataka.
Funkcionalnosti

    Preuzimanje RSS feedova: Aplikacija koristi Rust HTTP biblioteku (na primer, reqwest) za slanje HTTP zahteva i preuzimanje XML datoteke koja sadrži RSS feedove sa Medium stranice.

    Skladištenje u SQLite bazi podataka: Preuzeti feedovi se smeštaju u lokalnu SQLite bazu podataka radi kasnijeg čitanja i pretrage.

    Čitanje najnovijih feedova: Korisnicima je omogućeno da čitaju najnovije feedove direktno iz aplikacije.

    Grafičko sučelje: Aplikacija je implementirana sa Rust grafičkim sučeljem (na primer, druid) za lakšu interakciju sa korisnicima.

# Uputstvo za instalaciju

Pratite sledeće korake kako biste instalirali i pokrenuli Medium RSS Reader na vašem Windows računaru:
# 1. Preuzimanje repozitorijuma

Klonirajte ovaj GitHub repozitorijum ili preuzmite ZIP arhivu sa sledećeg linka: Link do repozitorijuma.
# 2. Instaliranje Rusta

Aplikacija je napisana u Rustu, pa ćete morati da imate Rust instaliran na svom računaru. Možete preuzeti i instalirati Rust sa zvaničnog sajta: Rust Download
# 3. Instaliranje neophodnih biblioteka

Otvorite komandni prompt ili terminal i navigirajte do direktorijuma sa preuzetim repozitorijumom. Zatim instalirajte neophodne Rust biblioteke koristeći cargo:

  `bash
  cargo build

# 4. Pokretanje aplikacije

Pokrenite aplikaciju iz komandnog prompta ili terminala:

  `bash
  cargo run

Aplikacija će se pokrenuti i pojaviti na ekranu. Možete početi preuzimanje i čitanje Medium feedova.
Napomena

Ova aplikacija je dizajnirana isključivo u edukativne svrhe i ne podržava osvežavanje informacija u stvarnom vremenu. Takođe, trenutno je testirana samo na Windows operativnom sistemu.
