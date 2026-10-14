//! The hand lists: decisions of Cezar, by word.

// A long infinitive in -iere after p/b/f/v/m or c/g/ch/gh is a hiatus in all its forms
// (copiere, învierea, aprecierilor), except in these native words and their forms
// (mierea, pieri). After a vowel it is a diphthong anyway (tăiere, înmuiere).
pub const IERE_ENDINGS: [&str; 6] = ["e", "ea", "i", "ii", "ile", "ilor"]; // after -ier: -iere, -ierea, -ieri, ...
// After -ier, ending an agent noun: -ier, -ieră, -iereasă and its forms (plumier, legumieră,
// rachiereasă, rachieresele)
pub const IER_ENDINGS: [&str; 8] = ["", "ă", "easă", "easa", "ese", "esei", "esele", "eselor"];
pub const IERE_DIPHTHONG: [&str; 4] = ["miere", "înmiere", "fiere", "piere"];
// Lemmas in -iare that are no long infinitives (fiare: s.f. pl.; mobiliare: s.n.)
pub const IARE_KEPT: [&str; 2] = ["fiare", "mobiliare"];

// Obsolete lemmas: (the lemma, syllabified, its current lemma, the stem its forms start
// with, whether dexonline lists it without its forms). A form of one is written as the
// current lemma's form with the same inflection (alineează -> aliniėză), and offers its own
// spelling, from this syllabification, on the switch (alineēză). Words with the stem are
// looked up even with no candidate digraph (alineez). For a lemma listed without its forms,
// a word missing from the database that starts with the stem takes the current lemma's
// stem instead (rachiareasa -> rachiereasa, or rachiarēsa); for the others it would only
// catch other words (Alinei). A form of several obsolete lemmas with different current
// lemmas is not rewritten: its own spellings, in the order of this list, are offered
// (alineam, of alinea and alini: alineam or alinēm).
pub const OBSOLETE: &[(&str, &str, &str, bool)] = &[
    ("a-li-ne-a", "alinia", "aline", false),
    ("a-li-ne-a-re", "aliniere", "aline", false),
    ("a-li-ni", "alina", "alin", false), // its imperfect is alinea's: alineam (alinea first)
    ("ra-chi-a-rea-să", "rachiereasă", "rachiare", true),
];

// Headwords in -eea take -eė with the -ee switch (Niceė, Andreė), except the
// pronoun aceea and its kin, which keep -eē.
pub const EEA_UNCHANGED: [&str; 2] = ["aceea", "oareceea"];
// Lemmas whose final -ae/-oe the -ee switch writes -aẹ/-oẹ (Nae -> Naẹ, aloe -> aloẹ); the forms
// of those in -oe follow femeie (aloea -> aloė, aloei -> aloii, aloele -> aloile), or, for a
// name in NAMES, keep their e (Zoei -> Zoẹi). Most other dexonline words in -ae/-oe are foreign
// (vitae, reggae, Poe); those with a foreign spelling (y, th, ph, rh, ch not before e/i) are left
// out. Words in -oae need no entry: -oae is -ōẹ in any word (Băboae).
#[rustfmt::skip]
pub const AE_OE: &[&str] = &[
    // names in -ae
    "nae", "necolae", "neculae", "nicolae", "niculae", "lae", "vlae", "brătae", "drăgae", "vlădae", "dănae", "străhae",
    "zgae", "bodogae", "băzdigae", "călae", "codae", "cobae", "decae", "flăndrae", "nămae", "halbae", "bae", "dae", "mae",
    "rae", "brae", "vrae", "andrae", "andornae", "aglae", "danae", "pasifae",
    // common words in -ae (variants of those in -aie)
    "babae", "bălae", "bălbătae", "bețae", "ciumăfae", "cusurae", "cârnae", "dudae", "dugae", "dăblae", "fae", "gae",
    "ghilăvae", "lețcae", "lăscae", "mereae", "mesereae", "nedeae",
    // names in -oe
    "noe", "zoe", "cloe", "arsinoe", "autonoe", "beroe", "ociroe", "teonoe", "averoe", "cosroe", "feroe", "neagoe",
    "stroe", "sisoe", "rădivoe", "popoe", "stăncoe", "brătoe", "banboe", "bartoe", "begoe", "beloe", "bogoe", "bucoe",
    "buhoe", "băloe", "băndoe", "bășcoe", "calcoe", "cascoe", "cragoe", "docoe", "doljoe", "geagoe", "gheoroe", "goe",
    "grigoe", "găfoe", "hodoe", "hosmăndioe", "ileroe", "iricioe", "lențoe", "lifoe", "mormoe", "mârmoe", "măndroe",
    "mănoe", "nacoe", "papoe", "pipoe", "pitolcoe", "pleșoe", "rostoe", "sicoe", "teahoe", "temoe", "troe", "usoe",
    "verevoe", "vlădoe", "voe", "țipoe",
    // common words in -oe
    "aloe", "benzoe", "canoe", "diploe", "meloe", "hoboe", "eroe", "geroe", "oboe", "evoe",
];
// Feminines in -ee whose -eu look-alike is another word, not their masculine: coree (the
// disease; coreu is the metrical foot). They follow femeie: coreei -> coreii.
pub const EE_NOT_MASCULINE: [&str; 1] = ["coree"];
// Feminine names of persons. With the -ee switch they keep the e of their genitive and
// articulated plural, written ẹ: Andreei -> Andreẹi, Andreelor -> Andreẹlor, Zoei -> Zoẹi (not
// Andreii, Zoii, like femeii). dexonline doesn't tell persons from places, which follow femeie
// (Galileei -> Galileii).
#[rustfmt::skip]
pub const NAMES: &[&str] = &[
    // in -eea, -ee
    "amalteea", "andreea", "casiopeea", "deleneea", "doroteea", "dulcineea", "euricleea", "filoteea", "galateea", "geea",
    "idoteea", "leea", "leucoteea", "medeea", "miraneea", "nesteea", "panopeea", "pasiteea", "pentesileea", "peribeea",
    "popeea", "salomeea", "stenebeea", "darclee",
    // in -oe (also in AE_OE)
    "zoe", "cloe", "arsinoe", "autonoe", "beroe", "ociroe", "teonoe",
];
// A digraph right after one of these is a hiatus when the rest is a known word
// (re-abilitat, auto-analiză). The rest must have at least 4 letters (not neam,
// autoare), and prea- + a known word is one syllable (prea-curat, not pre-acurat).
pub const PREFIXES: &[&str] = &[
    "re", "pre", "ne", "tele", "co", "semi", "anti", "multi", "auto", "micro", "macro", "foto", "pseudo", "bio", "neo",
    "radio", "aero", "agro", "electro", "hidro", "termo", "geo", "proto", "socio", "psiho", "stereo",
];
// Syllabifications decided by Cezar. They beat both dexonline and the rules,
// and for a form they give all its spellings: other lemmas' records add none.
// Several readings = a homograph: the first is the default, the others are
// offered as alternatives.
pub const SYLLABIFICATIONS: &[(&str, &[&str])] = &[
    ("neant", &["ne-ant"]),
    ("înfieri", &["în-fi-e-ri"]),
    ("mielea", &["mi-e-lea"]),     // al mielea, like al miilea
    ("piei", &["piei"]),           // not pi-ei (<- pia)
    ("răspieri", &["răs-pie-ri"]), // răspĭér, a -í (a verb, no long infinitive)
    ("vier", &["vier", "vi-er"]),  // the boar; păzitor de vie (rare)
    ("vieri", &["vieri", "vi-e-ri"]),
    ("viere", &["vi-e-re"]),           // the long infinitive (not the boar's vocative)
    ("ia", &["ia", "i-a"]),            // the verb lua, the interjection; the articulated ie
    ("miază", &["mia-ză", "mi-a-ză"]), // noon (amiază), myiasis
    ("ie", &["i-e", "ie"]),            // the blouse (articulated ia: i-a); the adverb ie
    // co- + a known word, but not a prefix
    ("coardă", &["coar-dă"]),
    ("coastă", &["coas-tă"]),
    ("coacere", &["coa-ce-re"]),
    ("coacăț", &["coa-căț"]),
    ("coacăță", &["coa-că-ță"]),
];

/// The readings of `word` in SYLLABIFICATIONS, if listed.
pub fn syllabifications(word: &str) -> Option<&'static [&'static str]> {
    SYLLABIFICATIONS.iter().find(|(w, _)| *w == word).map(|(_, r)| *r)
}

/// The endings of an -esc adjective (fiesc: fiești, fiescu, ..., fiască, fiasca)
pub const IESC_ENDINGS: [&str; 9] = [
    "ești", "escu", "escul", "escului", "eștii", "eștile", "eștilor", "ască", "asca",
];
