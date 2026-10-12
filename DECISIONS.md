# grafia concisă — settled decisions

This file is the log of closed decisions: what was decided, when, and why, with a pointer to where `concise-writing-spec.md` spells out the resulting rule. The mechanics (suffixes, lists, tables) live only in the spec.

**Precedence.** These decisions are the authority on intent; the spec implements them in full. A disagreement between the two is a defect, fixed in the spec in the same change. Reopen a decision only on purpose, and record the change here with its date.

## Naming
- The project is always named "grafia concisă" (slug `grafia-concisa`), never "scriere concisă".

## Core rule (spec: *Rule*)
- **Concision applies whenever no hiatus is involved.** Both vowels of ie/ia/ea/oa in one syllable → ẹ/ė/ē/ō. A hiatus is never converted.
- **2026-09: stress dropped.** The June 1, 2026 plan made stress a condition for ia/ea/oa. Syllabification alone decides.
- **No special cases after palatal consonants.** The base rule already handles ce/ci/ge/gi and stress-shifted derivatives (*ceasornic*, *ceainărie*).
- **Overlapping digraphs** are scanned left to right without overlap (*leoarcă* → *leōrcă*, *ploaie* → *plōẹ*).
- **2026-10-06: the verbal suffix -ează/-ească after a hiatus i is written -ėză/-ėscă** (*studiază* → *studiėză* ← *stu-di-a*, *sfiască* → *sfiėscă* ← *sfi-i*). **Why:** the suffix should never show a bare a, but an e form, as in the other persons (*studiez*, *sfiesc*). Spec: the suffix table under *Syllabification source*.
  - **2026-10-06, reopened the same day:** the adjectives in -iesc follow the same rule (*fiască* → *fiėscă*, *fiasca* → *fiėsca* ← *fi-esc*). It was first limited to verbs.

## Homographs (spec: *Homographs*)
- **2026-09-29: a word form can have more than one correct spelling.** This replaces the earlier principle of one output per form (*"No ambiguous forms"*).
  - **Why:** some spellings can't be decided from the word alone: *ia* is the verb *lua* or the articulated *ie*; *miază* is noon or myiasis; *curie*; *bielă*.
  - Every form keeps one default; the others are offered on a switch in the UI.
  - **Only backed spellings count** (explicit, dexonline or partial syllabification). A spelling from rules alone is taken to be a rules error, not a second reading.
  - **2026-10-09: a backed spelling also beats rules alone for the default**, even when the rules-only record is the form's own headword (*agreare* ← *agrea* `a-gre-a`, not *agrēre*; *riviera*, *sibieni*). **Why:** a separate dexonline lexeme with no syllabification of its own (the adjective *coagulat*, the noun *agreare*) put the rules' error first in about 35 forms. The explicit *ia* `ia` / `i-a` keeps the verb first. The narrower alternative, letting a derived form win only where its cut is copied from the lemma's dexonline span, was rejected: it is more complex and leaves *sibieni*, *olimpieni* wrong.
  - **2026-10-06: an ambiguity with the -ėză/-ėscă suffix shows both spellings.** When one record of a form gives the suffix -ėză/-ėscă/-ėsca, all its spellings count, even from rules alone (*liază*: the noun, or the verb *lia*, *liėză*). **Why:** the suffix rule applies to verbs and adjectives only, and the form alone does not say which it is.
- The optional -ee rule keeps one output per form, except for unknown words (see below).

## Syllabification (spec: *Syllabification source*)
- **Source: dexonline.** **2026-09-27:** dexonline always prevails over the rules, which only fill in what it does not cover.
- **2026-09-27: partial syllabifications are used.** **Why:** dexonline syllabifies completely only about 5% of the lemmas with ie/ia/ea/oa, and many of the rest are partial.
- **2026-09-27: rule-based fallback syllabifier.** This reverses the earlier deferral. The rules are knowingly incomplete and will be refined over time.
- **2026-09-27: Cezar's explicit syllabifications** (`rules::SYLLABIFICATIONS`) beat both dexonline and the rules (*neant* `ne-ant`). Since 2026-09-29, an entry may give several readings (*miază*: noon, the default, and myiasis).
  - **2026-10-09: an explicit syllabification of a form gives all its spellings**, and records of other lemmas add none. **Why:** *piei* is always a diphthong (*pẹi*), even though *pia* `pi-a` also gives the form *piei*. Added the same day: *mielea* `mi-e-lea`, *înfieri* `în-fi-e-ri`, *răspieri* `răs-pie-ri` (a verb, not a long infinitive), and the homograph *vier*: the boar (*vier*, the default) or păzitor de vie (*vi-er*, rare).
- *ideea* is syllabified *i-de-ea* only, never *i-de-e-a*.
- **Inflected forms derive their cuts from the lemma.** **Why:** dexonline often gives no syllabification for inflected forms, especially articulated nouns. The derivation is a heuristic: on 2026-09-24 it agreed with the forms' own dexonline syllabification in 337 of 338 cases (the exception: *cheflie* ← *che-fliu*).
  - A form that is itself a headword uses its own syllabification, which is why the noun *ploaie* beats the verb form of *ploua*. **2026-10-09:** unless its syllabification comes from rules alone and a derived one is backed (see *Homographs*).
  - **2026-10-05: a lemma's hiatus after o or e is not inherited by its forms.** **Why:** o and e diphthongize as a matter of course in Romanian, so a hiatus in the root says nothing about the form (*(să) ploaie* ← *plo-ua* is *plōẹ*). Only an i keeps the lemma's cut.
  - **2026-10-11: a vowel ending added after the final i of a verb in consonant + i is a hiatus** (*știa*, *știam* ← *ști*: *ști-a*, *ști-am*; *neștie* ← *nești*: *ne-ști-e*). **Why:** the final i of such a verb is a full vowel, so the ending starts a new syllable; the imperfect and the present *-ie* of *ști* and its compounds came out as diphthongs (*ștė*, *neștẹ*).

### Fallback rules, 2026-09-27 (spec: the fallback table)
- **ie hiatus in -ient/-ienț** (*client*, *experiență*). The one dexonline conflict is *farniente*.
- **ia and ie**:
  - medial *ia* after a consonant is a hiatus (the *-ia\** endings: *apropiat*), except in native stems (*chiar*, *piatră*, *viață*);
  - medial *ie* after a labial or palatal consonant is a diphthong (*fier*, *piept*), but long infinitives in *-iere* are a hiatus (*copiere*), except the native *miere*, *fiere*, *piere*;
  - **2026-10-09:** the long infinitive *-iere(a)* has the diphthong ie only after a vowel (*tă-ie-re*), so after a consonant all its forms are a hiatus (*co-pi-e-rea*, *în-vi-e-rea*, *co-pi-e-ri*, *-ierii*, *-ierile*, *-ierilor*);
  - **2026-10-09:** *revier*, *cocier*, *cucier*, *vietor* and *pietor* are a hiatus, like the other *-ier* and *-ietor* words;
  - **2026-10-06:** so are the adjectives in *-iesc* after a consonant (*zap-ci-esc*, *su-ru-gi-esc*): the e goes to the next syllable;
  - *-iabil* and *-eabil* are a hiatus (*fiabil*, *agreabil*);
  - if the *-iu* version of a word exists, its ie/ia is a hiatus (*argintie* ← *argintiu*);
  - *prieten* is *pri-e-ten*.
- **ea is a hiatus in neologisms** (*real*, *nuclear*, *creat*), chosen from dexonline's own syllabifications. Native words stay diphthongs (*deal*, *beat*, *-eală*).
- **Prefixes are recognized by checking that the rest is a known, frequent word** (*re-abilitat*; not *neam*, *autoare*). *prea* + a word stays one syllable (*preacurat*). co- is included, and its false hits are fixed by explicit syllabifications (*coastă*). *bi-* counts as a prefix (*bi-e-nal*).
- **2026-10-09: more neologism hiatuses**, after Cezar reviewed the rules-decided pairs that disagree with dexonline's own syllabifications of similar words:
  - ie after a labial or palatal consonant: *igienă*, *pieton*, *cvietism*, *societar*, *vienez*, *mielită* (not *miel*), *așchietor*, *bărbierit* (not *zbiera*), and final -ier/-ieră in words of more than one syllable (*plumier*, *banchier*, *genunchier*, *legumieră*; not *fier*, *zbier*). The *soviet* family is a hiatus everywhere, as in dexonline (*so-vi-et*, *so-vi-e-tic*, *sovietism*);
  - ea: *reactor*, *reacție*, *creator*, *pancreatic*, *steatoză*, *meandru*, *aleator*, *beatific*, *denuclearizat*;
  - oa: *coagulat*, *coabita*, *coaliție*, *protozoar*.
- **2026-10-09: the native ia stems were reviewed.** *chiajna* and *obială* are diphthongs. *liturghiar*, *eleghiacesc*, *stavropighiacesc*, *schiagrafie*, *schiagramă*, *Gorghias* and *Miaulis* are a hiatus. *rachiar*, *ochiar*, *peruchiar* and the other *-chiar* words are left as they are. **Why:** they are practically out of use.
- **2026-10-11: corrections found while testing every rule**, from the words each exception alone decides:
  - *-iabil* after a vowel follows its verb, a diphthong (*debreiabil* → *debreėbil* ← *de-bre-ia*); the *-abil* hiatus stays after a consonant (*fiabil*, *agreabil*);
  - final *-eal* after c is a hiatus too (*interliceal*); after g, d, b it stays a diphthong (*mangeal*, *deal*, *podbeal*);
  - *boreal* is a hiatus only in *boreal* itself and its compounds (*boreală*, *hiperboreală*), not in the native *-eală* words (*răboreală*, *stoboreală*);
  - the *-eat* hiatus covers whatever follows it: the feminine *-eată* (*anucleată*, *bacalaureată*) and *delineator*, *delineatură*, like the forms dexonline gives;
  - *ghiare* is a diphthong (*ghėre*, like *ghiară*): the exceptions for *-ghiare* and *-chiare* were for long infinitives, which are now written *-iere* (*înjunghiere*);
  - final *-iereasă*, with its forms, after a vowel and a consonant is a hiatus, like *-ier*: the i belongs to the stem (*rachiereasă* ← *rachier*, *pompiereasă* ← *pompier*, like *bărbiereasă*);
  - the *coaliț-* stem has no *coalit-* spelling.
  - **Why:** each exception was checked against every dexonline word it decides; these were the ones that gave a wrong spelling.
- **2026-10-05: final -ii after a consonant is a hiatus in verb infinitives only** (*sfi-i*, *pri-i*; not *copii*). **Why:** dexonline syllabifies none of the -ii verbs, and without the cut their forms came out as diphthongs (*sfiește*, *priască* are *sfi-eș-te*, *pri-as-că*).

## Long infinitives in -iare (spec: *Syllabification source*)
- **2026-10-09: a long infinitive in -iare is written -iere**, then follows the -iere rules (*apreciare* → *apreciere*, *tăiare* → *tăẹre*). **Why:** long infinitives in -iare are incorrect; the correct form always ends in -iere.
  - A form that another lemma also gives keeps that lemma's spelling (*conciliare*, the adjective). **Why:** the other word is the current one.
  - It replaces the earlier diphthong reading of -iare after a labial or palatal consonant, which now applies only to words that are not long infinitives (*fiare*).
  - **2026-10-11:** it is an obsolete form (see *Obsolete forms*): the switch offers the -iare spelling, with a footnote. This reverses the earlier rule that the -iere spelling has no alternative.

## Obsolete forms (spec: *Obsolete forms*)
- **2026-10-11: an obsolete form is written as its current form**, and the switch offers its own spelling, with a footnote naming the current form („Forma *alineare* e învechită; forma actuală e *aliniere*.”). **Why:** the obsolete form is incorrect, and its current form is what the text means; the switch keeps the old spelling at hand.
  - The obsolete lemmas are listed by hand with their current lemma (*alinea* → *alinia*, *alineare* → *aliniere*, *alini* → *alina*, *rachiareasă* → *rachiereasă*), and each form is paired with the current lemma's form of the same inflection (*alineează* → *aliniază*, *alineând* → *aliniind*). **Why:** the two paradigms differ beyond the stem.
  - The old spelling follows the old syllabification (*alineare*: *a-li-ne-a-re*, a hiatus, not *alinēre*; *rachiareasă*: *rachiarēsă*).
  - A form that a current lemma also gives keeps that lemma's spelling, with no switch (*alineat*, the noun).
  - **2026-10-12:** *alini* is the obsolete form of *alina* (*alinesc* → *alin*). A form of several obsolete lemmas with different current forms is not rewritten: the switch offers its own spellings, the one of the lemma listed first in `rules::OBSOLETE` by default, and the footnote names every current form (*alineam*, of *alinea* and *alini*: *alineam* or *alinēm*; „forma actuală e *aliniam* sau *alinam*”). **Why:** the text alone does not say which verb was meant.
  - A word missing from the database with the stem of an obsolete lemma that dexonline lists without its forms takes the current stem (*rachiareasa* → *rachierēsa*, or *rachiarēsa*). **Why:** for the other lemmas every form is in the database, and the stem would only catch other words (*Alinei*, of *Alina*).
  - Long infinitives in -iare are obsolete forms too (*apreciare* → *apreciere*, *ambreiere*, *inventariere*, *înjunghiere*).

## Optional rule -eẹ/aẹ/oẹ (spec: *Optional rule: -eẹ/aẹ/oẹ*)
- **A separate switch, default off.** Word-final *-ee* is read as *-eie* and written *-eẹ* (*idee* → *ideẹ*), and the forms of *-ee* lemmas follow the *femeie* pattern (*ideea* → *ideė*, *ideei* → *ideii*).
  - **2026-10-04:** extended to the articulated plurals in *-eele*/*-eelor* (*orhideile*; *bombeẹle* ← *bombeu*); to the *-uri* plurals of *-eu* lemmas longer than 4 letters (*bombeuri* → *bombeẹ*; not *pneuri*); to articulated *-eea* and genitive *-eei* beyond *-ee* lemmas (*spontaneė*, *Andreė*, *Galileii*), except the pronouns *aceea*, *ceea*, *oareceea*.
  - **2026-10-12, reopened:** the genitives of *spontaneu*'s feminine and of *Andreea* are now *spontaneẹi*, *Andreẹi* (see the kept feminines below).
- **2026-10-12: the switch is named -eẹ/aẹ/oẹ, and extends to final -ae and -oe** (*Nae* → *Naẹ*, *Nicolae* → *Nicolaẹ*, *Zoe* → *Zoẹ*, *Noe* → *Noẹ*, *aloe* → *aloẹ*), read as *-aie*, *-oie*. **Why:** they are the same hiatus as *-ee*, written as the *-aie*/*-oie* that is said.
  - Only for the words of a hand list (`rules::AE_OE`). **Why:** most dexonline words in -ae/-oe are foreign (*vitae*, *reggae*, *Poe*, *Crusoe*), and nothing in the data tells them apart. Words with a foreign spelling are left out of the list: a *y*, *th*, *ph*, *rh*, or a *ch* not before e/i (*Pasiphae*, *Callirhoe*).
  - **-oae is a rule of its own, for every word:** *-ōẹ* (*Băboae* → *Băbōẹ*, *lighioae* → *lighiōẹ*). **Why:** the words in -oae are native.
  - Common nouns in -oe follow the *femeie* pattern: *aloea* → *aloė*, *aloei* → *aloii*, *aloele* → *aloile*, *aloelor* → *aloilor*. Feminine names of persons keep their e: *Zoei* → *Zoẹi*, *Cloei* → *Cloẹi*, *Zoele* → *Zoẹle*.
- **2026-10-12: feminines with a masculine, and feminine names of persons, keep their plural and genitive**, written with *ẹ*: *zeele* → *zeẹle*, *zeei* → *zeẹi* (← *zee*, with the masculine *zeu*), *ateẹlor*, *pigmeẹle*, *spontaneẹi* (← *spontaneu*); *Andreei* → *Andreẹi*, *Andreelor* → *Andreẹlor*, *Medeẹi*, *Leẹi*. Their articulated *-eea* stays *-eė* (*zeė*, *Andreė*), and the forms of *-eie* lemmas are already *-eẹ-* by the base rules (*zeiele*, *rotofeiele* → *zeẹle*, *rotofeẹle*). **Why:** their plural is their singular (*zee*, *zeie*, *atee*), not *-ei* like *femeie*/*femei*, so the articulated plural is *zeiele*; names keep the e of their nominative.
  - The masculine is a lemma in -eu that dexonline lists as a masculine noun or an adjective (*zeu*, *ateu*, *pigmeu*, *spontaneu*), not a neuter (*ateneu*, *propileu*, *apogeu*: *propilee* stays *propilei*). The bare plural follows the articulated one (*zee* → *zeẹ*). A look-alike that is another word is no masculine: *coree*, the disease, is not the feminine of *coreu*, the metrical foot (`rules::EE_NOT_MASCULINE`: *coreei* → *coreii*).
  - The names are a hand list (`rules::NAMES`). **Why:** dexonline doesn't tell persons from places, and places keep the *femeie* pattern (*Galileei* → *Galileii*).
  - **2026-10-12, the same day: the build adds the genitive and articulated plural of every name of `rules::NAMES` that dexonline lists without them** (*Andreei*, *Andreele*, *Andreelor* ← *Andreea*; *Zoei* ← *Zoe*), so they are known words, not flagged as unknown. A name missing from dexonline (*Cloe*) keeps its forms unknown, still converted by the lists.
- If one spelling is both a singular and a plural, the singular reading wins.
- **2026-10-11, deferred:** whether the -ee switch should reach words derived from an -ee/-eu stem. The plural *licee* (*liceẹ*) could give *liceėn*, *liceėl*, *interliceėl*, and *muzeė* could give *muzeėl*, *intermuzeėl*. Until it is decided, the switch leaves them as they are (*interliceal*, *muzeal*).

## â → î (spec: *Optional rule: â → î*)
- **2026-10-06: â is written î, except in words with the root *român*/*rumân*** (*câine* → *cîine*; *român*, *aromân*, *rumânesc* keep â). A separate switch, default on.
  - **Why:** â and î write the same sound.

## Unknown words (spec: *Unknown words*)
- **2026-09-27:** a word with a candidate digraph that is missing from the database goes through the fallback rules and is flagged. Before, it passed through unchanged.
- **2026-10-04:** with the -ee switch on, an unknown word in *-eei* or *-eele*/*-eelor* is first tied to a known lemma, which gives one spelling (*Andreei* ← *Andreea*, *bombeele* ← *bombeu*). If no lemma is found, the word gets both spellings on the homograph switch, the *-eele* case with a footnote to help choose.
  - **2026-10-12, reopened:** an unknown *-eei* with no lemma gets *-eẹi*, the default, or *-eii* (*Zzeẹi* or *Zzeii*), no longer itself: no form keeps *-eei* under the switch now. A name of `rules::NAMES` gives one spelling (*Andreẹi*, *Andreẹlor*, *Leẹi*), and so does a word of `rules::AE_OE` (*Zoei* → *Zoẹi*); any other word in -ae/-oe is left as it is. The masculine is not looked for. **Why:** every form of a masculine or adjective lemma is in dexonline.
  - **2026-10-12, the same day:** an unknown *-eele*/*-eelor* of a known *-eea* lemma that is no name, a place, is its plural, like *femeile*, before any *-eu* lemma (*Iudeelor* → *Iudeilor*, *Galileele* → *Galileile*), and *coree* keeps its exception there too (*coreelor* → *coreilor*). **Why:** *iudeu*, *Galileu* and *coreu* are false masculines; they gave *Iudeẹlor*, *coreẹlor*.
- **2026-10-06:** an unknown form of an *-iesc* adjective is tied to its lemma and derived from it (*harabagiasca* → *harabagiėsca* ← *harabagiesc*). **Why:** dexonline lists some of these adjectives with no forms, and the rules alone would miss both the hiatus and the suffix rule.

## Reverse conversion (spec: *Reverse conversion*)
- Concise → standard is currently a plain character map. Reverse conversion (*Concis → standard* in the web app) and its round-trip test are kept for now.
- **2026-10-06:** the ė of a final -iėză/-iėscă/-iėsca reverts to a, so the suffix rule stays reversible (*studiėză* → *studiază*). The only conflicts are *fiiască*, *fiiasca* (*fiėscă*, *fiėsca*), which revert to *fiască*, *fiasca*.
- **2026-10-09: a long infinitive in -iare is not restored** (*apreciere* stays *apreciere*).
- **2026-10-06: î is left unchanged**, so â is not restored after the â → î rule (*cîine* stays *cîine*).
- **2026-10-07: with the -ee switch on, revert undoes the -ee rule by lookup** (*orhideẹ* → *orhidee*). When several forms give the same spelling, a DEX-MDA-DOOM form wins (a definition from MDA\*, DEX\*, NODEX\* or DOOM\*: *aleẹ* → *alee*, not *aleie*), then a plural in -ee beats one in -euri when both are DEX-MDA-DOOM (*apogeẹ* → *apogee*). **Why:** the variants outside DEX-MDA-DOOM (*aleie*, *chee*, Scriban only) are not the current word. Unknown words keep the character map.
  - **2026-10-07, extended the same day:** DOOM\* counts too (DEX-MDA-DOOM).
  - **2026-10-07, the same day:** only a definition of its own counts (the word is its headword), not a mention in another word's definition (*acneie*, under *coș* in MDA2). The forms still tied after both criteria are all offered on the homograph switch (*bordeẹ*: *bordeie* or *bordee*). **Why:** both are current words, and the text alone does not say which one was meant.
  - **2026-10-12: the spellings of the -eẹ/aẹ/oẹ extension that read, by the character map, as an existing word in -aie/-oaie/-eie** (*Naẹ*: *naie*; *zeẹi*: *zeiei*) revert by whether their source is a name, not by DEX-MDA-DOOM. A name (with a capital in dexonline, not in -oae) reverts to itself, the default, with the -aie word on the switch (*Naẹ*: *Nae* or *naie*; *Nicolaẹ*: *Nicolae* or *Nicolaie*). A common word, or any word in -oae, is an old spelling of the -aie word, and reverts to it with no switch (*baẹ* → *baie*, *Fōẹ* → *foaie*, *oboẹ* → *oboie*, *zeẹi* → *zeiei*, *aloii* → *aloii*). **Why:** names have no DEX definitions, so DEX-MDA-DOOM would always give the -aie word (*Naie*); the common words are old spellings, whose current form is what the text means. The ties of the -ee rule as it was before stay as they are (*bordeẹ*: *bordeie* or *bordee*).
  - **2026-10-12, the same day: a plural in -eie that is DEX-MDA-DOOM beats the -ee plural of the same lemma** (*curcubeẹ* → *curcubeie*, *curcubeẹle* → *curcubeiele*, not *curcubee*, *curcubeele*; *chindeẹle* → *chindeiele*). The -ee and -eie plurals of two different lemmas stay a tie (*bordeẹ*: *bordeie* or *bordee*), and so do those of one lemma when the -eie one is not DEX-MDA-DOOM. **Why:** the dictionaries give the plural in -eie (DEX '98: *curcubeu, curcubeie*).
  - **2026-10-12, the same day: the spellings of a name of `rules::NAMES` or a word of `rules::AE_OE` that are missing from the database revert to their form too** (*Andreẹi* → *Andreei*, *Leẹi* → *Leei*, *Andreẹlor* → *Andreelor*, *Zoẹi* → *Zoei*, *Cloẹ* → *Cloe*), with their other digraphs written or not (*Nēgoẹ*, *Neagoẹ* → *Neagoe*), unless their character-map reading is a known word (*Mamaẹi* → *Mamaiei*). **Why:** dexonline has the names (*Andreea*, *Zoe*) but not their genitives and plurals, so the character map gave *Andreiei*, *Zoiei*.
- It is not guaranteed to stay deterministic.

## Platforms and stack
- **2026-10-09: one Rust core replaces Python.** The rules, conversion, reverse conversion and the pipeline move to Rust. Python stays only as the reference until the Rust port gives the same output for every form in the database, then it is removed. **Why:** the rules must run offline on the web, Android and iOS, and they change often. One implementation called from every platform avoids keeping copies in sync.
  - **2026-10-09, the same day: once Rust matches Python, the Python rules are frozen** (`rules.py`, `db.py`, `server/`). Rule changes then go into Rust only, and the Flask app, which is not deployed, keeps the last Python rules until the PWA replaces it; Python is removed then. **Why:** two implementations would double the work of every rule change, and no deployed site depends on the Python version.
    - **2026-10-10: removed**, with the Flask app, its Docker image and the Python CI job, once the PWA replaced them. **Why:** nothing ran on them any more, and the round-trip test is in Rust.
  - **2026-10-09, the same day: the Python pipeline stays editable until the Rust pipeline replaces it.** **Why:** dexonline's data and the record flags come in through it, and a rule may need a new flag before the Rust pipeline exists.
    - **2026-10-10: replaced and removed.** The frozen Flask app keeps the last `word_db.sqlite` the Python pipeline built. **Why:** the Rust pipeline builds the same database from MariaDB as from the Python pipeline's records, byte for byte (292,128 forms of the 2026-10-04 dump), so nothing needs the Python one any more. Spec: *Data*.
  - **2026-10-09, the same day: the first step ports the database builder (`db.py`) along with the rules.** **Why:** how the default spelling, the alternatives and the reverse -ee form are chosen is part of the spec, and most test tables need a built database.
  - **2026-10-09, the same day: the data file format is chosen with the Rust pipeline**, before the web app. **Why:** the pipeline writes the file, and what it must support (WASM, memory mapping on iOS) is already known.
    - **2026-10-10: the data file is `grafia.dat`, maps built with the `fst` crate, not SQLite.** **Why:** SQLite in WASM needs a C build and a virtual file system, while only exact-key lookups are needed; `fst` is pure Rust, and its maps are read in place, so the iOS keyboard can memory-map the file. It is also smaller: 0.8 MB, 0.56 MB gzipped, against 9.1 MB and 2.5 MB for `word_db.sqlite` (2026-10-04 dump). Spec: *Data*.
    - **2026-10-10: the data file records the rules it was built with**, a hash of their code, and the web app's build refuses one built with other rules. **Why:** CI can't rebuild the data file, so a rule change pushed without a rebuilt one would deploy the old spellings of every known word. The hash is computed, not a version number bumped by hand, which could be forgotten just like the rebuild; a change to the code that leaves the output as it is also asks for a rebuild. Spec: *Data*.
  - **2026-10-09, the same day: iOS comes before Android.** **Why:** Cezar's own phone is an iPhone, so the iOS app can be tested in daily use first.
- **2026-10-09: the website becomes a serverless PWA.** The core runs in the browser as WASM, and the data file is cached for offline use. **Why:** it works offline and needs no server. The download is small (the current database is 1.7 MB with brotli), and the data is GPL anyway.
  - **2026-10-10: the PWA is hosted on GitHub Pages**, the first deployment of the web app. **Why:** the PWA is a static site, which GitHub Pages hosts for free, published by the repository's own CI.
- **2026-10-09: Android gets a text processing action** (*grafia concisă* in the text selection menu), not a keyboard. **Why:** it replaces the selected text in place in almost every app, and the user keeps their own keyboard.
  - **2026-10-09, the same day: the selection is replaced at once when every word has one known spelling.** With a homograph or an unknown word, a dialog first shows the switches, as on the web. **Why:** the usual case stays one tap, and the alternatives are still offered.
- **2026-10-09: iOS gets a keyboard with two modes** (or two keyboards). One mode converts each word as it is typed and offers Romanian word predictions. The other has the concise letters ẹ, ė, ē, ō and converts nothing. **Why:** iOS has no in-place text processing for other apps, so only a keyboard can write into them.
  - The keyboard never needs Full Access: everything is offline.
  - **2026-10-09, the same day: one keyboard, with a key that switches modes** and remembers the mode. **Why:** the user keeps one keyboard in the globe rotation, and the word data is loaded only in the converting mode.
  - **2026-10-09, the same day: the predictions come from `UITextChecker`**, the system's spell-checking lexicon, once a check on a device confirms it has Romanian. **Why:** third-party keyboards don't get the system's predictions, and `UITextChecker` needs no data of our own.
  - **2026-10-09, the same day: during development the app is installed with a free Apple ID.** The paid Apple Developer Program comes with the first TestFlight or App Store release. **Why:** Apple lets iPhone owners install the apps they develop on their own iPhone.
- The mobile apps are thin native layers (Kotlin, Swift) over the Rust core.
- **2026-10-09: possible later targets: Windows and macOS keyboards.** They will use the same Rust core. Not decided yet.

## Licensing
- dexonline data is GPL, and attribution is required in the web app and in the mobile apps.
- The mobile apps are planned as open source, with ads only to cover the Apple developer fees. This is GPL-compatible as long as the attribution is kept.
