# grafia concisă: regulile în vigoare

> **Sursa de adevăr este [`concise-writing-spec.md`](concise-writing-spec.md).** Acest fișier este doar redarea ei în română. Dacă cele două diferă, are dreptate specificația.
>
> **Regulă:** nu adăuga aici nimic care nu apare deja în specificație. O regulă nouă sau schimbată se scrie întâi acolo și abia apoi se reflectă aici. Motivele și istoricul deciziilor, cu datele lor, sunt în [`DECISIONS.md`](DECISIONS.md).

## Regula

| Standard | Concis | Cod (mică / mare) |
|---|---|---|
| ie | ẹ | U+1EB9 / U+1EB8 |
| ia | ė | U+0117 / U+0116 |
| ea | ē | U+0113 / U+0112 |
| oa | ō | U+014D / U+014C |

Concizia se aplică ori de câte ori nu e vorba de un hiat, adică atunci când cele două vocale sunt în aceeași silabă. Accentul nu contează și nu există cazuri speciale după ce/ci/ge/gi (*ceasornic* → *cēsornic*).

În interiorul unei silabe, căutarea merge de la stânga la dreapta, fără suprapuneri: *leoarcă* → *leōrcă*, *aripioară* → *aripiōră*.

O formă poate avea mai multe grafii concise corecte. Are întotdeauna una implicită (*ceai* → *cēi*). O formă cu două despărțiri în silabe (un omograf) are și cealaltă grafie, iar utilizatorul poate comuta pe ea (vezi *Omografe*). Excepția e regula opțională pentru -ee, care dă o singură grafie pe formă.

## Sursa despărțirii în silabe

Un cuvânt-titlu își folosește propria despărțire. Orice altă formă flexionară își ia tăieturile din despărțirea lemei (vezi tabelul de deducție de mai jos).

Despărțirea unei leme este cea explicită a lui Cezar, dacă există (`rules::SYLLABIFICATIONS`: *neant* `ne-ant`, *coastă* `coas-tă`, *mielea* `mi-e-lea`, *înfieri* `în-fi-e-ri`, *răspieri* `răs-pie-ri`, ...; o intrare poate avea mai multe citiri, iar prima e cea implicită: *ie* `i-e` / `ie`, *miază* `mia-ză` / `mi-a-ză`, *vier* `vier` / `vi-er`). Altfel este despărțirea din dexonline, completată de regulile de rezervă. O formă trecută acolo își ia toate grafiile de acolo, oricare i-ar fi lema: *piei* e doar *pẹi* (nu și *piei* ← *pia* `pi-a`).

- **Despărțirea din dexonline.** Se folosește doar prima variantă. Poate fi completă (`fa-mi-li-e`), un final de cuvânt (`-li-e`), un început de cuvânt (`a-ce-tal-`) sau un interior de cuvânt (`-pioa-`). În porțiunea pe care o acoperă, tăieturile ei au întotdeauna prioritate față de reguli. O despărțire care nu se potrivește cu cuvântul e ignorată.
- **Regulile de rezervă.** În rest, regulile pun doar tăieturi între două vocale, pentru că grafia concisă nu folosește altele. Regulile sunt incomplete și vor fi îmbunătățite:

| Vocale | Context | Tăietură | Exemple |
|---|---|---|---|
| ie | în -ient, -ienț/-ient + a/ă/e/i | hiat | *client*, *ambient*, *experiență*, *clientelă* (dexonline: *farniente* `far-nien-te`) |
| ia, ea | înainte de -bil | hiat | *fiabil*, *agreabil* |
| e-e | oriunde | hiat | *idee* → *ide-e* (comutatorul -ee îl citește *e-ie*) |
| e-a, i-a, i-e, o-a | după un prefix (re-, pre-, ne-, tele-, co-, semi-, anti-, multi-, auto-, micro-, macro-, foto-, pseudo-, bio-, neo-, radio-, aero-, agro-, electro-, hidro-, termo-, geo-, proto-, socio-, psiho-, stereo-), când restul e un cuvânt cunoscut: o formă a unei leme dexonline scrise cu literă mică, cu frecvența ≥ 0,9 și de cel puțin 4 litere | hiat | *re-abilitat*, *ne-adevărat*, *auto-analiză* (nu *neam*, *neagră*, *autoare*; *prea-curat* e *prea* + *curat*); *co-activ* (potrivirile greșite ale lui co- sunt corectate în `rules::SYLLABIFICATIONS`: *coastă*, *coardă*, *coacere*, *coacăț*) |
| i-e | prefixul bi- înainte de en | hiat | *bienal* (nu *biet*) |
| i-i | la final, după o consoană, într-un infinitiv verbal | hiat | *sfi-i*, *pri-i*, deci *sfiește*, *priască* își păstrează i-e/i-a (nu pluralul *copii*) |
| ea | terminații și rădăcini neologice (`rules::EA_HIATUS`): -eal final (nu după c/g/d/b), -linear/-hlear/-balnear, -laureat/-lineat/-nucleat și altele asemănătoare, nuclear-, creat-, reac(t/ț)-, steat-, meandr-, aleat-, beatif-, reali-/ideali-/boreal/cereal | hiat | *real*, *leal*, *boreal*, *nuclear*, *denuclearizat*, *linear*, *alineat*, *creator*, *pancreatic*, *reactor*, *reacție*, *steatoză*, *meandru*, *aleator*, *beatific*, *realizat*, *cereale* (nu *deal*, *Ardeal*, *beat*, *asear*, *greșeală*) |
| oa | rădăcini și terminații neologice (`rules::OA_HIATUS`): coagul-, coabit-, coaliț-/coaliz-, -zoar final | hiat | *coagulat*, *coabita*, *coaliție*, *coalizat*, *protozoar*, *cazoar* (nu *mozoară*) |
| ie, ia | la început de cuvânt sau după o vocală (inclusiv ă, î) | diftong | *iarbă*, *femeie*, *tăia*, *tăiere*, *înmuiere* |
| ie, ia | la final de cuvânt, după consoană | hiat | *familie*, *vie*, *istoria* |
| ia | în interior, după consoană (terminațiile -ia\*) | hiat | *apropiat*, *pian*, *social*, *specializat*, *glaciar*, *variare* |
| ia | la fel, dar în rădăcini native (`rules::IA_DIPHTHONG`) | diftong | *chiar*, *piatră*, *biată*, *viață*, *amiază*, *ghiață*, *maghiar*, *Caragiale*, *chiajna*, *obială* (nu *liturghiar*, *eleghiacesc*, *schiagramă*, *Gorghias*, *Miaulis*, *microbial*) |
| ia | -iare final după p/b/f/v/m sau c/g/ch/gh, într-un cuvânt care nu e infinitiv lung (vezi mai jos) | diftong | *fiare* (ca *fiere*) |
| ie | în interior, după p/b/f/v/m sau c/g/ch/gh | diftong | *fier*, *piept*, *pieptene* |
| ie | la fel, dar în rădăcini și terminații neologice (`rules::IE_HIATUS`: gien, pieton, cviet, societ, soviet, vien, miel + o/in/it/ie, -ietor/-ietoare, bier care nu e nici la început de cuvânt, nici după z/s), în -ier/-ieră final după o vocală și o consoană, -iesc final (adjective) și -iere/-ierea/-ieri/-ierii/-ierile/-ierilor final (infinitive lungi; după o vocală e diftong, vezi mai sus: *tăiere*) | hiat | *igienă*, *pieton*, *sovietism*, *cvietism*, *vienez*, *mielită*, *poliomielită*, *așchietor*, *bărbierit*, *corăbierie*, *bărbier*, *plumier*, *banchier*, *genunchier*, *legumieră*, *zapciesc*, *surugiesc*, *copiere*, *apreciere*, *veghiere*, *învierea*, *aprecierilor* (nu *miel*, *mielușor*, *zbiera*, *fier*, *zbier*; nu *miere*, *înmiere*, *fiere*, *piere* și formele lor (*mierea*, *pieri*): `rules::IERE_DIPHTHONG`) |
| ie | în interior, după altă consoană | hiat | *prieten*, *cartier* |
| orice altceva (ea, oa, ...) | oriunde | diftong | *seară*, *soare* |

Când mai multe înregistrări dau aceeași formă, grafia implicită se alege după prioritate (celelalte grafii devin variante, vezi *Omografe*):
1. Orice înregistrare bate una a unui infinitiv lung în -iare (vezi mai jos).
2. O despărțire susținută de una explicită, din dexonline sau parțială bate regulile singure, chiar și la un cuvânt-titlu: *agreare* e *agreare* (← *agrea* `a-gre-a`), nu cuvântul-titlu *agreare* despărțit prin reguli (*agrēre*).
3. Un cuvânt-titlu bate o formă dedusă.
4. O despărțire explicită bate dexonline, iar o lemă despărțită complet de dexonline bate una despărțită parțial.
5. Altfel câștigă prima înregistrare: a lexemului adăugat primul în dexonline, apoi a formei lui adăugate prima (după id-urile lor).

Tăieturile se deduc din lemă pe rând, pentru fiecare pereche de vocale:

| Perechea de vocale | Decide | Exemple |
|---|---|---|
| ambele litere în prefixul comun cu lema | tăietura lemei | *soarele* → *sōrele* (← *soa-re*), *reală* neschimbat (← *re-al*) |
| terminația înlocuiește o vocală a lemei pe același loc, după i | tăietura lemei | *familia* neschimbat (← *fa-mi-li-e*), *femeia* → *femeė* (← *fe-me-ie*) |
| terminația înlocuiește u final al unei leme în -iu | hiat | *argintie*, *argintia* neschimbate (← *ar-gin-tiu*), *cheflie* (← *che-fliu*) |
| terminație adăugată, alternanță în temă sau o/e înaintea unei vocale înlocuite | diftong | *ideea* → *ideē*, *creează* → *creēză*, *doarme* → *dōrme*, *ploaie* → *plōẹ* (← *plo-ua*), *scheaună* → *schēună* (← *sche-u-na*) |

Sufixul -ează/-ească al verbelor și adjectivelor după un i în hiat (scris -ază/-ască/-asca) păstrează o formă de e, ca la celelalte forme (*studiez*, *studiezi*; *sfiesc*, *sfiești*; *fiesc*, *fiești*), și se scrie -ėză/-ėscă/-ėsca:

| Forma | Lema | Rezultat |
|---|---|---|
| lema + -ză | se termină în hiatul i-a (verb de conjugarea I) | *studiază* → *studiėză* (← *stu-di-a*), *pliază* → *pliėză*, *înfiază* → *înfiėză*, *remaniază* → *remaniėză* |
| lema fără i final + -ască | se termină în hiatul i-i (verb de conjugarea a IV-a) | *sfiască* → *sfiėscă* (← *sfi-i*), *priască* → *priėscă* |
| lema fără -esc + -ască, -asca | se termină în hiatul i-esc (adjectiv) | *fiască* → *fiėscă*, *fiasca* → *fiėsca* (← *fi-esc*), *pustiască* → *pustiėscă* |

Un infinitiv lung în -iare nu e corect și se scrie în forma lui în -iere, care urmează apoi regulile pentru -iere: *apreciare* → *apreciere*, *apreciarea* → *aprecierē*, *tăiare* → *tăẹre*. În formele lui, ultimul -iar-/-iăr- devine -ier-: *apreciărilor* → *aprecierilor*. Un infinitiv lung este o lemă în -iare pe care dexonline o dă cu literă mică, ca substantiv feminin sau neclasificată (modelul F, T sau IL), cu excepția lui *fiare* și *mobiliare* (`rules::IARE_KEPT`). O formă pe care o dă și altă lemă păstrează grafia acelei leme: *conciliare* (adjectivul *conciliar*), *copiare* (substantivul *copiar*). Grafia în -iere nu apare atunci ca variantă pe comutatorul de omografe.

## Regula opțională pentru -ee (comutator, oprit implicit)

Lemele în *-ee* urmează modelul *femeie*: *-ee* final se citește *-eie* și se scrie *-eẹ*, la fel ca *femeie* → *femeẹ*.

| Forma | Condiția | Rezultat |
|---|---|---|
| singular în -ee | întotdeauna | *idee* → *ideẹ*, *alee* → *aleẹ* |
| plural în -ee | lema se termină în -eu | *bombee* (← *bombeu*) → *bombeẹ* |
| plural în -eele, -eelor | lema se termină în -eu | *bombeele* → *bombeẹle*, *liceelor* → *liceẹlor* |
| plural în -euri, -eurile, -eurilor | forma fără -ri/-rile/-rilor e lema ei în -eu, de mai mult de 4 litere | *bombeuri* → *bombeẹ*, *bombeurile* → *bombeẹle*, *liceurilor* → *liceẹlor* (nu *pneuri*, *zeurile*) |
| plural în -ee | lema se termină în -ee | *logoree* → *logorei* (ca *femeie* / *femei*) |
| plural în -ee | orice altă lemă | neschimbat |
| -eea final | lema se termină în -ee sau -eu | *ideea* → *ideė*, *orhideea* → *orhideė* (ca *femeia*), *spontaneea* → *spontaneė* (← *spontaneu*); grafia de bază este *ideē* (*i-de-ea*) |
| -eea final | lema e un cuvânt-titlu în -eea, care nu e în `rules::EEA_UNCHANGED` | *Niceea* → *Niceė*, *Andreea* → *Andreė*, *Galileea* → *Galileė* (pronumele *aceea*, *ceea*, *oareceea* rămân cu *-eē*); la fel un cuvânt necunoscut |
| -eei final | lema ca la -eea: în -ee sau -eu, ori un cuvânt-titlu în -eea care nu e în `rules::EEA_UNCHANGED` | *ideei* → *ideii*, *orhideei* → *orhideii* (ca *femeii*), *spontaneei* → *spontaneii*, *Galileei* → *Galileii* |
| -eele final | lema se termină în -ee | *orhideele* → *orhideile* (ca *femeile*) |
| -eelor final | lema se termină în -ee | *orhideelor* → *orhideilor* (ca *femeilor*) |

Dacă o grafie este și singularul, și pluralul unei leme în -ee, câștigă citirea de singular, așa că fiecare formă are tot o singură grafie.

Cu comutatorul pornit, conversia inversă caută cuvintele în bază ca să anuleze regula (*ideẹ* → *idee*, *orhideii* → *orhideei*; vezi *Conversia inversă*).

## Regula opțională â → î (comutator, pornit implicit)

â se scrie î (Â ca Î), cu excepția cuvintelor cu rădăcina *român* sau *rumân*, adică a oricărui cuvânt care conține *român* sau *rumân*: *câine* → *cîine*, *când* → *cînd*, *pâinea* → *pîinē*; neschimbate: *român*, *românesc*, *România*, *aromân*, *dacoromân*, *rumân*, *rumânesc*.

Se aplică fiecărui cuvânt după restul conversiei, fie că e în baza de date, fie că nu, și nu marchează niciodată un cuvânt ca necunoscut. Se aplică tuturor grafiilor unui omograf.

## Omografe

O formă ale cărei înregistrări dau grafii diferite este un omograf. Exemple:
- *ia*: *ė* (verbul *lua*, interjecția) sau *ia* (*ie* articulat, *i-a*);
- *ie*: *ie* (bluza, *i-e*) sau *ẹ* (adverbul);
- *miază*: *mėză* (amiază, *mia-ză*) sau *miază* (boala, *mi-a-ză*);
- *curie*: *curẹ* sau *curie*;
- *liază*: *liază* (substantivul) sau *liėză* (verbul *lia*);
- *vier*: *vẹr* (porcul, *vier*) sau *vier* (păzitorul de vie, *vi-er*).

Grafia implicită e cea aleasă după prioritatea de mai sus. Celelalte sunt păstrate în `forms.alternatives` și returnate împreună cu ea: în `segments[].alts` de la `convert` din nucleu și în `alternatives` de la `lookup`, cu cea implicită prima în ambele. Contează doar o grafie susținută de o despărțire explicită, din dexonline sau parțială, cu o excepție: când o înregistrare dă sufixul -ėză/-ėscă/-ėsca (vezi *Sursa despărțirii în silabe*), contează toate grafiile formei. `rules::SYLLABIFICATIONS` poate da mai multe citiri pentru un cuvânt; prima e cea implicită.

În interfață, toate grafiile unui omograf apar într-un singur comutator, pe un fond gri-închis. Grafia activă stă pe fondul zonei de text, iar celelalte apar gri-deschis pe fondul gri. Un clic pe una o face activă. Copierea ia grafiile active.

Regula -ee nu produce omografe: alegerea ei între singular și plural rămâne unică (vezi *Regula opțională pentru -ee*). Excepția e un cuvânt necunoscut în -eei, -eele sau -eelor (vezi *Cuvinte necunoscute*). Conversia inversă cu comutatorul -ee își arată egalitățile pe același comutator (vezi *Conversia inversă*).

## Majuscule

Tiparul de majuscule al cuvântului se păstrează: *Soare* → *Sōre*, *SOARE* → *SŌRE*. Celelalte tipare mixte se reduc la inițială mare.

## Normalizarea textului

Textul e normalizat NFC. La căutare, ş/ţ cu sedilă sunt tratate ca ș/ț cu virgulă.

## Cuvinte necunoscute

Un cuvânt care conține un grup candidat, dar lipsește din baza de date, e convertit doar de regulile de rezervă și e marcat. Cu comutatorul -ee e citit ca singular, cu două excepții.

Un cuvânt cu o terminație de adjectiv în -iesc după i (-iești, -iescu, -iescul, -iescului, -ieștii, -ieștile, -ieștilor, -iască, -iasca) e legat întâi de lema lui: terminația e înlocuită cu -esc, iar dacă rezultă o lemă în -iesc (`iesc_lemmas`), cuvântul e derivat din despărțirea lemei, inclusiv regula sufixului. dexonline are unele dintre aceste adjective fără forme: *harabagiasca* → *harabagiėsca*, *harabagiască* → *harabagiėscă*, *harabagiești* neschimbat (← *harabagi-esc*).

Un cuvânt în -eei e citit întâi ca genitivul unui cuvânt cunoscut: -eei final se înlocuiește cu -eea, apoi cu -ee, iar primul cunoscut e lema lui: cel în -eea trebuie să fie lemă (`ea_lemmas`), cel în -ee orice formă (`forms`). Regula -eei dă atunci o singură grafie: *Andreei* → *Andreii* (← *Andreea*, invariabil în dexonline, fără genitiv), *aleei* → *aleii* (← *alee*), *aceei* neschimbat (← *aceea*, vezi `rules::EEA_UNCHANGED`). *ideea* e doar o formă a lui *idee*, deci nu e lemă în -eea; *ideei* → *ideii* vine din *idee*. Dacă niciunul nu e cunoscut, cuvântul primește două grafii, neschimbat (implicit) sau cu -eii, afișate pe comutatorul de omografe: *Zzeei* sau *Zzeii*.

Un cuvânt în -euri/-eurile/-eurilor devine -eẹ/-eẹle/-eẹlor când cuvântul fără -ri/-rile/-rilor e o lemă în -eu (`eu_lemmas`) de mai mult de 4 litere, ca la o formă cunoscută. Altfel rămâne neschimbat.

Un cuvânt în -eele/-eelor:
- -ele/-elor final se înlocuiește cu -u. Dacă rezultă o lemă în -eu din baza de date (`eu_lemmas`), cuvântul e pluralul ei și are o singură grafie: *bombeele* → *bombeẹle* (← *bombeu*, al cărui plural în dexonline este *bombeuri*).
- Altfel primește două grafii, ca plural al unei leme în -ee (*coloboleile*, *coloboleilor*, implicit) sau al unei leme în -eu (*coloboleẹle*, *coloboleẹlor*). Ele apar pe comutatorul de omografe, cu o notă de subsol care folosește rădăcina cuvântului: „Dacă singularul e *colobolee*, alege forma *coloboleile/lor*. Dacă singularul e *coloboleu*, alege forma *coloboleẹle/lor*.”

Cuvintele fără grup candidat și fără iăr nu sunt căutate deloc.

## Conversia inversă

Conversia concis → standard este deocamdată o înlocuire de caractere (ẹ→ie, ė→ia, ē→ea, ō→oa), cu excepția lui ė din -iėză/-iėscă/-iėsca final, care redevine a (*studiėză* → *studiază*, *sfiėscă* → *sfiască*, *fiėsca* → *fiasca*). Asta e valabil doar cu regulile actuale și nu e o proprietate garantată. E inexactă pentru *fiiască*, *fiiasca* → *fiėscă*, *fiėsca*, care redevin *fiască*, *fiasca*. Un infinitiv lung în -iare nu e refăcut: *apreciere* rămâne *apreciere*. î rămâne neschimbat, deci â nu e refăcut după regula â → î (*cîine* rămâne *cîine*).

Cu comutatorul -ee pornit, textul e citit ca scris cu regula -ee. Un cuvânt e căutat întâi printre grafiile pe care regula le dă formelor din bază (`ee_revert`) și revine la forma din care provine: *orhideẹ* → *orhidee*, *orhideile* → *orhideele*, *ideė* → *ideea*, *bombeẹle* → *bombeurile* (dexonline nu are *bombeele*). Mai multe forme din dexonline pot da aceeași grafie, prin regulă sau neschimbate (*zeii*: forma *zeii*, sau *zeei* prin regulă). Ele sunt restrânse în doi pași:
1. o formă a unui lexem DEX-MDA-DOOM bate una fără. Lexemul trebuie să aibă o definiție proprie (al cărei titlu e chiar el) din MDA\*, DEX\*, NODEX\* sau DOOM\*; o mențiune în definiția altui cuvânt nu contează. *aleẹ* → *alee* (nu *aleie*, doar în Scriban), *cheẹ* → *cheie* (nu *chee*), *acneẹ* → *acnee* (*acneie* e doar pomenit la *coș* în MDA2);
2. între formele DEX-MDA-DOOM, un plural în -ee/-eele/-eelor bate unul în -euri/-eurile/-eurilor: *apogeẹ* → *apogee* (nu *apogeuri*).

Formele rămase la egalitate sunt oferite toate pe comutatorul de omografe (vezi *Omografe*). Cea implicită e rezultatul înlocuirii de caractere, altfel prima în ordinea alfabetului românesc: *bordeẹ* → *bordeie* (pluralul lui *bordei*) sau *bordee* (pluralul lui *bordeu*); *beẹ* → *beie* sau *bee*; *zeii* → *zeii* sau *zeei*.

Orice alt cuvânt, inclusiv cele necunoscute, trece prin înlocuirea de caractere (*femeẹ* → *femeie*, *zzeẹ* → *zzeie*). Căutarea nu ține cont de regula â → î, așa că un cuvânt cu â în bază revine prin înlocuirea de caractere după ea (*vîntureẹ* → *vîntureie*, nu *vânturee*).

## Date

Datele despre cuvinte vin într-un singur fișier, `grafia.dat` (structura lui e în `crates/core/src/datafile.rs`). Pe lângă cinci tabele, conține versiunea formatului, ziua la care sunt datele dexonline din care a fost construit (ziua ultimei modificări a unui lexem sau a unei definiții în dexonline) și o sumă de control, verificată la deschiderea fișierului. Fiecare tabel are cheia un cuvânt normalizat:
- `forms`: o formă → `concise`, grafia ei concisă; `concise_ee`, grafia ei cu regula -ee, când diferă de `concise`; și `alternatives`, celelalte grafii ale unui omograf.
- `eu_lemmas`: toate lemele normalizate în -eu, pentru pluralele necunoscute în -eele/-eelor (vezi *Cuvinte necunoscute*).
- `ea_lemmas`: toate lemele normalizate în -ea, pentru genitivele necunoscute în -eei (vezi *Cuvinte necunoscute*).
- `iesc_lemmas`: toate lemele normalizate în -iesc → despărțirea lor (*harabagi-esc*), pentru formele necunoscute ale adjectivelor în -iesc (vezi *Cuvinte necunoscute*).
- `ee_revert`: o grafie cu regula -ee → forma la care revine și celelalte forme ale unei egalități, doar unde înlocuirea de caractere singură nu dă răspunsul (vezi *Conversia inversă*).

Aceleași tabele dau mereu același fișier, octet cu octet.

`forms` păstrează doar formele care pot diferi: cele care conțin ie/ia/ea/oa sau iăr, ori se termină în -ee, -eea, -eei, -eele, -eelor, -euri, -eurile sau -eurilor. Intrarea pipeline-ului pentru fiecare formă flexionară este: forma, lema ei, despărțirea lemei din dexonline (completă, parțială sau goală), numărul (S/P), dacă lema e verb (modelul V sau VT din dexonline), dacă e substantiv feminin sau neclasificată, scrisă cu literă mică (modelul F, T sau IL; pentru infinitivele lungi în -iare) și dacă e DEX-MDA-DOOM: are o definiție activă proprie (al cărei titlu e chiar lexemul) dintr-o sursă al cărei nume scurt începe cu MDA, DEX, NODEX sau DOOM (MDA2, DEX '09, NODEX, DOOM 3, ...; nu dicționarul intern *dexonline*).
