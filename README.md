# Rss-Reader

Medium RSS Reader je jednostavna aplikacija napisana u Rustu koja omogućuje korisnicima da preuzimaju RSS feed sa Medium stranice, smještaju ih u lokalnu SQLite bazu podataka i čitaju najnovije feedove. Ova aplikacija je dizajnirana kao početnički projekt i ima za cilj pružiti osnovno razumjevanje programskog jezika Rust i njegovih biblioteka, kao i rad sa grafičkim sučeljem i bazom podataka.
Funkcionalnosti

Preuzimanje RSS feedova: Aplikacija koristi Rust HTTP biblioteku (na primer, reqwest) za slanje HTTP zahtjeva i preuzimanje XML datoteke koja sadrži RSS feedove sa Medium stranice.

Skladištenje u SQLite bazi podataka: Preuzeti feedovi se smještaju u lokalnu SQLite bazu podataka radi kasnijeg čitanja i pretrage.

Čitanje najnovijih feedova: Korisnicima je omogućeno da čitaju najnovije feedove direktno iz aplikacije.

Grafičko sučelje: Aplikacija je implementirana sa Rust grafičkim sučeljem (Dioxus v0.4) za lakšu interakciju sa korisnicima.

# Upute za instalaciju

Pratite sljedeće korake da biste instalirali i pokrenuli Medium RSS Reader na vašem Windows računalu:
# 1. Preuzimanje repozitorija

Klonirajte ovaj GitHub repozitorij ili preuzmite ZIP arhivu
# 2. Instaliranje Rusta

Aplikacija je napisana u Rustu, pa ćete morati imati Rust instaliran na svom računalu.
# 3. Instaliranje biblioteka

Otvorite naredbeni redak ili terminal i navigirajte do direktorija sa preuzetim repozitorijem. Zatim instalirajte Rust biblioteke koristeći cargo:

    cargo build

# 4. Pokretanje aplikacije

Pokrenite aplikaciju iz terminala:

      cargo run

Aplikacija će se pokrenuti i pojaviti na ekranu. Možete početi preuzimanje i čitanje Medium feedova.
# Napomena

Ova aplikacija je dizajnirana isključivo u edukativne svrhe i ne podržava osvežavanje informacija u stvarnom vremenu. Takođe, trenutno je testirana samo na Windows operativnom sistemu.
