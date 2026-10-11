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

A word form may have more than one correct concise spelling. It always has one default (*ceai* → *cēi*). A form with two syllabifications (a homograph) also carries its other spelling, and the user can switch to it (see *Homographs*). The optional -ee rule is the exception: it gives one output per form.

## Syllabification source

A headword uses its own syllabification. Any other inflected form takes its cuts from its lemma's syllabification (see the derivation table below).

A lemma's syllabification is Cezar's explicit one when listed (`rules::SYLLABIFICATIONS`: *neant* `ne-ant`, *coastă* `coas-tă`, *mielea* `mi-e-lea`, *înfieri* `în-fi-e-ri`, *răspieri* `răs-pie-ri`, ...; an entry may list several readings, the first being the default: *ie* `i-e` / `ie`, *miază* `mia-ză` / `mi-a-ză`, *vier* `vier` / `vi-er`), otherwise dexonline's hyphenation completed by fallback rules. A form listed there takes all its spellings from there, whatever its lemma: *piei* is only *pẹi* (not also *piei* ← *pia* `pi-a`).

- **dexonline hyphenation.** Only the first variant is used. It may be complete (`fa-mi-li-e`), a word end (`-li-e`), a word start (`a-ce-tal-`) or a word interior (`-pioa-`). Inside the span it covers, its cuts always prevail over the rules. A hyphenation that does not fit the word is ignored.
- **Fallback rules.** Everywhere else, the rules place only cuts between two vowels, since grafia concisă uses no other cuts. The rules are known to be incomplete and will be refined:

| Vowels | Context | Cut | Examples |
|---|---|---|---|
| ie | in -ient, -ienț/-ient + a/ă/e/i | hiatus | *client*, *ambient*, *experiență*, *clientelă* (dexonline: *farniente* `far-nien-te`) |
| ia, ea | before -bil, after a consonant | hiatus | *fiabil*, *agreabil* (not *debreiabil*: after a vowel, like *de-bre-ia*) |
| e-e | anywhere | hiatus | *idee* → *ide-e* (the -ee switch reads it as *e-ie*) |
| e-a, i-a, i-e, o-a | after a prefix (re-, pre-, ne-, tele-, co-, semi-, anti-, multi-, auto-, micro-, macro-, foto-, pseudo-, bio-, neo-, radio-, aero-, agro-, electro-, hidro-, termo-, geo-, proto-, socio-, psiho-, stereo-) when the rest is a known word: a form of a lowercase dexonline lexeme with frequency ≥ 0.9, of at least 4 letters | hiatus | *re-abilitat*, *ne-adevărat*, *auto-analiză* (not *neam*, *neagră*, *autoare*; *prea-curat* is *prea* + *curat*); *co-activ* (the false hits of co- are fixed in `rules::SYLLABIFICATIONS`: *coastă*, *coardă*, *coacere*, *coacăț*) |
| i-e | prefix bi- before en | hiatus | *bienal* (not *biet*) |
| i-i | word-final, after a consonant, in a verb infinitive | hiatus | *sfi-i*, *pri-i*, so *sfiește*, *priască* keep i-e/i-a (not the plural *copii*) |
| ea | neologism endings and stems (`rules::EA_HIATUS`): final -eal (not after g/d/b), -linear/-hlear/-balnear, -laureat(ă)/-lineat(ă)/-nucleat(ă) and similar, nuclear-, creat-, reac(t/ț)-, steat-, meandr-, aleat-, beatif-, reali-/ideali-/boreal (not in -răboreală/-stoboreală)/cereal | hiatus | *real*, *leal*, *interliceal*, *boreal*, *hiperboreală*, *nuclear*, *denuclearizat*, *linear*, *alineat*, *anucleată*, *laureată*, *creator*, *pancreatic*, *reactor*, *reacție*, *steatoză*, *meandru*, *aleator*, *beatific*, *realizat*, *cereale* (not *deal*, *Ardeal*, *mangeal*, *beat*, *asear*, *greșeală*, *răboreală*, *cleat*, *procleat*, *pleată*, *soreată*) |
| oa | neologism stems and endings (`rules::OA_HIATUS`): coagul-, coabit-, coaliț-/coaliz-, final -zoar | hiatus | *coagulat*, *coabita*, *coaliție*, *coalizat*, *protozoar*, *cazoar* (not *mozoară*) |
| ie, ia | word start, or after a vowel (ă, î included) | diphthong | *iarbă*, *femeie*, *tăia*, *tăiere*, *înmuiere* |
| ie, ia | word-final, after a consonant | hiatus | *familie*, *vie*, *istoria* |
| ia | medial, after a consonant (the -ia\* endings) | hiatus | *apropiat*, *pian*, *social*, *specializat*, *glaciar*, *variare* |
| ia | same, in native stems (`rules::IA_DIPHTHONG`) | diphthong | *chiar*, *piatră*, *biată*, *viață*, *amiază*, *ghiață*, *ghiarele*, *maghiar*, *rachiareasă*, *Caragiale*, *chiajna*, *obială* (not *liturghiar*, *eleghiacesc*, *schiagramă*, *Gorghias*, *Miaulis*, *microbial*) |
| ia | final -iare after p/b/f/v/m or c/g/ch/gh, in a word that is no long infinitive (see below) | diphthong | *fiare* (like *fiere*) |
| ie | medial, after p/b/f/v/m or c/g/ch/gh | diphthong | *fier*, *piept*, *pieptene* |
| ie | same, in neologism stems and endings (`rules::IE_HIATUS`: gien, pieton, cviet, societ, soviet, vien, miel + o/in/it/ie, -ietor/-ietoare, bier neither word-initial nor after z/s), in final -ier/-ieră after a vowel and a consonant, final -iesc (adjectives) and final -iere/-ierea/-ieri/-ierii/-ierile/-ierilor (long infinitives; after a vowel it is a diphthong, see above: *tăiere*) | hiatus | *igienă*, *pieton*, *sovietism*, *cvietism*, *vienez*, *mielită*, *poliomielită*, *așchietor*, *bărbierit*, *corăbierie*, *bărbier*, *plumier*, *banchier*, *genunchier*, *legumieră*, *zapciesc*, *surugiesc*, *copiere*, *apreciere*, *veghiere*, *învierea*, *aprecierilor* (not *miel*, *mielușor*, *zbiera*, *fier*, *zbier*; not *miere*, *înmiere*, *fiere*, *piere* and their forms (*mierea*, *pieri*): `rules::IERE_DIPHTHONG`) |
| ie | medial, after another consonant | hiatus | *prieten*, *cartier* |
| anything else (ea, oa, ...) | anywhere | diphthong | *seară*, *soare* |

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

A long infinitive in -iare is not standard and is written in its -iere form, which then follows the rules for -iere: *apreciare* → *apreciere*, *apreciarea* → *aprecierē*, *tăiare* → *tăẹre*. In its forms, the last -iar-/-iăr- becomes -ier-: *apreciărilor* → *aprecierilor*. A long infinitive is a lemma in -iare that dexonline lists in lowercase as a feminine noun or unclassified (model F, T or IL), except *fiare* and *mobiliare* (`rules::IARE_KEPT`). A form that another lemma also gives keeps that lemma's spelling: *conciliare* (the adjective *conciliar*), *copiare* (the noun *copiar*). The -iere spelling is then no alternative on the homograph switch.

## Optional rule: final -ee (switch, default off)

Lemmas in *-ee* follow the *femeie* pattern: word-final *-ee* is read as *-eie* and written *-eẹ*, the same as *femeie* → *femeẹ*.

| Form | Condition | Result |
|---|---|---|
| singular in -ee | always | *idee* → *ideẹ*, *alee* → *aleẹ* |
| plural in -ee | lemma ends in -eu | *bombee* (← *bombeu*) → *bombeẹ* |
| plural in -eele, -eelor | lemma ends in -eu | *bombeele* → *bombeẹle*, *liceelor* → *liceẹlor* |
| plural in -euri, -eurile, -eurilor | the form minus -ri/-rile/-rilor is its -eu lemma, of more than 4 letters | *bombeuri* → *bombeẹ*, *bombeurile* → *bombeẹle*, *liceurilor* → *liceẹlor* (not *pneuri*, *zeurile*) |
| plural in -ee | lemma ends in -ee | *logoree* → *logorei* (like *femeie* / *femei*) |
| plural in -ee | any other lemma | unchanged |
| final -eea | lemma ends in -ee or -eu | *ideea* → *ideė*, *orhideea* → *orhideė* (like *femeia*), *spontaneea* → *spontaneė* (← *spontaneu*); the base spelling is *ideē* (*i-de-ea*) |
| final -eea | lemma ends in -eea (headword), not `rules::EEA_UNCHANGED` | *Niceea* → *Niceė*, *Andreea* → *Andreė*, *Galileea* → *Galileė* (the pronouns *aceea*, *ceea*, *oareceea* keep *-eē*); also an unknown word |
| final -eei | lemma as for -eea: in -ee or -eu, or an -eea headword not in `rules::EEA_UNCHANGED` | *ideei* → *ideii*, *orhideei* → *orhideii* (like *femeii*), *spontaneei* → *spontaneii*, *Galileei* → *Galileii* |
| final -eele | lemma ends in -ee | *orhideele* → *orhideile* (like *femeile*) |
| final -eelor | lemma ends in -ee | *orhideelor* → *orhideilor* (like *femeilor*) |

If one spelling is both a singular and a plural of an -ee lemma, the singular reading wins, so each form still gets one output.

With the switch on, reverse conversion looks the words up to undo the rule (*ideẹ* → *idee*, *orhideii* → *orhideei*; see *Reverse conversion*).

## Optional rule: â → î (switch, default on)

â is written î (Â as Î), except in words with the root *român* or *rumân*, i.e. any word containing *român* or *rumân*: *câine* → *cîine*, *când* → *cînd*, *pâinea* → *pîinē*; unchanged: *român*, *românesc*, *România*, *aromân*, *dacoromân*, *rumân*, *rumânesc*.

It applies to every word after the rest of the conversion, whether the word is in the database or not, and never marks a word as unknown. It applies to every spelling of a homograph.

## Homographs

A form whose records give different spellings is a homograph. Examples:
- *ia*: *ė* (the verb *lua*, the interjection) or *ia* (the articulated *ie*, *i-a*);
- *ie*: *ie* (the blouse, *i-e*) or *ẹ* (the adverb);
- *miază*: *mėză* (noon, *mia-ză*) or *miază* (myiasis, *mi-a-ză*);
- *curie*: *curẹ* or *curie*;
- *liază*: *liază* (the noun) or *liėză* (the verb *lia*);
- *vier*: *vẹr* (the boar, *vier*) or *vier* (păzitor de vie, *vi-er*).

The default is the spelling chosen by the priority above. The others are stored in `forms.alternatives` and returned with it: in `segments[].alts` from the core's `convert` and in `alternatives` from its `lookup`, the default first in both. A spelling counts only when an explicit, dexonline or partial syllabification backs it, except that when one record gives the suffix -ėză/-ėscă/-ėsca (see *Syllabification source*), all the form's spellings count. `rules::SYLLABIFICATIONS` may give several readings for one word; the first is the default.

The UI shows every spelling of a homograph in one switch on a dark-grey track. The active spelling sits on the text area's background, and the others are light grey on the track. A click on one makes it active. Copying takes the active spellings.

The -ee rule does not produce homographs: its singular/plural choice stays single (see *Optional rule: final -ee*). The exception is an unknown word in -eei, -eele or -eelor (see *Unknown words*). Reverse conversion with the -ee switch shows its ties on the same switch (see *Reverse conversion*).

## Case

The case pattern of the word is carried over: *Soare* → *Sōre*, *SOARE* → *SŌRE*. Other mixed-case patterns are reduced to an initial capital.

## Input normalization

Text is normalized to NFC. For lookup, ş/ţ with cedilla are treated as ș/ț with comma below.

## Unknown words

A word that contains a candidate digraph but is missing from the database is converted by the fallback rules alone and flagged. With the -ee switch it is read as a singular, with two exceptions.

A word with an ending of an -iesc adjective after i (-iești, -iescu, -iescul, -iescului, -ieștii, -ieștile, -ieștilor, -iască, -iasca) is first tied to its lemma: the ending is replaced by -esc, and if that gives a lemma in -iesc (`iesc_lemmas`), the word is derived from the lemma's syllabification, suffix rule included. dexonline lists some of these adjectives with no forms: *harabagiasca* → *harabagiėsca*, *harabagiască* → *harabagiėscă*, *harabagiești* unchanged (← *harabagi-esc*).

A word in -eei is first read as the genitive of a known word: final -eei is replaced by -eea, then by -ee, and the first that is known is its lemma: -eea must be a lemma (`ea_lemmas`), -ee any form (`forms`). The -eei rule then gives one spelling: *Andreei* → *Andreii* (← *Andreea*, invariable in dexonline, with no genitive), *aleei* → *aleii* (← *alee*), *aceei* unchanged (← *aceea*, see `rules::EEA_UNCHANGED`). *ideea* is only a form of *idee*, so it is no lemma in -eea; *ideei* → *ideii* comes from *idee*. If neither is known, the word gets two spellings, unchanged (the default) or -eii, shown as a homograph switch: *Zzeei* or *Zzeii*.

A word in -euri/-eurile/-eurilor becomes -eẹ/-eẹle/-eẹlor when the word minus -ri/-rile/-rilor is a lemma in -eu (`eu_lemmas`) of more than 4 letters, as for a known form. Otherwise it is unchanged.

A word in -eele/-eelor:
- Final -ele/-elor is replaced by -u. If that gives a lemma in -eu from the database (`eu_lemmas`), the word is its plural, with one spelling: *bombeele* → *bombeẹle* (← *bombeu*, whose dexonline plural is *bombeuri*).
- Otherwise it gets two spellings, as the plural of an -ee lemma (*coloboleile*, *coloboleilor*, the default) or of an -eu lemma (*coloboleẹle*, *coloboleẹlor*). They are shown as a homograph switch, with a footnote using the word's stem: „Dacă singularul e *colobolee*, alege forma *coloboleile/lor*. Dacă singularul e *coloboleu*, alege forma *coloboleẹle/lor*.”

Words with no candidate digraph and no iăr are never looked up.

## Reverse conversion

Concise → standard is currently a character map (ẹ→ie, ė→ia, ē→ea, ō→oa), except that the ė of a word-final -iėză/-iėscă/-iėsca reverts to a (*studiėză* → *studiază*, *sfiėscă* → *sfiască*, *fiėsca* → *fiasca*). This holds only under the current rules and is not a guaranteed property. It is inexact for *fiiască*, *fiiasca* → *fiėscă*, *fiėsca*, which revert to *fiască*, *fiasca*. A long infinitive in -iare is not restored: *apreciere* stays *apreciere*. î is left unchanged, so â is not restored after the â → î rule (*cîine* stays *cîine*).

With the -ee switch on, the input is read as written with the -ee rule. A word is first looked up among the spellings the rule gives to database forms (`ee_revert`), and reverts to the form it came from: *orhideẹ* → *orhidee*, *orhideile* → *orhideele*, *ideė* → *ideea*, *bombeẹle* → *bombeurile* (dexonline has no *bombeele*). Several dexonline forms may give the same spelling, by the rule or unchanged (*zeii*: the form *zeii*, or *zeei* by the rule). They are narrowed down in two steps:
1. a form of a DEX-MDA-DOOM lexeme beats one without. The lexeme needs a definition of its own (its headword) from MDA\*, DEX\*, NODEX\* or DOOM\*; a mention in another word's definition does not count. *aleẹ* → *alee* (not *aleie*, Scriban only), *cheẹ* → *cheie* (not *chee*), *acneẹ* → *acnee* (*acneie* is only mentioned under *coș* in MDA2);
2. among DEX-MDA-DOOM forms, a plural in -ee/-eele/-eelor beats one in -euri/-eurile/-eurilor: *apogeẹ* → *apogee* (not *apogeuri*).

The forms still tied are all offered on the homograph switch (see *Homographs*). The default is the character map's result, otherwise the first in Romanian alphabetical order: *bordeẹ* → *bordeie* (plural of *bordei*) or *bordee* (plural of *bordeu*); *beẹ* → *beie* or *bee*; *zeii* → *zeii* or *zeei*.

Any other word, unknown ones included, goes through the character map (*femeẹ* → *femeie*, *zzeẹ* → *zzeie*). The lookup ignores the â → î rule, so a word with â in the database reverts by character map after it (*vîntureẹ* → *vîntureie*, not *vânturee*).

## Data

The word data comes in one file, `grafia.dat` (its layout is in `crates/core/src/datafile.rs`). Besides five tables, it holds the format version, the date of the dexonline data it was built from (the day of dexonline's latest change to a lexeme or a definition), the rules it was built with, and a checksum, checked when the file is opened. The rules are `RULES_HASH`, a hash of the code of `crates/core/src/rules.rs` and `build.rs` without comments or whitespace: the web app's build refuses a data file built with other rules. The tables are each keyed on a normalized word:
- `forms`: a form → `concise`, its concise spelling; `concise_ee`, its spelling with the -ee rule when that differs from `concise`; and `alternatives`, a homograph's other spellings.
- `eu_lemmas`: every normalized lemma in -eu, for unknown plurals in -eele/-eelor (see *Unknown words*).
- `ea_lemmas`: every normalized lemma in -ea, for unknown genitives in -eei (see *Unknown words*).
- `iesc_lemmas`: every normalized lemma in -iesc → its syllabification (*harabagi-esc*), for unknown forms of -iesc adjectives (see *Unknown words*).
- `ee_revert`: a spelling with the -ee rule → the form it reverts to and the other forms of a tie, only where the character map alone is not the answer (see *Reverse conversion*).

The same tables always give the same file, byte for byte.

`forms` stores only forms that can differ: those containing ie/ia/ea/oa or iăr, or ending in -ee, -eea, -eei, -eele, -eelor, -euri, -eurile or -eurilor. The pipeline input per inflected form is: the form, its lemma, the lemma's dexonline hyphenation (complete, partial or empty), the number (S/P), whether the lemma is a verb (dexonline model V or VT), whether it is a lowercase feminine noun or unclassified (model F, T or IL; for long infinitives in -iare), and whether it is DEX-MDA-DOOM: it has an active definition of its own (whose headword is the lexeme) from a source whose short name starts with MDA, DEX, NODEX or DOOM (MDA2, DEX '09, NODEX, DOOM 3, ...; not the internal *dexonline*).
