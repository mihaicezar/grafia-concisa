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
- **2026-10-05: final -ii after a consonant is a hiatus in verb infinitives only** (*sfi-i*, *pri-i*; not *copii*). **Why:** dexonline syllabifies none of the -ii verbs, and without the cut their forms came out as diphthongs (*sfiește*, *priască* are *sfi-eș-te*, *pri-as-că*).

## Long infinitives in -iare (spec: *Syllabification source*)
- **2026-10-09: a long infinitive in -iare is written -iere**, then follows the -iere rules (*apreciare* → *apreciere*, *tăiare* → *tăẹre*). **Why:** long infinitives in -iare are incorrect; the correct form always ends in -iere.
  - A form that another lemma also gives keeps that lemma's spelling (*conciliare*, the adjective). **Why:** the other word is the current one.
  - It replaces the earlier diphthong reading of -iare after a labial or palatal consonant, which now applies only to words that are not long infinitives (*fiare*).

## Optional -ee rule (spec: *Optional rule: final -ee*)
- **A separate switch, default off.** Word-final *-ee* is read as *-eie* and written *-eẹ* (*idee* → *ideẹ*), and the forms of *-ee* lemmas follow the *femeie* pattern (*ideea* → *ideė*, *ideei* → *ideii*).
  - **2026-10-04:** extended to the articulated plurals in *-eele*/*-eelor* (*orhideile*; *bombeẹle* ← *bombeu*); to the *-uri* plurals of *-eu* lemmas longer than 4 letters (*bombeuri* → *bombeẹ*; not *pneuri*); to articulated *-eea* and genitive *-eei* beyond *-ee* lemmas (*spontaneė*, *Andreė*, *Galileii*), except the pronouns *aceea*, *ceea*, *oareceea*.
- If one spelling is both a singular and a plural, the singular reading wins.

## â → î (spec: *Optional rule: â → î*)
- **2026-10-06: â is written î, except in words with the root *român*/*rumân*** (*câine* → *cîine*; *român*, *aromân*, *rumânesc* keep â). A separate switch, default on.
  - **Why:** â and î write the same sound.

## Unknown words (spec: *Unknown words*)
- **2026-09-27:** a word with a candidate digraph that is missing from the database goes through the fallback rules and is flagged. Before, it passed through unchanged.
- **2026-10-04:** with the -ee switch on, an unknown word in *-eei* or *-eele*/*-eelor* is first tied to a known lemma, which gives one spelling (*Andreei* ← *Andreea*, *bombeele* ← *bombeu*). If no lemma is found, the word gets both spellings on the homograph switch, the *-eele* case with a footnote to help choose.
- **2026-10-06:** an unknown form of an *-iesc* adjective is tied to its lemma and derived from it (*harabagiasca* → *harabagiėsca* ← *harabagiesc*). **Why:** dexonline lists some of these adjectives with no forms, and the rules alone would miss both the hiatus and the suffix rule.

## Reverse conversion (spec: *Reverse conversion*)
- Concise → standard is currently a plain character map. The `/api/revert` endpoint and the round-trip test are kept for now.
- **2026-10-06:** the ė of a final -iėză/-iėscă/-iėsca reverts to a, so the suffix rule stays reversible (*studiėză* → *studiază*). The only conflicts are *fiiască*, *fiiasca* (*fiėscă*, *fiėsca*), which revert to *fiască*, *fiasca*.
- **2026-10-09: a long infinitive in -iare is not restored** (*apreciere* stays *apreciere*).
- **2026-10-06: î is left unchanged**, so â is not restored after the â → î rule (*cîine* stays *cîine*).
- **2026-10-07: with the -ee switch on, revert undoes the -ee rule by lookup** (*orhideẹ* → *orhidee*). When several forms give the same spelling, a DEX-MDA-DOOM form wins (a definition from MDA\*, DEX\*, NODEX\* or DOOM\*: *aleẹ* → *alee*, not *aleie*), then a plural in -ee beats one in -euri when both are DEX-MDA-DOOM (*apogeẹ* → *apogee*). **Why:** the variants outside DEX-MDA-DOOM (*aleie*, *chee*, Scriban only) are not the current word. Unknown words keep the character map.
  - **2026-10-07, extended the same day:** DOOM\* counts too (DEX-MDA-DOOM).
  - **2026-10-07, the same day:** only a definition of its own counts (the word is its headword), not a mention in another word's definition (*acneie*, under *coș* in MDA2). The forms still tied after both criteria are all offered on the homograph switch (*bordeẹ*: *bordeie* or *bordee*). **Why:** both are current words, and the text alone does not say which one was meant.
- It is not guaranteed to stay deterministic.

## Platforms and stack
- **2026-10-09: one Rust core replaces Python.** The rules, conversion, reverse conversion and the pipeline move to Rust. Python stays only as the reference until the Rust port gives the same output for every form in the database, then it is removed. **Why:** the rules must run offline on the web, Android and iOS, and they change often. One implementation called from every platform avoids keeping copies in sync.
  - **2026-10-09, the same day: once Rust matches Python, the Python rules are frozen** (`rules.py`, `db.py`, `server/`). Rule changes then go into Rust only, and the Flask app, which is not deployed, keeps the last Python rules until the PWA replaces it; Python is removed then. **Why:** two implementations would double the work of every rule change, and no deployed site depends on the Python version.
  - **2026-10-09, the same day: the Python pipeline stays editable until the Rust pipeline replaces it.** **Why:** dexonline's data and the record flags come in through it, and a rule may need a new flag before the Rust pipeline exists.
    - **2026-10-10: replaced and removed.** The frozen Flask app keeps the last `word_db.sqlite` the Python pipeline built. **Why:** the Rust pipeline builds the same database from MariaDB as from the Python pipeline's records, byte for byte (292,128 forms of the 2026-10-04 dump), so nothing needs the Python one any more. Spec: *Data*.
  - **2026-10-09, the same day: the first step ports the database builder (`db.py`) along with the rules.** **Why:** how the default spelling, the alternatives and the reverse -ee form are chosen is part of the spec, and most test tables need a built database.
  - **2026-10-09, the same day: the data file format is chosen with the Rust pipeline**, before the web app. **Why:** the pipeline writes the file, and what it must support (WASM, memory mapping on iOS) is already known.
    - **2026-10-10: the data file is `grafia.dat`, maps built with the `fst` crate, not SQLite.** **Why:** SQLite in WASM needs a C build and a virtual file system, while only exact-key lookups are needed; `fst` is pure Rust, and its maps are read in place, so the iOS keyboard can memory-map the file. It is also smaller: 0.8 MB, 0.56 MB gzipped, against 9.1 MB and 2.5 MB for `word_db.sqlite` (2026-10-04 dump). Spec: *Data*.
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
