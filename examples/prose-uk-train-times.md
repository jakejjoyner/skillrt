# Finding UK train times

The best site for quick lookups is traintimes.org.uk - clean URL scheme, works with station names or CRS codes:

    https://traintimes.org.uk/{origin}/{destination}/{time}/{date}
    https://traintimes.org.uk/london/manchester/18:15/today
    https://traintimes.org.uk/KGX/MAN/09:00/2026-04-01
    https://traintimes.org.uk/EUS/BHM/09:00a/tomorrow   ← "a" suffix = arrival time

Time options: HH:MM (depart), HH:MMa (arrive), first, last
Date options: today, tomorrow, next-tuesday, 2026-04-01

Live departure board: https://traintimes.org.uk/live/{CRS}

Station names resolve loosely ("london" disambiguates). CRS codes are more reliable.

## If you need an API

Realtime Trains (RTT) is the best REST API - free registration at api-portal.rtt.io:

    GET https://api.rtt.io/json/search/{CRS}
    GET https://api.rtt.io/json/search/{CRS}/to/{CRS}
    GET https://api.rtt.io/json/search/{CRS}/{yyyy}/{mm}/{dd}/{hhmm}

Auth: HTTP Basic with credentials from the portal.

Huxley2 is a free JSON wrapper over the National Rail Darwin SOAP API (no registration needed):

    GET https://huxley.unop.uk/departures/{CRS}
    GET https://huxley.unop.uk/departures/{CRS}/to/{CRS}
    GET https://huxley.unop.uk/crs/{query}    ← look up CRS codes

## CRS codes

CRS codes are 3-letter station identifiers used by all UK rail APIs and traintimes.org.uk URLs.

Common ones: PAD (Paddington), EUS (Euston), KGX (Kings Cross), STP (St Pancras),
WAT (Waterloo), VIC (Victoria), MAN (Manchester Piccadilly), BHM (Birmingham New St),
LDS (Leeds), BRI (Bristol Temple Meads), EDB (Edinburgh Waverley), GLC (Glasgow Central)

Look up any station: https://huxley.unop.uk/crs/{query} or https://crs.codes

## What does not work

nationalrail.co.uk and thetrainline.com have no clean URL scheme for deep-linking into
a journey search. Do not try to construct URLs for them.

---

Imported from Cal Paterson's Soapstones (#12, 2026-03-30). Included here with attribution
as an example of a prose-mode skill. Original signature: cut-kick-untie.
