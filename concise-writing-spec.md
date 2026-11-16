# grafia concisă — specification (v0.3)

This file holds the complete current rules, with no history. Code and tests follow it. Why each rule was chosen, and when, is in `DECISIONS.md`, which is the authority on intent: a disagreement between the two is a defect, fixed here in the same change. `README.ro.md` renders this file in Romanian.

## Rule

| Standard | Concis | Code point (lower / upper) |
|---|---|---|
| ie | ẹ | U+1EB9 / U+1EB8 |
| ia | ė | U+0117 / U+0116 |
| ea | ē | U+0113 / U+0112 |
| oa | ō | U+014D / U+014C |

Concision applies whenever no hiatus is involved, i.e. both vowels are in the same syllable. Stress plays no role, and there are no special cases after ce/ci/ge/gi (*ceasornic* → *cēsornic*).

Within a syllable the scan runs left to right and matches do not overlap: *leoarcă* → *leōrcă*, *aripioară* → *aripiōră*.

A word form may have more than one correct concise spelling. It always has one default (*ceai* → *cēi*). A form with two syllabifications (a homograph) also carries its other spelling, and the user can switch to it (see *Homographs*). The optional -eẹ/aẹ/oẹ rule is the exception: it gives one output per form.

## Syllabification source

A headword uses its own syllabification. Any other inflected form takes its cuts from its lemma's syllabification (see the derivation table below).

A lemma's syllabification is Cezar's explicit one when listed (`silabisiri.yaml`: *neant* `ne-ant`, *mielea* `mi-e-lea`, *înfieri* `în-fi-eri`, *răspieri* `răs-pieri`, ...; an entry may list several readings, the first being the default: *miază* `mia-ză` / `mi-a-ză`, *vier* `vier` / `vi-er`), otherwise dexonline's hyphenation completed by fallback rules. The word *ie* has only the reading `i-e` and stays *ie*, with no alternative. A form listed there takes all its spellings from there, whatever its lemma, the default first: *piei* `piei` / `pi-ei` is *pẹi* (← *piele*, *pieri*), then *piei* (← *pia*), although by its records *piei* ← *pia* `pi-a`, which dexonline backs, would be its only spelling.

- **dexonline hyphenation.** Only the first variant is used. It may be complete (`fa-mi-li-e`), a word end (`-li-e`), a word start (`a-ce-tal-`) or a word interior (`-pioa-`). Inside the span it covers, its cuts always prevail over the rules. A hyphenation that does not fit the word is ignored.
- **Fallback rules.** Everywhere else, the rules place only cuts between two vowels, since grafia concisă uses no other cuts. They are the steps of `pasi.yaml`, tried in order for each pair of vowels: the first that applies decides. A step that consults an exception table applies only where an entry of the table applies too; otherwise the next step is tried. Below, L is a labial (p, b, f, v, m) and P a palatal (c, g, ch, gh), the classes of `clase.yaml`. The rules are known to be incomplete and will be refined:

| Step | Vowels | Context | Cut | Examples |
|---|---|---|---|---|
| e-e | e-e | anywhere | hiatus | *idee* → *ide-e* (the -eẹ/aẹ/oẹ switch reads it as *e-ie*), *aceea* |
| nu e prefix | o-a | the word starts like a prefix followed by a known word, but has no prefix: *coard*, *coast*, *coacere*, *coacăț* (co- + *ardă*, *astă*, ...); a word with neo-, pseudo- or another prefix would go here too | diphthong | *coardă*, *coastă*, *coacere*, *coacăț*, *coacăță* (not *coactiv*) |
| prefix și un cuvânt cunoscut | the prefix's last vowel and the next | after a prefix (re-, pre-, ne-, tele-, co-, semi-, anti-, multi-, auto-, micro-, macro-, foto-, pseudo-, bio-, neo-, radio-, aero-, agro-, electro-, hidro-, termo-, geo-, proto-, socio-, psiho-, stereo-) when the rest is a known word: a form of a lowercase dexonline lexeme with frequency ≥ 0.9, of at least 4 letters; not when the word starts with the prefix *prea-*, a prefix of its own, followed by a known word | hiatus | *re-abilitat*, *ne-adevărat*, *auto-analiză*, *co-activ*, *pre-aviz*, *pre-încălzi* (pre- + *încălzi*) (not *neam*, *neagră*, *autoare*; *preacurat* is prea- + *curat*, one syllable) |
| -ient | ie | in -ient, -ienț, followed by a, ă, e, i or the word's end | hiatus | *client*, *ambient*, *experiență*, *clientelă* (dexonline: *farniente* `far-nien-te`) |
| -abil | ia, ea | before -bil, after a consonant | hiatus | *fiabil*, *agreabil* (not *debreiabil*: after a vowel, like *de-bre-ia*) |
| bi-en | ie | word-initial *bien* | hiatus | *bienal* (not *biet*) |
| ea în neologisme | ea | neologism endings and stems (`hiat-ea.yaml`): final -eal (not after g/d/b), -linear/-hlear/-balnear, -laureat-/-lineat-/-nucleat- and similar (whatever follows), nuclear-, creat-, reac(t/ț)-, steat-, meandr-, aleat-, beatif-, reali-/ideali-/boreal (not in -răboreală/-stoboreală)/cereal | hiatus | *real*, *leal*, *interliceal*, *boreal*, *hiperboreală*, *nuclear*, *denuclearizat*, *linear*, *alineat*, *anucleată*, *laureată*, *delineator*, *creator*, *pancreatic*, *reactor*, *reacție*, *steatoză*, *meandru*, *aleator*, *beatific*, *realizat*, *cereale* (not *deal*, *Ardeal*, *mangeal*, *beat*, *asear*, *greșeală*, *răboreală*, *cleat*, *procleat*, *pleată*, *soreată*) |
| oa în neologisme | oa | neologism stems and endings (`hiat-oa.yaml`): coagul-, coabit-, coaliț-/coaliz-, final -zoar | hiatus | *coagulat*, *coabita*, *coaliție*, *coalizat*, *protozoar*, *cazoar* (not *mozoară*) |
| -ii final la verbe | i-i | word-final, after a consonant, in a verb infinitive | hiatus | *sfi-i*, *pri-i*, so *sfiește*, *priască* keep i-e/i-a (not the plural *copii*) |
| ie, ia la început sau după vocală | ie, ia | word start, or after a vowel (ă, â, î included) | diphthong | *iarbă*, *femeie*, *tăia*, *tăiere*, *înmuiere* |
| ie, ia final după consoană | ie, ia | word-final, after a consonant | hiatus | *familie*, *vie*, *istoria* |
| -iare care nu e infinitiv lung | ia | final -iare after L or P, in a word that is no long infinitive (see below) | diphthong | *fiare* (like *fiere*), *ghiare* (like *ghiară*) |
| tulpini native cu ia | ia | after a consonant, in native stems (`diftong-ia.yaml`) | diphthong | *chiar*, *piatră*, *biată*, *viață*, *amiază*, *ghiață*, *ghiarele*, *maghiar*, *Caragiale*, *chiajna*, *obială* (not *liturghiar*, *eleghiacesc*, *schiagramă*, *Gorghias*, *Miaulis*, *microbial*) |
| ia după consoană | ia | medial, after a consonant (the -ia\* endings) | hiatus | *apropiat*, *pian*, *social*, *specializat*, *glaciar*, *variare* |
| -iere nativ | ie | *miere*, *înmiere*, *fiere*, *piere*, and their forms in -ierea, -ieri, -ierii, -ierile, -ierilor | diphthong | *miere*, *mierea*, *înmiere*, *fiere*, *pieri* |
| -iere, infinitive lungi | ie | final -iere, -ierea, -ieri, -ierii, -ierile, -ierilor after L or P (long infinitives; after a vowel it is a diphthong, see above: *tăiere*) | hiatus | *copiere*, *apreciere*, *veghiere*, *învierea*, *aprecierilor* |
| -ier, -ieră, -iereasă | ie | final -ier, -ieră, -iereasă and the forms of -iereasă, after L or P, in a word of more than one syllable | hiatus | *bărbier*, *plumier*, *banchier*, *genunchier*, *legumieră*, *rachiereasă*, *pompiereasă* (not *fier*, *zbier*) |
| -iesc | ie | final -iesc (adjectives) after L or P | hiatus | *fiesc*, *zapciesc*, *surugiesc* |
| ie în neologisme | ie | after L or P, in neologism stems and endings (`hiat-ie.yaml`): gien, pieton, cviet, societ, soviet, vien, miel + o/in/it/ie, -ietor/-ietoare, bier neither word-initial nor after z/s | hiatus | *igienă*, *pieton*, *sovietism*, *cvietism*, *vienez*, *mielită*, *poliomielită*, *așchietor*, *bărbierit*, *corăbierie* (not *miel*, *mielușor*, *zbiera*) |
| ie după labială sau palatală | ie | after L or P | diphthong | *fier*, *piept*, *pieptene* |
| ie după altă consoană | ie | after another consonant | hiatus | *prieten*, *cartier* |
| restul | anything else (ea, oa, ...) | anywhere | diphthong | *seară*, *soare* |

When several records produce the same form, the default spelling is chosen by priority (the other spellings become alternatives, see *Homographs*):
1. Any record beats one of a long infinitive in -iare (see below).
2. A syllabification backed by an explicit, dexonline or partial one beats rules alone, even for a headword: *agreare* is *agreare* (← *agrea* `a-gre-a`), not the headword *agreare* by rules (*agrēre*).
3. A headword beats a derived form.
4. An explicit syllabification beats dexonline, and a lemma syllabified completely by dexonline beats a partial one.
5. Otherwise the first record wins: that of the lexeme added to dexonline first, then of its form added first (by their ids).

Cuts are derived from the lemma one vowel pair at a time:

| Vowel pair | Decided by | Examples |
|---|---|---|
| both letters in the prefix shared with the lemma | the lemma's cut | *soarele* → *sōrele* (← *soa-re*), *reală* unchanged (← *re-al*) |
| ending replaces a lemma vowel at the same place, after i | the lemma's cut | *familia* unchanged (← *fa-mi-li-e*), *femeia* → *femeė* (← *fe-me-ie*) |
| ending replaces the final u of an -iu lemma | hiatus | *argintie*, *argintia* unchanged (← *ar-gin-tiu*), *cheflie* (← *che-fliu*) |
| ending appended after the final i of a verb in consonant + i | hiatus | *știa*, *știam*, *știau* unchanged (← *ști*), *neștie* (← *ne-ști*); not after a vowel (*locuia* → *locuė* ← *lo-cu-i*) |
| appended ending, stem alternation, or o/e before a replaced vowel | diphthong | *ideea* → *ideē*, *creează* → *creēză*, *doarme* → *dōrme*, *ploaie* → *plōẹ* (← *plo-ua*), *scheaună* → *schēună* (← *sche-u-na*) |

The suffix -ează/-ească of verbs and adjectives after a hiatus i (written -ază/-ască/-asca) keeps an e form, as in the other forms (*studiez*, *studiezi*; *sfiesc*, *sfiești*; *fiesc*, *fiești*), and is written -ėză/-ėscă/-ėsca:

| Form | Lemma | Result |
|---|---|---|
| lemma + -ză | ends in a hiatus i-a (a verb of the first conjugation) | *studiază* → *studiėză* (← *stu-di-a*), *pliază* → *pliėză*, *înfiază* → *înfiėză*, *remaniază* → *remaniėză* |
| lemma minus final i + -ască | ends in a hiatus i-i (a verb of the fourth conjugation) | *sfiască* → *sfiėscă* (← *sfi-i*), *priască* → *priėscă* |
| lemma minus -esc + -ască, -asca | ends in a hiatus i-esc (an adjective) | *fiască* → *fiėscă*, *fiasca* → *fiėsca* (← *fi-esc*), *pustiască* → *pustiėscă* |

A long infinitive in -iare is not standard and is written in its -iere form, which then follows the rules for -iere: *apreciare* → *apreciere*, *apreciarea* → *aprecierē*, *tăiare* → *tăẹre*. In its forms, the last -iar-/-iăr- becomes -ier-: *apreciărilor* → *aprecierilor*. A long infinitive is a lemma in -iare that dexonline lists in lowercase as a feminine noun or unclassified (model F, T or IL), except *fiare* and *mobiliare* (`invechite.yaml: iare_pastrate`). A form that another lemma also gives keeps that lemma's spelling: *conciliare* (the adjective *conciliar*), *copiare* (the noun *copiar*). The -iere spelling is then no alternative on the homograph switch. Otherwise the long infinitive is an obsolete form, and its own spelling is the alternative (see *Obsolete forms*).

Other obsolete forms are written as their current form too (see *Obsolete forms*).

## Optional rule: -eẹ/aẹ/oẹ (switch, default on)

Lemmas in *-ee* follow the *femeie* pattern: word-final *-ee* is read as *-eie* and written *-eẹ*, the same as *femeie* → *femeẹ*. Final *-ae* and *-oe* are read *-aie*, *-oie* and written *-aẹ*, *-oẹ*.

A lemma is *kept* when it ends in -eu, when it ends in -ee and has a masculine, or when it is a feminine name of a person in `ee.yaml: nume` (*Andreea*, *Medeea*, *Leea*, *Zoe*, *Cloe*, ...). A kept lemma keeps the e of its genitive and plural, written *ẹ*, instead of following *femeii*, *femeile*. The masculine of an -ee lemma is the same word in -eu, if dexonline lists it as a masculine noun or an adjective (model M or A): *zee* ← *zeu*, *atee* ← *ateu*, *pigmee* ← *pigmeu*; not a neuter (*propilee*: *propileu*, *atenee*: *ateneu*), nor another word (`ee.yaml: fara_masculin`: *coree*, the disease, and *coreu*, the metrical foot).

| Form | Condition | Result |
|---|---|---|
| singular in -ee | always | *idee* → *ideẹ*, *alee* → *aleẹ* |
| plural in -ee | kept lemma | *bombee* (← *bombeu*) → *bombeẹ*, *zee* (← *zee*, with *zeu*) → *zeẹ* |
| plural in -eele, -eelor | kept lemma | *bombeele* → *bombeẹle*, *liceelor* → *liceẹlor*, *zeele* → *zeẹle*, *ateelor* → *ateẹlor*, *Andreelor* → *Andreẹlor* |
| plural in -euri, -eurile, -eurilor | the form minus -ri/-rile/-rilor is its -eu lemma, of more than 4 letters | *bombeuri* → *bombeẹ*, *bombeurile* → *bombeẹle*, *liceurilor* → *liceẹlor* (not *pneuri*, *zeurile*) |
| plural in -ee | any other lemma in -ee | *logoree* → *logorei* (like *femeie* / *femei*), *propilee* → *propilei* |
| plural in -ee | any other lemma | unchanged |
| final -eea | lemma ends in -ee or -eu | *ideea* → *ideė*, *orhideea* → *orhideė* (like *femeia*), *zeea* → *zeė*, *spontaneea* → *spontaneė* (← *spontaneu*); the base spelling is *ideē* (*i-de-ea*) |
| final -eea | lemma ends in -eea (headword), not `ee.yaml: eea_neschimbate` | *Niceea* → *Niceė*, *Andreea* → *Andreė*, *Galileea* → *Galileė*; also an unknown word. The pronouns *aceea*, *ceea*, *oareceea* are left to the base rules (*aceē*) |
| final -eei | lemma as for -eea, kept | *zeei* → *zeẹi*, *ateei* → *ateẹi*, *spontaneei* → *spontaneẹi*, *Andreei* → *Andreẹi*, *Medeei* → *Medeẹi* |
| final -eei | lemma as for -eea, not kept | *ideei* → *ideii*, *orhideei* → *orhideii* (like *femeii*), *Galileei* → *Galileii* |
| final -eele | any other lemma in -ee, or in -eea and no name | *orhideele* → *orhideile* (like *femeile*), *Galileele* → *Galileile* (an unknown word) |
| final -eelor | any other lemma in -ee, or in -eea and no name | *orhideelor* → *orhideilor* (like *femeilor*), *Iudeelor* → *Iudeilor* (an unknown word) |
| final -oae | any word | *Băboae* → *Băbōẹ*, *lighioae* → *lighiōẹ* |
| final -ae, -oe | lemma in `ee.yaml: ae_oe` | *Nae* → *Naẹ*, *Nicolae* → *Nicolaẹ*, *Zoe* → *Zoẹ*, *Noe* → *Noẹ*, *aloe* → *aloẹ* (not *vitae*, *reggae*, *Poe*) |
| final -oea | lemma in -oe in `ee.yaml: ae_oe` | *aloea* → *aloė* (like *ideea*) |
| final -oei, -oele, -oelor | lemma in -oe in `ee.yaml: ae_oe`, not in `ee.yaml: nume` | *aloei* → *aloii*, *aloele* → *aloile*, *aloelor* → *aloilor* (like *femeii*, *femeile*, *femeilor*) |
| final -oei, -oele, -oelor | lemma in -oe in `ee.yaml: ae_oe` and `ee.yaml: nume` | *Zoei* → *Zoẹi*, *Cloei* → *Cloẹi*, *Zoele* → *Zoẹle*, *Zoelor* → *Zoẹlor* |

The forms of an -eie lemma need no rule: the base rules already write them with *ẹ* (*zeiele* → *zeẹle*, *zeiei* → *zeẹi*, *rotofeiele* → *rotofeẹle*).

`ee.yaml: ae_oe` lists the words in -ae/-oe the rule applies to, and the names in -oe of `ee.yaml: nume` count as its words without being written in it. Most dexonline words in -ae/-oe are foreign (*vitae*, *personae*, *reggae*, *Poe*, *Crusoe*). It leaves out the words with a foreign spelling: a *y*, *th*, *ph*, *rh*, or a *ch* not followed by e/i (*Pasiphae*, *Callirhoe*).

If one spelling is both a singular and a plural of an -ee lemma, the singular reading wins, so each form still gets one output.

With the switch on, reverse conversion looks the words up to undo the rule (*ideẹ* → *idee*, *orhideii* → *orhideei*, *Naẹ* → *Nae*; see *Reverse conversion*).

## Optional rule: â → î (switch, default on)

â is written î (Â as Î), except in words with the root *român* or *rumân* (`a-i.yaml: radacini_cu_a`), i.e. any word containing *român* or *rumân*: *câine* → *cîine*, *când* → *cînd*, *pâinea* → *pîinē*; unchanged: *român*, *românesc*, *România*, *aromân*, *dacoromân*, *rumân*, *rumânesc*.

It applies to every word after the rest of the conversion, whether the word is in the database or not, and never marks a word as unknown. It applies to every spelling of a homograph.

## Homographs

A form whose records give different spellings is a homograph. Examples:
- *ia*: *ė* (the verb *lua*, the interjection) or *ia* (the articulated *ie*, *i-a*);
- *iei*: *ẹi* (the verb *lua*) or *iei* (the genitive of *ie*, *i-ei*);
- *miază*: *mėză* (noon, *mia-ză*) or *miază* (myiasis, *mi-a-ză*);
- *curie*: *curẹ* or *curie*;
- *liază*: *liază* (the noun) or *liėză* (the verb *lia*);
- *vier*: *vẹr* (the boar, *vier*) or *vier* (păzitor de vie, *vi-er*), and so its forms (*vẹrul* or *vierul*), the vocative *viere* included: *vẹre* (*vie-re*) or *viere* (*vi-e-re*).

The default is the spelling chosen by the priority above. The others are stored in `forms.alternatives` and returned with it: in `segments[].alts` from the core's `convert` and in `alternatives` from its `lookup`, the default first in both. A spelling counts only when an explicit, dexonline or partial syllabification backs it, except that when one record gives the suffix -ėză/-ėscă/-ėsca (see *Syllabification source*), all the form's spellings count. `silabisiri.yaml` may give several readings for one word; the first is the default.

The UI shows every spelling of a homograph in one switch on a dark-grey track. The active spelling sits on the text area's background, and the others are light grey on the track. A click on one makes it active. Copying takes the active spellings.

The -eẹ/aẹ/oẹ rule does not produce homographs: its singular/plural choice stays single (see *Optional rule: -eẹ/aẹ/oẹ*). The exception is an unknown word in -eei, -eele or -eelor with no known lemma (see *Unknown words*). Reverse conversion with the -eẹ/aẹ/oẹ switch shows its ties on the same switch (see *Reverse conversion*).

## Obsolete forms

An obsolete form is written as its current form, and its own spelling is offered on the homograph switch, after the current one, with a footnote: „Forma *alineare* e învechită; forma actuală e *aliniere*.” In the core's `convert`, the segment's `obsolete` holds the word, then its current forms, in the word's case (*Alineare*, *Aliniere*); `lookup` returns the current forms in `obsolete`. The obsolete forms are:
- the long infinitives in -iare (see *Syllabification source*): *apreciarea* → *aprecierē* or *apreciarē*;
- the forms of the lemmas in `invechite.yaml: leme`, each listed with its current lemma: *alinea* → *alinia*, *alineare* → *aliniere*, *alini* → *alina*, *rachiareasă* → *rachiereasă*. A form takes the spelling of the current lemma's form with the same dexonline inflection and variant (*alineează* → *aliniėză* ← *aliniază*, *alineez* → *aliniez*), and its own spelling from its own syllabification, listed with it in `invechite.yaml: leme` (*a-li-ne-a*: *alineēză*, *alineare*; *ra-chi-a-rea-să*: *rachiarēsă*). Words that start with the stem of one (*aline-*, *rachiare-*) are looked up even with no candidate digraph (*alineez*).

A form that another lemma also gives keeps that lemma's spelling, with no switch: *alineat* (the noun), *conciliare* (the adjective *conciliar*).

A form of several obsolete lemmas with different current forms is not rewritten: the switch offers its own spellings, one per lemma, in the order of `invechite.yaml: leme`, and the footnote names every current form. *alineam*, the imperfect of both *alinea* and *alini*, is *alineam* (*a-li-ne-am*) or *alinēm* (*a-li-neam*): „Forma *alineam* e învechită; forma actuală e *aliniam* sau *alinam*.”

A word missing from the database that starts with the stem of an obsolete lemma that dexonline lists without its forms (`fara_forme: true` in `invechite.yaml: leme`: *rachiareasă*) takes the current lemma's stem and is converted by the rules alone (*rachiareasa* → *rachierēsa*); its own spelling comes from the obsolete lemma's syllabification (*rachiarēsa*). For the other obsolete lemmas, whose forms are all in the database, the stem would only catch other words, which are left as they are (*Alinei*, of *Alina*).

## Case

The case pattern of the word is carried over: *Soare* → *Sōre*, *SOARE* → *SŌRE*. Other mixed-case patterns are reduced to an initial capital.

## Input normalization

Text is normalized to NFC. For lookup, ş/ţ with cedilla are treated as ș/ț with comma below.

## Text not converted

The text between `[` and `]` is left as it is, byte for byte: no conversion, no â → î, no normalization. The brackets are dropped. It works in both directions: *Soare [fier ceai â] fier* → *Sōre fier ceai â fẹr*; *[fẹr] fẹr* → *fẹr fier*. The first `]` closes a span, so spans don't nest (*[a [fier] ceai]* → *a [fier cēi]*), and a `[` or `]` with no partner stays in the text as an ordinary character. An empty span (`[]`) vanishes. In the core's `convert` and `revert`, a span is a segment of its own with `literal: true` and `t` its inside; the rest is converted as usual, and the â → î switch skips the spans.

Names of Latin origin are not converted. A word is a Latin name when it is in `neconvertite.yaml: nume_latine` (names ending in a vowel + -ian, exact forms only: *Traian*) or when it is capitalized and ends in -us (*Troianus*, *Gordianus*, *Helianthus*). It is never looked up, so it is not flagged as unknown, and it keeps its case: *Traian*, *TRAIAN*. The list covers exact forms only, for now: its other forms (*Traiana*, *Traiană* → *Traėnă*) and the other names in a vowel + -ian (*Troian* → *Troėn*, *Stoian*) are converted, and a text keeps them with a `[ ]` span (*[Traiana]*). Forms may be added to the list later.

## Unknown words

A word that contains a candidate digraph but is missing from the database is converted by the fallback rules alone and flagged, unless it is listed in `silabisiri.yaml`: it then takes its first syllabification from there, with no switch, and is still flagged (in a database without *neant*, *neant* stays *neant*, not *nēnt*). With the -eẹ/aẹ/oẹ switch it is read as a singular, except as follows.

A word with an ending of an -iesc adjective after i (`necunoscute.yaml: iesc`: -iești, -iescu, -iescul, -iescului, -ieștii, -ieștile, -ieștilor, -iască, -iasca) is first tied to its lemma: the ending is replaced by -esc, and if that gives a lemma in -iesc (`iesc_lemmas`), the word is derived from the lemma's syllabification, suffix rule included. dexonline lists some of these adjectives with no forms: *harabagiasca* → *harabagiėsca*, *harabagiască* → *harabagiėscă*, *harabagiești* unchanged (← *harabagi-esc*).

The genitive and articulated plural of a name in `ee.yaml: nume` that dexonline lists are in the database (see *Data*). A word in -eei, -eele or -eelor that is the genitive or the articulated plural of a name missing from dexonline (final -eei, -eele, -eelor replaced by -eea or -ee) gets one spelling, the name's, and is flagged: *Cloei* → *Cloẹi*; in a database without *Leea*, *Leei* → *Leẹi*.

A word of a lemma in `ee.yaml: ae_oe`, itself or the lemma in -oe plus -a, -i, -le or -lor, follows the -eẹ/aẹ/oẹ rule as a form of it: *Zoei* → *Zoẹi*, *Cloei* → *Cloẹi*. Any other word in -ae/-oe is left as it is, and is not looked up (*vitae*). A word in -oae is written -ōẹ (*zzoae* → *zzōẹ*).

Any other word in -eei is first read as the genitive of a known word: final -eei is replaced by -eea, then by -ee, and the first that is known is its lemma: -eea must be a lemma (`ea_lemmas`), -ee any form (`forms`). The -eei rule then gives one spelling: *Niceei* → *Niceii* (← *Niceea*), *aleei* → *aleii* (← *alee*), *aceei* unchanged (← *aceea*, see `ee.yaml: eea_neschimbate`). *ideea* is only a form of *idee*, so it is no lemma in -eea; *ideei* → *ideii* comes from *idee*. The masculine of an -ee lemma is not looked for: every form of a masculine or adjective lemma is in the database. If neither is known, the word gets two spellings, -eẹi (the default, as for a kept lemma) or -eii, shown as a homograph switch: *Zzeẹi* or *Zzeii*.

A word in -euri/-eurile/-eurilor becomes -eẹ/-eẹle/-eẹlor when the word minus -ri/-rile/-rilor is a lemma in -eu (`eu_lemmas`) of more than 4 letters, as for a known form. Otherwise it is unchanged.

A word in -eele/-eelor:
- Final -le/-lor is replaced by -a. If that gives a lemma in -eea from the database (`ea_lemmas`), not in `ee.yaml: eea_neschimbate`, the word is its plural, with one spelling, like *femeile*: *Iudeelor* → *Iudeilor*, *Galileele* → *Galileile* (not by the false masculines *iudeu*, *Galileu*). A plural of *coree* (`ee.yaml: fara_masculin`) is read likewise: *coreelor* → *coreilor* (not by *coreu*).
- Otherwise, final -ele/-elor is replaced by -u. If that gives a lemma in -eu from the database (`eu_lemmas`), the word is its plural, with one spelling: *bombeele* → *bombeẹle* (← *bombeu*, whose dexonline plural is *bombeuri*).
- Otherwise it gets two spellings, as the plural of an -ee lemma with no masculine (*coloboleile*, *coloboleilor*, the default) or of a kept lemma, in -eu or in -ee with a masculine (*coloboleẹle*, *coloboleẹlor*). They are shown as a homograph switch, with a footnote using the word's stem: „Dacă singularul e *colobolee* și nu are masculinul *coloboleu*, alege forma *coloboleile/lor*. Dacă e *coloboleu*, sau *colobolee* cu masculinul *coloboleu*, alege *coloboleẹle/lor*.”

Words with no candidate digraph, no iăr and no obsolete stem are never looked up, and neither are the Latin names (see *Text not converted*).

## Reverse conversion

Concise → standard is currently a character map (ẹ→ie, ė→ia, ē→ea, ō→oa), except that the ė of a word-final -iėză/-iėscă/-iėsca reverts to a (*studiėză* → *studiază*, *sfiėscă* → *sfiască*, *fiėsca* → *fiasca*). This holds only under the current rules and is not a guaranteed property. It is inexact for *fiiască*, *fiiasca* → *fiėscă*, *fiėsca*, which revert to *fiască*, *fiasca*. A long infinitive in -iare is not restored: *apreciere* stays *apreciere*. î is left unchanged, so â is not restored after the â → î rule (*cîine* stays *cîine*).

With the -eẹ/aẹ/oẹ switch on, the input is read as written with the -eẹ/aẹ/oẹ rule. A word is first looked up among the spellings the rule gives to database forms (`ee_revert`), and reverts to the form it came from: *orhideẹ* → *orhidee*, *orhideile* → *orhideele*, *ideė* → *ideea*, *bombeẹle* → *bombeurile* (dexonline has no *bombeele*), *Naẹ* → *Nae*, *aloẹ* → *aloe*, *Băbōẹ* → *Băboae*. Several dexonline forms may give the same spelling, by the rule or unchanged (*zeẹi*: the form *zeiei*, or *zeei* by the rule). They are narrowed down in three steps:
1. a form of a DEX-MDA-DOOM lexeme beats one without. The lexeme needs a definition of its own (its headword) from MDA\*, DEX\*, NODEX\* or DOOM\*; a mention in another word's definition does not count. *aleẹ* → *alee* (not *aleie*, Scriban only), *cheẹ* → *cheie* (not *chee*), *acneẹ* → *acnee* (*acneie* is only mentioned under *coș* in MDA2);
2. among DEX-MDA-DOOM forms, a plural in -ee/-eele/-eelor beats one in -euri/-eurile/-eurilor: *apogeẹ* → *apogee* (not *apogeuri*).
3. a plural in -eie, -eiele, -eielor that is DEX-MDA-DOOM beats the -ee one of the same lemma: *curcubeẹ* → *curcubeie*, *curcubeẹle* → *curcubeiele* (not *curcubee*, *curcubeele*); not that of another lemma (*bordeẹ*: *bordeie* of *bordei*, or *bordee* of *bordeu*).

The forms still tied are all offered on the homograph switch (see *Homographs*). The default is the character map's result, otherwise the first in Romanian alphabetical order: *bordeẹ* → *bordeie* (plural of *bordei*) or *bordee* (plural of *bordeu*); *beẹ* → *beie* or *bee*. A spelling of the -ae/-oe/-oae rules, or a kept genitive or plural in -eẹi/-eẹle/-eẹlor, whose character-map reading (in -aie, -oaie, -eie) is a dexonline form is not decided so: if its source is a name (with a capital in dexonline, and not in -oae), the source is the default and the -aie word the alternative (*Naẹ* → *Nae* or *naie*, *Nicolaẹ* → *Nicolae* or *Nicolaie*); otherwise the source is an old spelling, and the word reverts to the -aie word, with no switch (*baẹ* → *baie*, *Fōẹ* → *foaie*, *oboẹ* → *oboie*, *zeẹi* → *zeiei*, *zeẹle* → *zeiele*, *aloii* → *aloii*).

A spelling missing from `ee_revert` that the rule gives to a name of `ee.yaml: nume` or a word of `ee.yaml: ae_oe` reverts to that form, even when the form is missing from the database, unless its character-map reading is a known word: *Andreẹi* → *Andreei*, *Leẹi* → *Leei*, *Andreẹlor* → *Andreelor*, *Zoẹi* → *Zoei*, *Cloẹ* → *Cloe*, and so does such a spelling with its other digraphs left as they are (*Neagoẹ*, like *Nēgoẹ* → *Neagoe*), but *Mamaẹi* → *Mamaiei*, *laẹ* → *laie*.

Any other word, unknown ones included, goes through the character map (*femeẹ* → *femeie*, *zzeẹ* → *zzeie*, *Zzeẹi* → *Zzeiei*). The lookup ignores the â → î rule, so a word with â in the database reverts by character map after it (*vîntureẹ* → *vîntureie*, not *vânturee*).

## Rule files

The hand lists of the rules, the steps of the fallback rules and their exception tables are YAML files in `reguli/`, which a linguist edits. They are compiled into the core when it is built, so the apps read no YAML. A list is cited by its file and key: `ee.yaml: nume`; a table by its file: `hiat-ea.yaml`; a step by its name: the step „-ient” of `pasi.yaml`.

| File | Key | Holds | Each word |
|---|---|---|---|
| `silabisiri.yaml` | (a word) | Cezar's syllabifications, `word: syllabification` or `word: [syllabification, ...]`, the default first (see *Syllabification source*) | has the word's letters, and no empty syllable |
| `invechite.yaml` | `leme` | the obsolete lemmas, in order, each with `lema` (syllabified), `actuala`, `tulpina` and `fara_forme` (`true` or `false`; by default `false`), see *Obsolete forms* | `tulpina` starts `lema`; with `fara_forme: true`, `actuala` has at least as many letters as `tulpina` |
| | `iare_pastrate` | the lemmas in -iare that are no long infinitives (see *Syllabification source*) | ends in -iare |
| `ee.yaml` | `nume` | the feminine names of persons (see *Optional rule: -eẹ/aẹ/oẹ*) | ends in -eea, -ee or -oe |
| | `ae_oe` | the words in -ae/-oe of the -eẹ/aẹ/oẹ rule; the names in -oe of `nume` count as its words, and are not written in it | ends in -ae or -oe; is no name in -oe of `nume` |
| | `eea_neschimbate` | the headwords in -eea that keep -eē | ends in -eea |
| | `fara_masculin` | the feminines in -ee whose -eu look-alike is no masculine | ends in -ee |
| `neconvertite.yaml` | `nume_latine` | the Latin names left as they are (see *Text not converted*) | ends in a vowel + -ian |
| `a-i.yaml` | `radacini_cu_a` | the roots whose words keep â (see *Optional rule: â → î*) | holds â |
| `necunoscute.yaml` | `iesc` | the endings of an -esc adjective after i (see *Unknown words*) | |
| `clase.yaml` | (a capital) | the classes of letters of the contexts, `L: [p, b, f, v, m]` (see below) | a letter or a digraph |

The steps of the fallback rules are `pasi.yaml`, a list of steps, in order (see the fallback table, under *Syllabification source*). The first step that applies to a pair of vowels decides it. A step's keys:

| Key | Required | Meaning |
|---|---|---|
| `pas` | yes | its name, which the messages and the trace give |
| `tipar`, `ante`, `non_ante`, `post`, `non_post` | no | its patterns and their contexts, as in an exception table (below), each pattern holding two vowels side by side; a step with no `tipar` and no `prefixe` applies to every pair |
| `prefixe` | no | the word starts with one of these, and the pair is the prefix's last vowel and the next letter; not with `tipar` or a context |
| `rest_cunoscut` | with `prefixe` | the rest of the word, after the prefix, is a known word (see the fallback table) of at least this many letters |
| `non_prefixe` | no, with `prefixe` | not when the word starts with one of these, followed by a known word (of any length) |
| `doar` | no | `verb`: only in a verb infinitive (dexonline model V or VT) |
| `tabel` | no | an exception table: the step applies only where one of its entries applies too; otherwise the next step is tried |
| `rezultat` | yes | `hiat` (two syllables) or `diftong` (one; for vowels that make no digraph, simply no cut) |
| `exemple` | unless `tabel` | the words the step decides; a step with `tabel` has none, since the entries of its table have them |
| `contraexemple` | with `non_ante`, `non_post`, `non_prefixe`, `rest_cunoscut` or `doar`; never with `tabel` | words that hold the letters of a pattern, or start with a prefix, but which a condition excludes |

```yaml
- pas: -ier, -ieră, -iereasă
  tipar: [ier]
  ante: [L, P]
  non_ante: [^C, ^CC]   # nu în cuvintele de o silabă
  post: [$, ă$, easă$, easa$, ese$, esei$, esele$, eselor$]
  rezultat: hiat
  exemple: [plumier, bărbier, banchier, legumieră, rachiereasă, ...]
  contraexemple: [fier, zbier]
```

In a context of a step or of an entry of a table, a capital is a class of letters: C a consonant (any letter that is not a vowel), V a vowel (a, ă, â, e, i, î, o, u), and those of `clase.yaml`, each named by a single capital other than C and V, with its letters and digraphs (L: p, b, f, v, m; P: c, g, ch, gh). A class matches one of its letters or digraphs, so `^CC` is two consonants from the word's start. A pattern holds letters only.

Each exception table is a file of its own, a list of entries. The step of `pasi.yaml` that consults it names it in its `tabel`, and has the table's pair in a pattern and its result:

| File | Pair | Its entries make the pair |
|---|---|---|
| `diftong-ia.yaml` | ia | a diphthong |
| `hiat-ea.yaml` | ea | a hiatus |
| `hiat-ie.yaml` | ie | a hiatus |
| `hiat-oa.yaml` | oa | a hiatus |

An entry applies to a pair of vowels when one of its patterns holds both vowels, and its contexts hold just before and just after that pattern. The first entry of the table that applies decides the pair. An entry's keys:

| Key | Required | Meaning |
|---|---|---|
| `tipar` | yes | one or more patterns, each holding the table's pair: letters, with `^` at its start for the word's start and `$` at its end for the word's end |
| `ante` | no | just before the pattern is one of these: letters and classes (`gr`, `C`), `^` alone (the pattern starts the word), or `^` and letters or classes (the word starts with them, then the pattern: `^gor`, `^CC`) |
| `non_ante` | no | just before the pattern is none of these, written as in `ante` |
| `post` | no | just after the pattern is one of these: letters and classes (`in`, `CV`), `$` alone (the pattern ends the word), or letters or classes and `$` (they end the word: `e$`) |
| `non_post` | no | just after the pattern is none of these, written as in `post` |
| `exemple` | yes | the words the entry decides |
| `contraexemple` | with `non_ante` or `non_post` | words that hold the letters of a pattern, but which its contexts, or its `^` or `$`, exclude |

```yaml
- tipar: [ghia]
  non_ante: [n, litur, ^gor]
  non_post: [t, n, l, ce]
  exemple: [maghiar, ghiară, ghiarele, ghiață]
  contraexemple: [unghiar, liturghiar, gorghias, privileghiat, aghian, oghial, eleghiacesc, stavropighiacesc]
```

The format:
- YAML 1.2, in UTF-8, with or without a byte order mark. A file maps its keys to their values (`silabisiri.yaml` maps words, `clase.yaml` classes), and every key is required, except in a table or in `pasi.yaml`, a list of entries or steps, each mapping its keys to their values. A list is written `[a, b]`, or one `- a` per line. Every value is text: `n`, `no`, `on` and `true` are words, not booleans.
- Keys are lowercase ASCII with `_`, except the classes of `clase.yaml`, a capital each. Words are written in lower case, in the standard orthography: NFC, ș/ț with a comma below, â inside words.
- Order is kept where it matters: the syllabifications of a word and the lemmas of `leme`, the first being the default; the steps of `pasi.yaml` and the entries of a table, the first that applies deciding.
- `#` starts a comment. Comments explain; they never change a spelling, and the data file's rules don't include them, nor the examples and counterexamples of the steps and of the tables, nor the names of the steps (see *Data*).
- Anchors (`&`), tags (`!`) and a second document (`---`, `...`) are not used.

The loading checks run when the core is built, which an error stops, and with `cargo run -p grafia-reguli`, which in CI also shows each message on its line of the pull request. A message gives the file, the line and the column, in Romanian: `reguli/ee.yaml:3:30: eroare: „zoş” are ş cu sedilă; scrie ș, cu virgulă.` They check:
- the YAML syntax; the keys, none unknown, repeated or missing; the type of each value (a list, a word, `true` or `false`), and that none is empty, an empty value being reported at its key; that `silabisiri.yaml`, `pasi.yaml` and the tables are not empty; and that each YAML file of `reguli/` is a rule file (the other files, hidden or not in .yaml/.yml, such as .DS_Store or an editor's backup, are left alone);
- the characters: an error for ş/ţ with a cedilla, ã, a decomposed letter, a capital other than a class of a context, a space, a hyphen outside a syllabification, a `^` or a `$` other than those of a pattern or a context above, and any other character outside the Romanian alphabet; a warning for î inside a word (*neînțeles* is right);
- no word twice in a list, no syllabification twice for a word, no lemma twice in `leme`; in an entry of a table or a step, no pattern, prefix, item or word twice in its list, and no word both in `exemple` and in `contraexemple`; no letter twice in a class;
- each word as the table of the lists says, and each pattern of a table holding the table's pair;
- the classes: each named by a single capital other than C and V, with letters and digraphs of at most two letters; a capital of a context is C, V or a class of `clase.yaml`;
- the steps: `rezultat` is `hiat` or `diftong`, `doar` is `verb`, `rest_cunoscut` a number of letters; `prefixe` asks for `rest_cunoscut` and takes no `tipar` and no context, `rest_cunoscut` and `non_prefixe` ask for `prefixe`, and a context asks for a `tipar`; each pattern of a step holds two vowels side by side; `exemple` is required unless `tabel`, and `contraexemple` with `non_ante`, `non_post`, `non_prefixe`, `rest_cunoscut` or `doar`; no name twice;
- the tables of the steps: a step with `tabel` names a table, holds its pair in a pattern, has its result, and has no `exemple` or `contraexemple`; each table is named by exactly one step;
- the last step: it has no `tipar`, `prefixe`, context, `doar` or `tabel`, so that every pair is decided.

The rules checks run with the tests, `cargo test`, and with `cargo xtask reguli`, which runs the loading checks first; in CI they too show each message on its line. They decide the pairs of vowels of each example and counterexample of a step or of an entry of a table by the fallback rules alone, with no explicit syllabification and no dexonline (the examples of a step with `doar: verb` as verb infinitives, every other word as no verb), and check that:
- each word of `exemple` has a pair that the entry or the step decides, not an earlier step or entry: `reguli/diftong-ia.yaml:27:13: eroare: „ghiare” nu e decis de intrarea aceasta, ci de pasul „-iare care nu e infinitiv lung” (pasi.yaml:100); alege un cuvânt pe care îl decide ea.`;
- each word of `contraexemple` has a pair that the letters of a pattern hold, or that follows a prefix, and the entry or the step applies to none of these pairs, which the rules all cut the other way;
- each pattern, each prefix, and each item of `ante` and `post` decides a word of `exemple`, and each item of `non_ante`, `non_post` and `non_prefixe` excludes a word of `contraexemple`.

## Data

The word data comes in one file, `grafia.dat` (its layout is in `crates/core/src/datafile.rs`). Besides five tables, it holds the format version, the date of the dexonline data it was built from (the day of dexonline's latest change to a lexeme or a definition), the rules it was built with, and a checksum, checked when the file is opened. The rules are `RULES_HASH`, a hash of the code of the files of `crates/core/src/rules/` and of `build.rs`, and of the code the rule files compile to (their values, not their comments, nor the examples of the steps and of the tables, nor the names of the steps; see *Rule files*), without comments or whitespace: the web app's build refuses a data file built with other rules. The tables are each keyed on a normalized word:
- `forms`: a form → `concise`, its concise spelling; `concise_ee`, its spelling with the -eẹ/aẹ/oẹ rule when that differs from `concise`; `alternatives`, a homograph's other spellings; and `current`, an obsolete form's current form (see *Obsolete forms*).
- `eu_lemmas`: every normalized lemma in -eu, for unknown plurals in -eele/-eelor (see *Unknown words*).
- `ea_lemmas`: every normalized lemma in -ea, for unknown genitives in -eei (see *Unknown words*).
- `iesc_lemmas`: every normalized lemma in -iesc → its syllabification (*harabagi-esc*), for unknown forms of -iesc adjectives (see *Unknown words*).
- `ee_revert`: a spelling with the -eẹ/aẹ/oẹ rule → the form it reverts to and the other forms of a tie, only where the character map alone is not the answer (see *Reverse conversion*).

The same tables always give the same file, byte for byte.

The file is built from an export of dexonline's records: the pipeline input described below, the words the prefix rule may find and the date of the data. The export is published as an archive on a release of the repository, `dexonline-YYYY-MM-DD` (the date of the data), and `dexonline.lock` pins it by its SHA-256. The file is not committed: the repository's CI builds it from the pinned export, with the rules it then ships with, and so does anyone with `cargo xtask data`. The same export and rules give the same file, byte for byte. Only a dexonline update needs dexonline's database itself, to make a new export.

`forms` stores only forms that can differ: those containing ie/ia/ea/oa or iăr, or ending in -ee, -eea, -eei, -eele, -eelor, -euri, -eurile, -eurilor or -oae, the forms of the lemmas in `ee.yaml: ae_oe` (*Nae*, *aloei*), the forms of obsolete lemmas (*alineez*), and, built from `ee.yaml: nume`, the genitive and articulated plural (-i, -le, -lor added to the name, its final -a dropped) of each listed name that dexonline lists without them (*Andreei*, *Andreele*, *Andreelor* ← *Andreea*; *Zoei* ← *Zoe*). The pipeline input per inflected form is: the form, its lemma, the lemma's dexonline hyphenation (complete, partial or empty), the number (S/P), whether the lemma is a verb (dexonline model V or VT), whether it is a lowercase feminine noun or unclassified (model F, T or IL; for long infinitives in -iare), and whether it is DEX-MDA-DOOM: it has an active definition of its own (whose headword is the lexeme) from a source whose short name starts with MDA, DEX, NODEX or DOOM (MDA2, DEX '09, NODEX, DOOM 3, ...; not the internal *dexonline*), the form's inflection and variant, which pair the forms of an obsolete lemma with those of its current lemma, and whether the lemma is a masculine noun or an adjective (model M or A), which makes a lemma in -eu the masculine of its -ee feminine (*zeu*, *zee*).
