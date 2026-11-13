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

Concizia se aplică ori de câte ori nu e vorba de un hiat, adică atunci când cele două sunete sunt în aceeași silabă. Accentul nu contează și nu există cazuri speciale după ce/ci/ge/gi (*ceasornic* → *cēsornic*).

În interiorul unei silabe, căutarea merge de la stânga la dreapta, fără suprapuneri: *leoarcă* → *leōrcă*, *aripioară* → *aripiōră*.

O formă poate avea mai multe grafii concise corecte. Are întotdeauna una implicită (*ceai* → *cēi*). O formă cu două despărțiri în silabe (un omograf) are și cealaltă grafie, iar utilizatorul poate comuta pe ea (vezi *Omografe*). Excepția e regula opțională -eẹ/aẹ/oẹ, care dă o singură grafie pe formă.

## Sursa despărțirii în silabe

Un cuvânt-titlu își folosește propria despărțire. Orice altă formă flexionară își ia tăieturile din despărțirea lemei (vezi tabelul de deducție de mai jos).

Despărțirea unei leme este cea explicită a lui Cezar, dacă există (`silabisiri.yaml`: *neant* `ne-ant`, *coastă* `coas-tă`, *mielea* `mi-e-lea`, *înfieri* `în-fi-eri`, *răspieri* `răs-pieri`, ...; o intrare poate avea mai multe citiri, iar prima e cea implicită: *miază* `mia-ză` / `mi-a-ză`, *vier* `vier` / `vi-er`). Altfel este despărțirea din dexonline, completată de regulile de rezervă. Cuvântul *ie* are doar citirea `i-e` și rămâne *ie*, fără alternativă. O formă trecută acolo își ia toate grafiile de acolo, oricare i-ar fi lema: *piei* e doar *pẹi* (nu și *piei* ← *pia* `pi-a`).

- **Despărțirea din dexonline.** Se folosește doar prima variantă. Poate fi completă (`fa-mi-li-e`), un final de cuvânt (`-li-e`), un început de cuvânt (`a-ce-tal-`) sau un interior de cuvânt (`-pioa-`). În porțiunea pe care o acoperă, tăieturile ei au întotdeauna prioritate față de reguli. O despărțire care nu se potrivește cu cuvântul e ignorată.
- **Regulile de rezervă.** În rest, regulile pun doar tăieturi între două vocale, pentru că grafia concisă nu folosește altele. Regulile sunt incomplete și vor fi îmbunătățite:

| Vocale | Context | Tăietură | Exemple |
|---|---|---|---|
| ie | în -ient, -ienț/-ient + a/ă/e/i | hiat | *client*, *ambient*, *experiență*, *clientelă* (dexonline: *farniente* `far-nien-te`) |
| ia, ea | înainte de -bil, după o consoană | hiat | *fiabil*, *agreabil* (nu *debreiabil*: după o vocală, ca *de-bre-ia*) |
| e-e | oriunde | hiat | *idee* → *ide-e* (comutatorul -eẹ/aẹ/oẹ îl citește *e-ie*) |
| e-a, i-a, i-e, o-a | după un prefix (re-, pre-, ne-, tele-, co-, semi-, anti-, multi-, auto-, micro-, macro-, foto-, pseudo-, bio-, neo-, radio-, aero-, agro-, electro-, hidro-, termo-, geo-, proto-, socio-, psiho-, stereo-), când restul e un cuvânt cunoscut: o formă a unei leme dexonline scrise cu literă mică, cu frecvența ≥ 0,9 și de cel puțin 4 litere | hiat | *re-abilitat*, *ne-adevărat*, *auto-analiză* (nu *neam*, *neagră*, *autoare*; *prea-curat* e *prea* + *curat*); *co-activ* (potrivirile greșite ale lui co- sunt corectate în `silabisiri.yaml`: *coastă*, *coardă*, *coacere*, *coacăț*) |
| i-e | prefixul bi- înainte de en | hiat | *bienal* (nu *biet*) |
| i-i | la final, după o consoană, într-un infinitiv verbal | hiat | *sfi-i*, *pri-i*, deci *sfiește*, *priască* își păstrează i-e/i-a (nu pluralul *copii*) |
| ea | terminații și rădăcini neologice (`rules::EA_HIATUS`): -eal final (nu după g/d/b), -linear/-hlear/-balnear, -laureat-/-lineat-/-nucleat- și altele asemănătoare (orice ar urma), nuclear-, creat-, reac(t/ț)-, steat-, meandr-, aleat-, beatif-, reali-/ideali-/boreal (nu în -răboreală/-stoboreală)/cereal | hiat | *real*, *leal*, *interliceal*, *boreal*, *hiperboreală*, *nuclear*, *denuclearizat*, *linear*, *alineat*, *anucleată*, *laureată*, *delineator*, *creator*, *pancreatic*, *reactor*, *reacție*, *steatoză*, *meandru*, *aleator*, *beatific*, *realizat*, *cereale* (nu *deal*, *Ardeal*, *mangeal*, *beat*, *asear*, *greșeală*, *răboreală*, *cleat*, *procleat*, *pleată*, *soreată*) |
| oa | rădăcini și terminații neologice (`rules::OA_HIATUS`): coagul-, coabit-, coaliț-/coaliz-, -zoar final | hiat | *coagulat*, *coabita*, *coaliție*, *coalizat*, *protozoar*, *cazoar* (nu *mozoară*) |
| ie, ia | la început de cuvânt sau după o vocală (inclusiv ă, î) | diftong | *iarbă*, *femeie*, *tăia*, *tăiere*, *înmuiere* |
| ie, ia | la final de cuvânt, după consoană | hiat | *familie*, *vie*, *istoria* |
| ia | în interior, după consoană (terminațiile -ia\*) | hiat | *apropiat*, *pian*, *social*, *specializat*, *glaciar*, *variare* |
| ia | la fel, dar în rădăcini native (`rules::IA_DIPHTHONG`) | diftong | *chiar*, *piatră*, *biată*, *viață*, *amiază*, *ghiață*, *ghiarele*, *maghiar*, *Caragiale*, *chiajna*, *obială* (nu *liturghiar*, *eleghiacesc*, *schiagramă*, *Gorghias*, *Miaulis*, *microbial*) |
| ia | -iare final după p/b/f/v/m sau c/g/ch/gh, într-un cuvânt care nu e infinitiv lung (vezi mai jos) | diftong | *fiare* (ca *fiere*) |
| ie | în interior, după p/b/f/v/m sau c/g/ch/gh | diftong | *fier*, *piept*, *pieptene* |
| ie | la fel, dar în rădăcini și terminații neologice (`rules::IE_HIATUS`: gien, pieton, cviet, societ, soviet, vien, miel + o/in/it/ie, -ietor/-ietoare, bier care nu e nici la început de cuvânt, nici după z/s), în -ier/-ieră/-iereasă (și formele lui -iereasă) final după o vocală și o consoană, -iesc final (adjective) și -iere/-ierea/-ieri/-ierii/-ierile/-ierilor final (infinitive lungi; după o vocală e diftong, vezi mai sus: *tăiere*) | hiat | *igienă*, *pieton*, *sovietism*, *cvietism*, *vienez*, *mielită*, *poliomielită*, *așchietor*, *bărbierit*, *corăbierie*, *bărbier*, *plumier*, *banchier*, *genunchier*, *legumieră*, *rachiereasă*, *pompiereasă*, *zapciesc*, *surugiesc*, *copiere*, *apreciere*, *veghiere*, *învierea*, *aprecierilor* (nu *miel*, *mielușor*, *zbiera*, *fier*, *zbier*; nu *miere*, *înmiere*, *fiere*, *piere* și formele lor (*mierea*, *pieri*): `rules::IERE_DIPHTHONG`) |
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
| terminație adăugată după i-ul final al unui verb în consoană + i | hiat | *știa*, *știam*, *știau* neschimbate (← *ști*), *neștie* (← *ne-ști*); nu după o vocală (*locuia* → *locuė* ← *lo-cu-i*) |
| terminație adăugată, alternanță în temă sau o/e înaintea unei vocale înlocuite | diftong | *ideea* → *ideē*, *creează* → *creēză*, *doarme* → *dōrme*, *ploaie* → *plōẹ* (← *plo-ua*), *scheaună* → *schēună* (← *sche-u-na*) |

Sufixul -ează/-ească al verbelor și adjectivelor după un i în hiat (scris -ază/-ască/-asca) păstrează o formă de e, ca la celelalte forme (*studiez*, *studiezi*; *sfiesc*, *sfiești*; *fiesc*, *fiești*), și se scrie -ėză/-ėscă/-ėsca:

| Forma | Lema | Rezultat |
|---|---|---|
| lema + -ză | se termină în hiatul i-a (verb de conjugarea I) | *studiază* → *studiėză* (← *stu-di-a*), *pliază* → *pliėză*, *înfiază* → *înfiėză*, *remaniază* → *remaniėză* |
| lema fără i final + -ască | se termină în hiatul i-i (verb de conjugarea a IV-a) | *sfiască* → *sfiėscă* (← *sfi-i*), *priască* → *priėscă* |
| lema fără -esc + -ască, -asca | se termină în hiatul i-esc (adjectiv) | *fiască* → *fiėscă*, *fiasca* → *fiėsca* (← *fi-esc*), *pustiască* → *pustiėscă* |

Un infinitiv lung în -iare nu e corect și se scrie în forma lui în -iere, care urmează apoi regulile pentru -iere: *apreciare* → *apreciere*, *apreciarea* → *aprecierē*, *tăiare* → *tăẹre*. În formele lui, ultimul -iar-/-iăr- devine -ier-: *apreciărilor* → *aprecierilor*. Un infinitiv lung este o lemă în -iare pe care dexonline o dă cu literă mică, ca substantiv feminin sau neclasificată (modelul F, T sau IL), cu excepția lui *fiare* și *mobiliare* (`invechite.yaml: iare_pastrate`). O formă pe care o dă și altă lemă păstrează grafia acelei leme: *conciliare* (adjectivul *conciliar*), *copiare* (substantivul *copiar*). Grafia în -iere nu apare atunci ca variantă pe comutatorul de omografe. Altfel, infinitivul lung e o formă învechită, iar grafia lui proprie e varianta (vezi *Forme învechite*).

Și alte forme învechite se scriu în forma lor actuală (vezi *Forme învechite*).

## Regula opțională -eẹ/aẹ/oẹ (comutator, pornit implicit)

Lemele în *-ee* urmează modelul *femeie*: *-ee* final se citește *-eie* și se scrie *-eẹ*, la fel ca *femeie* → *femeẹ*. *-ae* și *-oe* finale se citesc *-aie*, *-oie* și se scriu *-aẹ*, *-oẹ*.

O lemă e *păstrată* când se termină în -eu, când se termină în -ee și are un masculin, sau când e un nume feminin de persoană din `ee.yaml: nume` (*Andreea*, *Medeea*, *Leea*, *Zoe*, *Cloe*, ...). O lemă păstrată își păstrează *e*-ul la genitiv și la plural, scris *ẹ*, în loc să urmeze *femeii*, *femeile*. Masculinul unei leme în -ee e același cuvânt în -eu, dacă dexonline îl dă ca substantiv masculin sau adjectiv (modelul M sau A): *zee* ← *zeu*, *atee* ← *ateu*, *pigmee* ← *pigmeu*; nu și un neutru (*propilee*: *propileu*, *atenee*: *ateneu*), nici alt cuvânt (`ee.yaml: fara_masculin`: *coree*, boala, și *coreu*, piciorul metric).

| Forma | Condiția | Rezultat |
|---|---|---|
| singular în -ee | întotdeauna | *idee* → *ideẹ*, *alee* → *aleẹ* |
| plural în -ee | lemă păstrată | *bombee* (← *bombeu*) → *bombeẹ*, *zee* (← *zee*, cu *zeu*) → *zeẹ* |
| plural în -eele, -eelor | lemă păstrată | *bombeele* → *bombeẹle*, *liceelor* → *liceẹlor*, *zeele* → *zeẹle*, *ateelor* → *ateẹlor*, *Andreelor* → *Andreẹlor* |
| plural în -euri, -eurile, -eurilor | forma fără -ri/-rile/-rilor e lema ei în -eu, de mai mult de 4 litere | *bombeuri* → *bombeẹ*, *bombeurile* → *bombeẹle*, *liceurilor* → *liceẹlor* (nu *pneuri*, *zeurile*) |
| plural în -ee | orice altă lemă în -ee | *logoree* → *logorei* (ca *femeie* / *femei*), *propilee* → *propilei* |
| plural în -ee | orice altă lemă | neschimbat |
| -eea final | lema se termină în -ee sau -eu | *ideea* → *ideė*, *orhideea* → *orhideė* (ca *femeia*), *zeea* → *zeė*, *spontaneea* → *spontaneė* (← *spontaneu*); grafia de bază este *ideē* (*i-de-ea*) |
| -eea final | lema e un cuvânt-titlu în -eea, care nu e în `ee.yaml: eea_neschimbate` | *Niceea* → *Niceė*, *Andreea* → *Andreė*, *Galileea* → *Galileė*; la fel un cuvânt necunoscut. Pronumele *aceea*, *ceea*, *oareceea* sunt lăsate regulilor de bază (*aceē*) |
| -eei final | lema ca la -eea, păstrată | *zeei* → *zeẹi*, *ateei* → *ateẹi*, *spontaneei* → *spontaneẹi*, *Andreei* → *Andreẹi*, *Medeei* → *Medeẹi* |
| -eei final | lema ca la -eea, nepăstrată | *ideei* → *ideii*, *orhideei* → *orhideii* (ca *femeii*), *Galileei* → *Galileii* |
| -eele final | orice altă lemă în -ee, sau în -eea care nu e nume | *orhideele* → *orhideile* (ca *femeile*), *Galileele* → *Galileile* (un cuvânt necunoscut) |
| -eelor final | orice altă lemă în -ee, sau în -eea care nu e nume | *orhideelor* → *orhideilor* (ca *femeilor*), *Iudeelor* → *Iudeilor* (un cuvânt necunoscut) |
| -oae final | orice cuvânt | *Băboae* → *Băbōẹ*, *lighioae* → *lighiōẹ* |
| -ae, -oe final | lema e în `ee.yaml: ae_oe` | *Nae* → *Naẹ*, *Nicolae* → *Nicolaẹ*, *Zoe* → *Zoẹ*, *Noe* → *Noẹ*, *aloe* → *aloẹ* (nu *vitae*, *reggae*, *Poe*) |
| -oea final | lema în -oe e în `ee.yaml: ae_oe` | *aloea* → *aloė* (ca *ideea*) |
| -oei, -oele, -oelor finale | lema în -oe e în `ee.yaml: ae_oe`, nu în `ee.yaml: nume` | *aloei* → *aloii*, *aloele* → *aloile*, *aloelor* → *aloilor* (ca *femeii*, *femeile*, *femeilor*) |
| -oei, -oele, -oelor finale | lema în -oe e în `ee.yaml: ae_oe` și în `ee.yaml: nume` | *Zoei* → *Zoẹi*, *Cloei* → *Cloẹi*, *Zoele* → *Zoẹle*, *Zoelor* → *Zoẹlor* |

Formele unei leme în -eie nu au nevoie de regulă: regulile de bază le scriu deja cu *ẹ* (*zeiele* → *zeẹle*, *zeiei* → *zeẹi*, *rotofeiele* → *rotofeẹle*).

`ee.yaml: ae_oe` cuprinde cuvintele în -ae/-oe cărora li se aplică regula, iar numele în -oe din `ee.yaml: nume` contează ca fiind ale ei, fără să fie scrise în ea. Cele mai multe cuvinte în -ae/-oe din dexonline sunt străine (*vitae*, *personae*, *reggae*, *Poe*, *Crusoe*). Lista lasă pe dinafară cuvintele cu grafie străină: un *y*, *th*, *ph*, *rh* sau un *ch* neurmat de e/i (*Pasiphae*, *Callirhoe*).

Dacă o grafie este și singularul, și pluralul unei leme în -ee, câștigă citirea de singular, așa că fiecare formă are tot o singură grafie.

Cu comutatorul pornit, conversia inversă caută cuvintele în bază ca să anuleze regula (*ideẹ* → *idee*, *orhideii* → *orhideei*, *Naẹ* → *Nae*; vezi *Conversia inversă*).

## Regula opțională â → î (comutator, pornit implicit)

â se scrie î (Â ca Î), cu excepția cuvintelor cu rădăcina *român* sau *rumân* (`a-i.yaml: radacini_cu_a`), adică a oricărui cuvânt care conține *român* sau *rumân*: *câine* → *cîine*, *când* → *cînd*, *pâinea* → *pîinē*; neschimbate: *român*, *românesc*, *România*, *aromân*, *dacoromân*, *rumân*, *rumânesc*.

Se aplică fiecărui cuvânt după restul conversiei, fie că e în baza de date, fie că nu, și nu marchează niciodată un cuvânt ca necunoscut. Se aplică tuturor grafiilor unui omograf.

## Omografe

O formă ale cărei înregistrări dau grafii diferite este un omograf. Exemple:
- *ia*: *ė* (verbul *lua*, interjecția) sau *ia* (*ie* articulat, *i-a*);
- *miază*: *mėză* (amiază, *mia-ză*) sau *miază* (boala, *mi-a-ză*);
- *curie*: *curẹ* sau *curie*;
- *liază*: *liază* (substantivul) sau *liėză* (verbul *lia*);
- *vier*: *vẹr* (porcul, *vier*) sau *vier* (păzitorul de vie, *vi-er*).

Grafia implicită e cea aleasă după prioritatea de mai sus. Celelalte sunt păstrate în `forms.alternatives` și returnate împreună cu ea: în `segments[].alts` de la `convert` din nucleu și în `alternatives` de la `lookup`, cu cea implicită prima în ambele. Contează doar o grafie susținută de o despărțire explicită, din dexonline sau parțială, cu o excepție: când o înregistrare dă sufixul -ėză/-ėscă/-ėsca (vezi *Sursa despărțirii în silabe*), contează toate grafiile formei. `silabisiri.yaml` poate da mai multe citiri pentru un cuvânt; prima e cea implicită.

În interfață, toate grafiile unui omograf apar într-un singur comutator, pe un fond gri-închis. Grafia activă stă pe fondul zonei de text, iar celelalte apar gri-deschis pe fondul gri. Un clic pe una o face activă. Copierea ia grafiile active.

Regula -eẹ/aẹ/oẹ nu produce omografe: alegerea ei între singular și plural rămâne unică (vezi *Regula opțională -eẹ/aẹ/oẹ*). Excepția e un cuvânt necunoscut în -eei, -eele sau -eelor, fără lemă cunoscută (vezi *Cuvinte necunoscute*). Conversia inversă cu comutatorul -eẹ/aẹ/oẹ își arată egalitățile pe același comutator (vezi *Conversia inversă*).

## Forme învechite

O formă învechită se scrie în forma ei actuală, iar grafia ei proprie e oferită pe comutatorul de omografe, după cea actuală, cu o notă de subsol: „Forma *alineare* e învechită; forma actuală e *aliniere*.” În `convert` din nucleu, `obsolete` din segment conține cuvîntul, apoi formele lui actuale, cu majusculele cuvîntului (*Alineare*, *Aliniere*); `lookup` întoarce formele actuale în `obsolete`. Formele învechite sînt:
- infinitivele lungi în -iare (vezi *Sursa despărțirii în silabe*): *apreciarea* → *aprecierē* sau *apreciarē*;
- formele lemelor din `invechite.yaml: lemme`, fiecare cu lema ei actuală: *alinea* → *alinia*, *alineare* → *aliniere*, *alini* → *alina*, *rachiareasă* → *rachiereasă*. O formă ia grafia formei lemei actuale cu aceeași flexiune și variantă din dexonline (*alineează* → *aliniėză* ← *aliniază*, *alineez* → *aliniez*), iar grafia ei proprie vine din propria despărțire, trecută cu ea în `invechite.yaml: lemme` (*a-li-ne-a*: *alineēză*, *alineare*; *ra-chi-a-rea-să*: *rachiarēsă*). Cuvintele care încep cu rădăcina uneia (*aline-*, *rachiare-*) sînt căutate și fără grup candidat (*alineez*).

O formă pe care o dă și altă lemă păstrează grafia acelei leme, fără comutator: *alineat* (substantivul), *conciliare* (adjectivul *conciliar*).

O formă a mai multor leme învechite, cu forme actuale diferite, nu se rescrie: comutatorul oferă grafiile ei proprii, cîte una pentru fiecare lemă, în ordinea din `invechite.yaml: lemme`, iar nota numește toate formele actuale. *alineam*, imperfectul și lui *alinea*, și lui *alini*, e *alineam* (*a-li-ne-am*) sau *alinēm* (*a-li-neam*): „Forma *alineam* e învechită; forma actuală e *aliniam* sau *alinam*.”

Un cuvînt care lipsește din bază și începe cu rădăcina unei leme învechite pe care dexonline o are fără forme (`fara_forme: true` în `invechite.yaml: lemme`: *rachiareasă*) ia rădăcina lemei actuale și e convertit doar prin reguli (*rachiareasa* → *rachierēsa*); grafia lui proprie vine din despărțirea lemei învechite (*rachiarēsa*). La celelalte leme învechite, care au toate formele în bază, rădăcina ar prinde doar alte cuvinte, care rămîn neschimbate (*Alinei*, de la *Alina*).

## Majuscule

Tiparul de majuscule al cuvântului se păstrează: *Soare* → *Sōre*, *SOARE* → *SŌRE*. Celelalte tipare mixte se reduc la inițială mare.

## Normalizarea textului

Textul e normalizat NFC. La căutare, ş/ţ cu sedilă sunt tratate ca ș/ț cu virgulă.

## Text neconvertit

Textul dintre `[` și `]` rămâne neschimbat, octet cu octet: fără conversie, fără â → î, fără normalizare. Parantezele dispar. Merge în ambele direcții: *Soare [fier ceai â] fier* → *Sōre fier ceai â fẹr*; *[fẹr] fẹr* → *fẹr fier*. Prima `]` încheie porțiunea, deci porțiunile nu se includ una în alta (*[a [fier] ceai]* → *a [fier cēi]*), iar o `[` sau `]` fără pereche rămâne în text ca un caracter obișnuit. O porțiune goală (`[]`) dispare. În `convert` și `revert` din nucleu, o porțiune e un segment aparte, cu `literal: true` și `t` conținutul ei; restul se convertește ca de obicei, iar comutatorul â → î sare peste porțiuni.

Numele de origine latină nu se convertesc. Un cuvânt e nume latin când e în `neconvertite.yaml: nume_latine` (nume terminate într-o vocală + -ian, numai formele exacte: *Traian*) sau când are majusculă și se termină în -us (*Troianus*, *Gordianus*, *Helianthus*). Nu e căutat deloc, deci nu e marcat ca necunoscut, și își păstrează majusculele: *Traian*, *TRAIAN*. Lista cuprinde deocamdată numai formele exacte: celelalte forme ale lui (*Traiana*, *Traiană* → *Traėnă*) și alte nume terminate într-o vocală + -ian (*Troian* → *Troėn*, *Stoian*, *Boian*) se convertesc, iar un text le păstrează cu o porțiune `[ ]` (*[Traiana]*). Formele pot fi adăugate pe listă mai târziu.

## Cuvinte necunoscute

Un cuvânt care conține un grup candidat, dar lipsește din baza de date, e convertit doar de regulile de rezervă și e marcat. Cu comutatorul -eẹ/aẹ/oẹ e citit ca singular, cu excepțiile de mai jos.

Un cuvânt cu o terminație de adjectiv în -iesc după i (`necunoscute.yaml: iesc`: -iești, -iescu, -iescul, -iescului, -ieștii, -ieștile, -ieștilor, -iască, -iasca) e legat întâi de lema lui: terminația e înlocuită cu -esc, iar dacă rezultă o lemă în -iesc (`iesc_lemmas`), cuvântul e derivat din despărțirea lemei, inclusiv regula sufixului. dexonline are unele dintre aceste adjective fără forme: *harabagiasca* → *harabagiėsca*, *harabagiască* → *harabagiėscă*, *harabagiești* neschimbat (← *harabagi-esc*).

Genitivul și pluralul articulat al unui nume din `ee.yaml: nume` pe care îl dă dexonline sunt în bază (vezi *Date*). Un cuvânt în -eei, -eele sau -eelor care e genitivul sau pluralul articulat al unui nume care lipsește din dexonline (-eei, -eele, -eelor finale înlocuite cu -eea sau -ee) primește o singură grafie, a numelui, și e marcat: *Cloei* → *Cloẹi*; într-o bază fără *Leea*, *Leei* → *Leẹi*.

Un cuvânt al unei leme din `ee.yaml: ae_oe`, chiar lema sau lema în -oe plus -a, -i, -le sau -lor, urmează regula -eẹ/aẹ/oẹ ca o formă a ei: *Zoei* → *Zoẹi*, *Cloei* → *Cloẹi*. Orice alt cuvânt în -ae/-oe rămâne neschimbat și nu e căutat (*vitae*). Un cuvânt în -oae se scrie -ōẹ (*zzoae* → *zzōẹ*).

Orice alt cuvânt în -eei e citit întâi ca genitivul unui cuvânt cunoscut: -eei final se înlocuiește cu -eea, apoi cu -ee, iar primul cunoscut e lema lui: cel în -eea trebuie să fie lemă (`ea_lemmas`), cel în -ee orice formă (`forms`). Regula -eei dă atunci o singură grafie: *Niceei* → *Niceii* (← *Niceea*), *aleei* → *aleii* (← *alee*), *aceei* neschimbat (← *aceea*, vezi `ee.yaml: eea_neschimbate`). *ideea* e doar o formă a lui *idee*, deci nu e lemă în -eea; *ideei* → *ideii* vine din *idee*. Masculinul unei leme în -ee nu e căutat: toate formele lemelor masculine sau adjectivale sunt în bază. Dacă niciunul nu e cunoscut, cuvântul primește două grafii, cu -eẹi (implicit, ca la o lemă păstrată) sau cu -eii, afișate pe comutatorul de omografe: *Zzeẹi* sau *Zzeii*.

Un cuvânt în -euri/-eurile/-eurilor devine -eẹ/-eẹle/-eẹlor când cuvântul fără -ri/-rile/-rilor e o lemă în -eu (`eu_lemmas`) de mai mult de 4 litere, ca la o formă cunoscută. Altfel rămâne neschimbat.

Un cuvânt în -eele/-eelor:
- -le/-lor final se înlocuiește cu -a. Dacă rezultă o lemă în -eea din baza de date (`ea_lemmas`), care nu e în `ee.yaml: eea_neschimbate`, cuvântul e pluralul ei, cu o singură grafie, ca *femeile*: *Iudeelor* → *Iudeilor*, *Galileele* → *Galileile* (nu prin falsele masculine *iudeu*, *Galileu*). Un plural al lui *coree* (`ee.yaml: fara_masculin`) se citește la fel: *coreelor* → *coreilor* (nu prin *coreu*).
- Altfel, -ele/-elor final se înlocuiește cu -u. Dacă rezultă o lemă în -eu din baza de date (`eu_lemmas`), cuvântul e pluralul ei și are o singură grafie: *bombeele* → *bombeẹle* (← *bombeu*, al cărui plural în dexonline este *bombeuri*).
- Altfel primește două grafii, ca plural al unei leme în -ee fără masculin (*coloboleile*, *coloboleilor*, implicit) sau al unei leme păstrate, în -eu sau în -ee cu masculin (*coloboleẹle*, *coloboleẹlor*). Ele apar pe comutatorul de omografe, cu o notă de subsol care folosește rădăcina cuvântului: „Dacă singularul e *colobolee* și nu are masculinul *coloboleu*, alege forma *coloboleile/lor*. Dacă e *coloboleu*, sau *colobolee* cu masculinul *coloboleu*, alege *coloboleẹle/lor*.”

Cuvintele fără grup candidat, fără iăr și fără rădăcină învechită nu sunt căutate deloc, nici numele latine (vezi *Text neconvertit*).

## Conversia inversă

Conversia concis → standard este deocamdată o înlocuire de caractere (ẹ→ie, ė→ia, ē→ea, ō→oa), cu excepția lui ė din -iėză/-iėscă/-iėsca final, care redevine a (*studiėză* → *studiază*, *sfiėscă* → *sfiască*, *fiėsca* → *fiasca*). Asta e valabil doar cu regulile actuale și nu e o proprietate garantată. E inexactă pentru *fiiască*, *fiiasca* → *fiėscă*, *fiėsca*, care redevin *fiască*, *fiasca*. Un infinitiv lung în -iare nu e refăcut: *apreciere* rămâne *apreciere*. î rămâne neschimbat, deci â nu e refăcut după regula â → î (*cîine* rămâne *cîine*).

Cu comutatorul -eẹ/aẹ/oẹ pornit, textul e citit ca scris cu regula -eẹ/aẹ/oẹ. Un cuvânt e căutat întâi printre grafiile pe care regula le dă formelor din bază (`ee_revert`) și revine la forma din care provine: *orhideẹ* → *orhidee*, *orhideile* → *orhideele*, *ideė* → *ideea*, *bombeẹle* → *bombeurile* (dexonline nu are *bombeele*), *Naẹ* → *Nae*, *aloẹ* → *aloe*, *Băbōẹ* → *Băboae*. Mai multe forme din dexonline pot da aceeași grafie, prin regulă sau neschimbate (*zeẹi*: forma *zeiei*, sau *zeei* prin regulă). Ele sunt restrânse în trei pași:
1. o formă a unui lexem DEX-MDA-DOOM bate una fără. Lexemul trebuie să aibă o definiție proprie (al cărei titlu e chiar el) din MDA\*, DEX\*, NODEX\* sau DOOM\*; o mențiune în definiția altui cuvânt nu contează. *aleẹ* → *alee* (nu *aleie*, doar în Scriban), *cheẹ* → *cheie* (nu *chee*), *acneẹ* → *acnee* (*acneie* e doar pomenit la *coș* în MDA2);
2. între formele DEX-MDA-DOOM, un plural în -ee/-eele/-eelor bate unul în -euri/-eurile/-eurilor: *apogeẹ* → *apogee* (nu *apogeuri*).
3. un plural în -eie, -eiele, -eielor care e DEX-MDA-DOOM bate pe cel în -ee al aceleiași leme: *curcubeẹ* → *curcubeie*, *curcubeẹle* → *curcubeiele* (nu *curcubee*, *curcubeele*); nu și pe al altei leme (*bordeẹ*: *bordeie*, al lui *bordei*, sau *bordee*, al lui *bordeu*).

Formele rămase la egalitate sunt oferite toate pe comutatorul de omografe (vezi *Omografe*). Cea implicită e rezultatul înlocuirii de caractere, altfel prima în ordinea alfabetului românesc: *bordeẹ* → *bordeie* (pluralul lui *bordei*) sau *bordee* (pluralul lui *bordeu*); *beẹ* → *beie* sau *bee*. O grafie a regulilor -ae/-oe/-oae, sau un genitiv ori plural păstrat în -eẹi/-eẹle/-eẹlor, a cărei citire prin înlocuirea de caractere (în -aie, -oaie, -eie) e o formă din dexonline nu se decide așa: dacă sursa ei e un nume (cu majusculă în dexonline și nu în -oae), sursa e implicită, iar cuvântul în -aie alternativa (*Naẹ* → *Nae* sau *naie*, *Nicolaẹ* → *Nicolae* sau *Nicolaie*); altfel sursa e o grafie veche, iar cuvântul revine la cel în -aie, fără comutator (*baẹ* → *baie*, *Fōẹ* → *foaie*, *oboẹ* → *oboie*, *zeẹi* → *zeiei*, *zeẹle* → *zeiele*, *aloii* → *aloii*).

O grafie care lipsește din `ee_revert` și pe care regula o dă unui nume din `ee.yaml: nume` sau unui cuvânt din `ee.yaml: ae_oe` revine la acea formă, chiar dacă forma lipsește din bază, afară de cazul în care citirea ei prin înlocuirea de caractere e un cuvânt cunoscut: *Andreẹi* → *Andreei*, *Leẹi* → *Leei*, *Andreẹlor* → *Andreelor*, *Zoẹi* → *Zoei*, *Cloẹ* → *Cloe*, la fel și o astfel de grafie cu celelalte grupuri lăsate cum sunt (*Neagoẹ*, ca *Nēgoẹ* → *Neagoe*), dar *Mamaẹi* → *Mamaiei*, *laẹ* → *laie*.

Orice alt cuvânt, inclusiv cele necunoscute, trece prin înlocuirea de caractere (*femeẹ* → *femeie*, *zzeẹ* → *zzeie*, *Zzeẹi* → *Zzeiei*). Căutarea nu ține cont de regula â → î, așa că un cuvânt cu â în bază revine prin înlocuirea de caractere după ea (*vîntureẹ* → *vîntureie*, nu *vânturee*).

## Fișierele de reguli

Listele făcute de mână ale regulilor sunt fișiere YAML din `reguli/`, pe care le editează un lingvist. Ele sunt compilate în nucleu când e construit, așa că aplicațiile nu citesc YAML. O listă e numită după fișierul și cheia ei: `ee.yaml: nume`.

| Fișier | Cheie | Ce conține | Fiecare cuvânt |
|---|---|---|---|
| `silabisiri.yaml` | (un cuvânt) | despărțirile în silabe ale lui Cezar, `cuvânt: silabisire` sau `cuvânt: [silabisire, ...]`, cea implicită prima (vezi *Sursa despărțirii în silabe*) | are literele cuvântului și nicio silabă goală |
| `invechite.yaml` | `lemme` | lemele învechite, în ordine, fiecare cu `lema` (despărțită în silabe), `actuala`, `tulpina` și `fara_forme` (`true` sau `false`; implicit `false`), vezi *Forme învechite* | `tulpina` e începutul lui `lema`; cu `fara_forme: true`, `actuala` are cel puțin atâtea litere câte are `tulpina` |
| | `iare_pastrate` | lemele în -iare care nu sunt infinitive lungi (vezi *Sursa despărțirii în silabe*) | se termină în -iare |
| `ee.yaml` | `nume` | numele feminine de persoane (vezi *Regula opțională -eẹ/aẹ/oẹ*) | se termină în -eea, -ee sau -oe |
| | `ae_oe` | cuvintele în -ae/-oe ale regulii -eẹ/aẹ/oẹ; numele în -oe din `nume` contează ca fiind ale ei și nu sunt scrise în ea | se termină în -ae sau -oe; nu e un nume în -oe din `nume` |
| | `eea_neschimbate` | cuvintele-titlu în -eea care păstrează -eē | se termină în -eea |
| | `fara_masculin` | femininele în -ee a căror sosie în -eu nu e masculinul lor | se termină în -ee |
| `neconvertite.yaml` | `nume_latine` | numele latine lăsate cum sunt (vezi *Text neconvertit*) | se termină într-o vocală + -ian |
| `a-i.yaml` | `radacini_cu_a` | rădăcinile ale căror cuvinte își păstrează â (vezi *Regula opțională â → î*) | are â |
| `necunoscute.yaml` | `iesc` | terminațiile unui adjectiv în -esc după i (vezi *Cuvinte necunoscute*) | |

Formatul:
- YAML 1.2, în UTF-8, cu sau fără marcaj de ordine a octeților (BOM). Un fișier asociază cheilor lui valorile lor (`silabisiri.yaml` asociază cuvinte), iar fiecare cheie e obligatorie. O listă se scrie `[a, b]` sau câte un `- a` pe rând. Orice valoare e text: `n`, `no`, `on` și `true` sunt cuvinte, nu valori booleene.
- Cheile sunt ASCII cu litere mici și `_`. Cuvintele se scriu cu litere mici, în ortografia standard: NFC, ș/ț cu virgulă, â în interiorul cuvintelor.
- Ordinea se păstrează acolo unde contează: silabisirile unui cuvânt și lemele din `lemme`, prima fiind cea implicită.
- `#` începe un comentariu. Comentariile explică; nu schimbă nicio grafie, iar regulile fișierului de date nu le cuprind (vezi *Date*).
- Ancorele (`&`), etichetele (`!`) și un al doilea document (`---`, `...`) nu se folosesc.

Verificările la încărcare rulează când e construit nucleul, pe care o eroare îl oprește, și cu `cargo run -p grafia-reguli`, care în CI arată și fiecare mesaj pe rândul lui din pull request. Un mesaj dă fișierul, rândul și coloana, în română: `reguli/ee.yaml:3:30: eroare: „zoş” are ş cu sedilă; scrie ș, cu virgulă.` Ele verifică:
- sintaxa YAML; cheile, niciuna necunoscută, repetată sau lipsă; tipul fiecărei valori (o listă, un cuvânt, `true` sau `false`) și că niciuna nu e goală, o valoare goală fiind raportată la cheia ei; că `silabisiri.yaml` nu e gol; și că fiecare fișier YAML din `reguli/` e un fișier de reguli (celelalte fișiere, ascunse sau care nu sunt .yaml/.yml, ca .DS_Store sau copia de rezervă a unui editor, sunt lăsate în pace);
- caracterele: o eroare pentru ş/ţ cu sedilă, ã, o literă descompusă, o majusculă, un spațiu, o cratimă în afara unei silabisiri și orice alt caracter din afara alfabetului românesc; un avertisment pentru î în interiorul unui cuvânt (*neînțeles* e corect);
- niciun cuvânt de două ori într-o listă, nicio silabisire de două ori la un cuvânt, nicio lemă de două ori în `lemme`;
- fiecare cuvânt așa cum spune tabelul de mai sus.

## Date

Datele despre cuvinte vin într-un singur fișier, `grafia.dat` (structura lui e în `crates/core/src/datafile.rs`). Pe lângă cinci tabele, conține versiunea formatului, ziua la care sunt datele dexonline din care a fost construit (ziua ultimei modificări a unui lexem sau a unei definiții în dexonline), regulile cu care a fost construit și o sumă de control, verificată la deschiderea fișierului. Regulile sunt `RULES_HASH`, o sumă (hash) a codului din fișierele din `crates/core/src/rules/` și din `build.rs` și a codului în care sunt compilate fișierele de reguli (valorile lor, nu comentariile; vezi *Fișierele de reguli*), fără comentarii și spații: construirea aplicației web refuză un fișier de date construit cu alte reguli. Fiecare tabel are cheia un cuvânt normalizat:
- `forms`: o formă → `concise`, grafia ei concisă; `concise_ee`, grafia ei cu regula -eẹ/aẹ/oẹ, când diferă de `concise`; `alternatives`, celelalte grafii ale unui omograf; și `current`, forma actuală a unei forme învechite (vezi *Forme învechite*).
- `eu_lemmas`: toate lemele normalizate în -eu, pentru pluralele necunoscute în -eele/-eelor (vezi *Cuvinte necunoscute*).
- `ea_lemmas`: toate lemele normalizate în -ea, pentru genitivele necunoscute în -eei (vezi *Cuvinte necunoscute*).
- `iesc_lemmas`: toate lemele normalizate în -iesc → despărțirea lor (*harabagi-esc*), pentru formele necunoscute ale adjectivelor în -iesc (vezi *Cuvinte necunoscute*).
- `ee_revert`: o grafie cu regula -eẹ/aẹ/oẹ → forma la care revine și celelalte forme ale unei egalități, doar unde înlocuirea de caractere singură nu dă răspunsul (vezi *Conversia inversă*).

Aceleași tabele dau mereu același fișier, octet cu octet.

Fișierul e construit dintr-un export al înregistrărilor din dexonline: intrarea pipeline-ului descrisă mai jos, cuvintele pe care le poate găsi regula prefixelor și ziua datelor. Exportul e publicat ca arhivă într-o versiune (release) a depozitului, `dexonline-AAAA-LL-ZZ` (ziua datelor), iar `dexonline.lock` îl fixează prin suma lui SHA-256. Fișierul nu e salvat în depozit: CI-ul depozitului îl construiește din exportul fixat, cu regulile pe care le publică apoi împreună cu el, și la fel oricine, cu `cargo xtask data`. Același export și aceleași reguli dau același fișier, octet cu octet. Doar o actualizare dexonline are nevoie de baza de date dexonline însăși, pentru un export nou.

`forms` păstrează doar formele care pot diferi: cele care conțin ie/ia/ea/oa sau iăr, ori se termină în -ee, -eea, -eei, -eele, -eelor, -euri, -eurile, -eurilor sau -oae, formele lemelor din `ee.yaml: ae_oe` (*Nae*, *aloei*), formele lemelor învechite (*alineez*) și, construite din `ee.yaml: nume`, genitivul și pluralul articulat (-i, -le, -lor adăugate numelui, fără -a final) al fiecărui nume din listă pe care dexonline îl dă fără ele (*Andreei*, *Andreele*, *Andreelor* ← *Andreea*; *Zoei* ← *Zoe*). Intrarea pipeline-ului pentru fiecare formă flexionară este: forma, lema ei, despărțirea lemei din dexonline (completă, parțială sau goală), numărul (S/P), dacă lema e verb (modelul V sau VT din dexonline), dacă e substantiv feminin sau neclasificată, scrisă cu literă mică (modelul F, T sau IL; pentru infinitivele lungi în -iare) și dacă e DEX-MDA-DOOM: are o definiție activă proprie (al cărei titlu e chiar lexemul) dintr-o sursă al cărei nume scurt începe cu MDA, DEX, NODEX sau DOOM (MDA2, DEX '09, NODEX, DOOM 3, ...; nu dicționarul intern *dexonline*), flexiunea și varianta formei, care împerechează formele unei leme învechite cu cele ale lemei ei actuale, și dacă lema e substantiv masculin sau adjectiv (modelul M sau A), ceea ce face dintr-o lemă în -eu masculinul femininului ei în -ee (*zeu*, *zee*).
