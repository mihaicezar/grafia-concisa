//! grafia concisă — table-driven test suite (the port of server/test_server.py).
//!
//! Run: cargo test --workspace
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::sync::LazyLock;

use grafia_concisa::build::{Record, Tables, build};
use grafia_concisa::rules::{
    concise_ee, concise_from_reference, concise_from_syllables, iere_spelling, long_infinitive, revert_text, syllabify,
};
use grafia_concisa::{Options, convert, lookup, revert};

// (syllabified, expected concise). Add a case = add a line.
const WORDS: &[(&str, &str)] = &[
    // diphthongs, stressed
    ("soa-re", "sōre"),
    ("sea-ră", "sēră"),
    ("mea", "mē"),
    ("fier", "fẹr"),
    ("iar-bă", "ėrbă"),
    ("ceai", "cēi"),
    // diphthongs, unstressed
    ("cea-sor-nic", "cēsornic"),
    ("ia-urt", "ėurt"),
    ("ie-pu-raș", "ẹpuraș"),
    ("foar-fe-cu-ță", "fōrfecuță"),
    ("cea-i-nă-ri-e", "cēinărie"),
    // hiatus: unchanged
    ("fi-e-ca-re", "fiecare"),
    ("re-al", "real"),
    ("te-a-tru", "teatru"),
    ("pi-an", "pian"),
    ("bo-a", "boa"),
    ("vi-e", "vie"),
    ("co-a-li-ți-e", "coaliție"),
    // hiatus + diphthong in one word
    ("a-ce-ea", "aceē"),
    // triphthongs: left to right, no overlap
    ("leoar-că", "leōrcă"),
    ("a-ri-pioa-ră", "aripiōră"),
    ("beai", "bēi"),
    ("iei", "ẹi"),
    ("ploa-ie", "plōẹ"),
    ("șoa-re-ce", "șōrece"),
    ("ie-pu-re", "ẹpure"),
    ("pâi-nea", "pâinē"), // â is the â -> î switch's job, applied after the lookup
];

// Forms without their own syllabification: cuts come from the lemma.
// (form, lemma syllabified, expected)
const DERIVED: &[(&str, &str, &str)] = &[
    ("soarele", "soa-re", "sōrele"), // shared prefix: lemma decides
    ("reală", "re-al", "reală"),
    ("familia", "fa-mi-li-e", "familia"), // ending replaces a vowel: lemma decides
    ("familiei", "fa-mi-li-e", "familiei"),
    ("femeia", "fe-me-ie", "femeė"),
    ("ploaia", "ploa-ie", "plōė"),
    ("ideea", "i-de-e", "ideē"), // appended ending: diphthong
    ("creează", "cre-a", "creēză"),
    ("înșeală", "în-șe-la", "înșēlă"), // stem alternation: diphthong
    ("doarme", "dor-mi", "dōrme"),
    ("ploaie", "plo-ua", "plōẹ"), // o/e before a replaced vowel: diphthong
    ("scheaună", "sche-u-na", "schēună"),
    ("taie", "tă-ia", "taẹ"),
    ("argintie", "ar-gin-tiu", "argintie"), // -iu lemma: hiatus
    ("cheflia", "che-fliu", "cheflia"),
    ("sfiește", "sfi-i", "sfiește"), // -ii verb: lemma decides
    ("sfiască", "sfi-i", "sfiėscă"), // -ează/-ească after a hiatus i: -ėză, -ėscă
    ("priască", "pri-i", "priėscă"),
    ("studiază", "stu-di-a", "studiėză"),
    ("pliază", "pli-a", "pliėză"),
    ("remaniază", "re-ma-ni-a", "remaniėză"),
    ("fiască", "fi-esc", "fiėscă"),
    ("fiasca", "fi-esc", "fiėsca"),
];

// -ee rule: (form, lemma syllabified, number, expected with switch)
const EE_WORDS: &[(&str, &str, &str, &str)] = &[
    ("idee", "i-de-e", "S", "ideẹ"),
    ("alee", "a-le-e", "S", "aleẹ"),
    ("bombee", "bom-beu", "P", "bombeẹ"),
    ("bombeele", "bom-beu", "P", "bombeẹle"),
    ("bombeelor", "bom-beu", "P", "bombeẹlor"),
    ("liceele", "li-ceu", "P", "liceẹle"),
    ("liceelor", "li-ceu", "P", "liceẹlor"),
    ("bombeuri", "bom-beu", "P", "bombeẹ"), // -eu lemma > 4 letters
    ("bombeurile", "bom-beu", "P", "bombeẹle"),
    ("liceurilor", "li-ceu", "P", "liceẹlor"),
    ("pneurile", "pneu", "P", "pneurile"), // pneu: 4 letters
    ("pneuri", "pneu", "P", "pneuri"),
    ("logoree", "lo-go-re-e", "P", "logorei"), // plural read first...
    ("logoree", "lo-go-re-e", "S", "logoreẹ"), // ...singular wins
    ("femeie", "fe-me-ie", "S", "femeẹ"),      // base rules already
    ("ideea", "i-de-e", "S", "ideė"),          // like femeia
    ("ideei", "i-de-e", "S", "ideii"),         // like femeii
    ("orhideea", "or-hi-de-e", "S", "orhideė"),
    ("niceea", "ni-ce-ea", "S", "niceė"), // headword in -eea
    ("andreea", "an-dre-ea", "S", "andreė"),
    ("aceea", "a-ce-ea", "S", "aceē"),               // ...except the pronoun
    ("spontaneea", "spon-ta-neu", "S", "spontaneė"), // -eu adjective
    ("galileei", "ga-li-le-ea", "S", "galileii"),    // -eei: same lemmas as -eea
    ("spontaneei", "spon-ta-neu", "S", "spontaneii"),
    ("plebeea", "ple-beu", "S", "plebeė"),
    ("orhideei", "or-hi-de-e", "S", "orhideii"),
    ("orhideele", "or-hi-de-e", "P", "orhideile"),   // like femeile
    ("orhideelor", "or-hi-de-e", "P", "orhideilor"), // like femeilor
];

// (word, dexonline hyphenation, expected syllabified, expected source, "V" for a verb).
// Outside dexonline's span, rules place only the cuts between vowels.
const SYLLABIFY: &[(&str, &str, &str, &str, &str)] = &[
    ("familie", "fa-mi-li-e", "fa-mi-li-e", "dexonline", ""),
    ("familie", "-li-e", "fami-li-e", "partial", ""), // -x: word end
    ("acetaldehidă", "a-ce-tal-", "a-ce-tal-dehidă", "partial", ""), // x-: word start
    ("aripioară", "-pioa-", "ari-pioa-ră", "partial", ""), // -x-: word interior
    ("biologie", "-lo-gi-e, -log-i-e", "bio-lo-gi-e", "partial", ""), // first variant
    ("soare", "-xy-", "soare", "rules", ""),          // does not fit: ignored
    ("soare", "", "soare", "rules", ""),              // oa, ea: diphthong
    ("seară", "", "seară", "rules", ""),
    ("iarbă", "", "iarbă", "rules", ""),      // ie/ia at word start
    ("femeie", "", "femeie", "rules", ""),    // ie/ia after a vowel
    ("familie", "", "famili-e", "rules", ""), // final, after a consonant
    ("vie", "", "vi-e", "rules", ""),
    ("fier", "", "fier", "rules", ""), // medial, after p/b/f/v/m
    ("piept", "", "piept", "rules", ""),
    ("client", "", "cli-ent", "rules", ""), // medial, other consonant
    ("social", "", "soci-al", "rules", ""),
    ("idee", "", "ide-e", "rules", ""), // e-e: hiatus
    ("aceea", "", "ace-ea", "rules", ""),
    ("ambient", "", "ambi-ent", "rules", ""), // -ient, -ienț...: hiatus
    ("experiență", "", "experi-ență", "rules", ""),
    ("clientelă", "", "cli-entelă", "rules", ""),
    ("farniente", "far-nien-te", "far-nien-te", "dexonline", ""), // dexonline prevails
    ("sfii", "", "sfi-i", "rules", "V"),                          // final -ii in a verb: hiatus
    ("prii", "", "pri-i", "rules", "V"),
    ("copii", "", "copii", "rules", ""),    // ...not in other words
    ("bienal", "", "bi-enal", "rules", ""), // prefix bi-
    ("real", "", "re-al", "rules", ""),     // ea: EA_HIATUS neologisms
    ("boreal", "", "bore-al", "rules", ""),
    ("leal", "", "le-al", "rules", ""),
    ("linear", "", "line-ar", "rules", ""),
    ("nuclear", "", "nucle-ar", "rules", ""),
    ("alineat", "", "aline-at", "rules", ""),
    ("realizat", "", "re-alizat", "rules", ""),
    ("cereale", "", "cere-ale", "rules", ""),
    ("denuclearizat", "", "denucle-arizat", "rules", ""),
    ("pancreatic", "", "pancre-atic", "rules", ""),
    ("reactor", "", "re-actor", "rules", ""),
    ("aleator", "", "ale-ator", "rules", ""),
    ("deal", "", "deal", "rules", ""), // ...not native words
    ("podbeal", "", "podbeal", "rules", ""),
    ("beat", "", "beat", "rules", ""),
    ("asear", "", "asear", "rules", ""),
    ("dogoreală", "", "dogoreală", "rules", ""),
    ("bianual", "", "bi-anual", "rules", ""),
    ("coagulat", "", "co-agulat", "rules", ""), // oa: OA_HIATUS neologisms
    ("protozoar", "", "protozo-ar", "rules", ""),
    ("mozoară", "", "mozoară", "rules", ""),
    ("biet", "", "biet", "rules", ""),
    ("biată", "", "biată", "rules", ""),    // IA_DIPHTHONG stem
    ("fiabil", "", "fi-abil", "rules", ""), // -iabil, -eabil: hiatus
    ("agreabil", "", "agre-abil", "rules", ""),
    ("copiere", "", "copi-ere", "rules", ""), // final -iere: hiatus...
    ("veghiere", "", "veghi-ere", "rules", ""),
    ("miere", "", "miere", "rules", ""), // ...except IERE_DIPHTHONG
    ("înmiere", "", "înmiere", "rules", ""),
    ("xerocopiere", "", "xerocopi-ere", "rules", ""),
    ("învierea", "", "învi-erea", "rules", ""), // articulated: -ierea
    ("mierea", "", "mierea", "rules", ""),
    ("copierilor", "", "copi-erilor", "rules", ""), // ...and all its endings
    ("pieri", "", "pieri", "rules", ""),            // (piere: IERE_DIPHTHONG)
    ("schierea", "", "schi-erea", "rules", ""),
    ("tăiere", "", "tăiere", "rules", ""), // vowel + -iere: diphthong
    ("înmuiere", "", "înmuiere", "rules", ""),
    ("bărbier", "", "bărbi-er", "rules", ""), // except final -ier/-ieră after
    ("plumier", "", "plumi-er", "rules", ""), // a vowel and a consonant...
    ("legumieră", "", "legumi-eră", "rules", ""),
    ("zbier", "", "zbier", "rules", ""),          // (not fier, zbier)
    ("bărbierit", "", "bărbi-erit", "rules", ""), // ...IE_HIATUS stems...
    ("zbieret", "", "zbieret", "rules", ""),
    ("igienă", "", "igi-enă", "rules", ""),
    ("mielită", "", "mi-elită", "rules", ""),
    ("miel", "", "miel", "rules", ""),
    ("așchietor", "", "așchi-etor", "rules", ""),
    ("sovietism", "", "sovi-etism", "rules", ""),
    ("zapciesc", "", "zapci-esc", "rules", ""), // ...and adjectives in -iesc
    ("surugiesc", "", "surugi-esc", "rules", ""),
    ("ciocoiesc", "", "ciocoiesc", "rules", ""), // (after a vowel: diphthong)
    ("apropiat", "", "apropi-at", "rules", ""),  // medial ia after a consonant:
    ("pian", "", "pi-an", "rules", ""),          // hiatus (-ia* endings)
    ("deviant", "", "devi-ant", "rules", ""),
    ("fiare", "", "fiare", "rules", ""),      // final -iare after p/b/f/v/m, c/g
    ("variare", "", "vari-are", "rules", ""), // ...but not after other consonants
    ("specializat", "", "speci-alizat", "rules", ""),
    ("îmbiat", "", "îmbi-at", "rules", ""),
    ("biatlon", "", "bi-atlon", "rules", ""),
    ("miazmă", "", "mi-azmă", "rules", ""),
    ("chiar", "", "chiar", "rules", ""), // ...except IA_DIPHTHONG stems
    ("piatră", "", "piatră", "rules", ""),
    ("amiază", "", "amiază", "rules", ""),
    ("maghiar", "", "maghiar", "rules", ""),
    ("chiajna", "", "chiajna", "rules", ""),
    ("obială", "", "obială", "rules", ""),
    ("microbial", "", "microbi-al", "rules", ""),
    ("schiagramă", "", "schi-agramă", "rules", ""),
    ("liturghiar", "", "liturghi-ar", "rules", ""),
    ("eleghiacesc", "", "eleghi-acesc", "rules", ""),
    ("miază", "mi-a-ză", "mia-ză", "manual", ""),
    ("cartier", "", "carti-er", "rules", ""),       // -tier: after t anyway
    ("reabilitat", "", "re-abilitat", "rules", ""), // prefix + known word
    ("autoanaliză", "", "auto-analiză", "rules", ""),
    ("neam", "", "neam", "rules", ""), // rest too short
    ("autoare", "", "autoare", "rules", ""),
    ("preacurat", "", "preacurat", "rules", ""), // prea- + curat, not pre- + acurat
    ("preaviz", "", "pre-aviz", "rules", ""),
    ("reazem", "", "reazem", "rules", ""),    // rest not a known word
    ("coace", "", "coace", "rules", ""),      // rest too short
    ("coactiv", "", "co-activ", "rules", ""), // co- prefix
    ("neant", "", "ne-ant", "manual", ""),    // SYLLABIFICATIONS beat all
    ("mielea", "", "mi-e-lea", "manual", ""),
    ("răspieri", "", "răs-pie-ri", "manual", ""),
    ("coastă", "", "coas-tă", "manual", ""),
    ("coastă", "co-as-tă", "coas-tă", "manual", ""),
];

// Homographs: extra records for the test DB (form, lemma, hyphenation, number[, V])...
const HOMOGRAPH_RECORDS: &[&[&str]] = &[
    &["ia", "ia", "", "S"],      // interjection
    &["ia", "lua", "lu-a", "S"], // verb: el ia
    &["ia", "ie", "", "S"],      // articulated ie (SYLLABIFICATIONS: i-e, ie)
    &["ie", "ie", "", "S"],
    &["miază", "miază", "", "S"],              // SYLLABIFICATIONS: mia-ză, mi-a-ză
    &["trepied", "trepied", "", "S"],          // rules alone: trepẹd...
    &["trepied", "trepied", "tre-pi-ed", "S"], // ...dexonline: hiatus
    &["liază", "liază", "li-a-ză", "S"],       // noun...
    &["liază", "lia", "", "S", "V"],           // ...or verb: -ėză, both spellings count
    &["piei", "piei", "", "S"],                // SYLLABIFICATIONS: piei...
    &["piei", "pia", "pi-a", "S"],             // ...so no other lemma adds a spelling
    &["vier", "vier", "", "S"],                // SYLLABIFICATIONS: vier, vi-er
    &["vierul", "vier", "", "S"],
    &["agreare", "agreare", "", "S"],           // a headword by rules alone...
    &["agreare", "agrea", "a-gre-a", "S", "V"], // ...loses to a backed derived form
];
// ...and (word, all spellings, default first). No alternatives: [].
const HOMOGRAPHS: &[(&str, &[&str])] = &[
    ("ia", &["ė", "ia"]),
    ("ie", &["ie", "ẹ"]),
    ("Ia", &["Ė", "Ia"]),
    ("miază", &["mėză", "miază"]),  // noon (default), myiasis
    ("trepied", &[]),               // a rules-only spelling is no alternative...
    ("liază", &["liază", "liėză"]), // ...except one with -ėză/-ėscă
    ("piei", &[]),                  // SYLLABIFICATIONS: the form's only spelling
    ("vier", &["vẹr", "vier"]),     // the boar (default), păzitor de vie
    ("vierul", &["vẹrul", "vierul"]),
    ("agreare", &[]), // the rules-only agrēre is no alternative
];
// Long infinitives in -iare (flag F) are written -iere: records for the test DB.
const LONG_INFINITIVE_RECORDS: &[&[&str]] = &[
    &["apreciarea", "apreciare", "a-pre-ci-a-re", "S", "F"],
    &["apreciărilor", "apreciare", "a-pre-ci-a-re", "P", "F"],
    &["tăiarea", "tăiare", "", "S", "F"],
    &["fiare", "fiare", "", "S", "F"],           // rules::IARE_KEPT: no long infinitive
    &["variare", "variare", "", "S"],            // no flag F: no long infinitive
    &["conciliare", "conciliare", "", "S", "F"], // loses to any other record...
    &["conciliare", "conciliar", "", "P"],       // ...the adjective
];

// Unknown words with the -ee switch: (word, all spellings, default first).
const UNKNOWN_EE: &[(&str, &[&str])] = &[
    ("zzeele", &["zzeile", "zzeẹle"]), // plural of zzee (default) or of zzeu
    ("Zzeelor", &["Zzeilor", "Zzeẹlor"]),
    ("zzee", &["zzeẹ"]),                  // read as a singular
    ("Andreei", &["Andreii"]),            // genitive of the known andreea
    ("aleei", &["aleii"]),                // no aleea, but alee is known
    ("aceei", &["aceei"]),                // aceea is known, but keeps -eea
    ("plebeei", &["plebeei", "plebeii"]), // plebeea is a form, not a lemma
    ("Zzeei", &["Zzeei", "Zzeii"]),       // no known lemma: unchanged (default) or -eii
    ("muzeele", &["muzeẹle"]),            // plural of a known -eu lemma: one spelling
    ("Muzeelor", &["Muzeẹlor"]),
    ("muzeurile", &["muzeẹle"]), // known -eu lemma
    ("Muzeuri", &["Muzeẹ"]),
    ("zzeurilor", &["zzeurilor"]), // no known lemma: unchanged
];
// Reverse -ee: extra records for the test DB (form, lemma, hyphenation, number, V, D = DEX-MDA-DOOM).
const EE_REVERT_RECORDS: &[&[&str]] = &[
    &["alee", "alee", "", "S", "", "D"],
    &["aleie", "aleie", "", "S"], // no DEX-MDA-DOOM: aleẹ -> alee
    &["cheie", "cheie", "", "S", "", "D"],
    &["chee", "chee", "", "S"],                      // no DEX-MDA-DOOM: cheẹ -> cheie
    &["apogee", "apogeu", "a-po-geu", "P", "", "D"], // both DEX-MDA-DOOM: -ee beats -euri
    &["apogeuri", "apogeu", "a-po-geu", "P", "", "D"],
    &["caree", "careu", "ca-reu", "P"], // only -euri DEX-MDA-DOOM
    &["careuri", "careu", "ca-reu", "P", "", "D"],
    &["bordee", "bordeu", "bor-deu", "P", "", "D"], // both DEX-MDA-DOOM, neither in -euri: a tie
    &["bordeie", "bordei", "bor-dei", "P", "", "D"],
    &["zeii", "zeu", "zeu", "P", "", "D"], // no candidate, but spelled like zeei's -ee
    &["zeei", "zee", "ze-e", "S"],         // spelling: zeii (DEX-MDA-DOOM) wins
    &["pigmeei", "pigmee", "pig-me-e", "S", "D"], // flags in any order: D alone
    &["pigmeii", "pigmeu", "pig-meu", "P", "D"],
    &["rarelee", "rareleu", "ra-re-leu", "P"], // neither DEX-MDA-DOOM: -ee and -euri tie
    &["rareleuri", "rareleu", "ra-re-leu", "P"],
];
// ...and (concise word, all forms it reverts to with the -ee switch, default first). One form: [].
const REVERT_HOMOGRAPHS: &[(&str, &[&str])] = &[
    ("bordeẹ", &["bordeie", "bordee"]), // default: the character map's
    ("Bordeẹ", &["Bordeie", "Bordee"]),
    ("aleẹ", &[]),                          // decided: alee
    ("pigmeii", &["pigmeii", "pigmeei"]),   // both DEX-MDA-DOOM, pigmeii unchanged: a tie
    ("rareleẹ", &["rarelee", "rareleuri"]), // -ee over -euri only among DEX-MDA-DOOM forms
];
// Lemmas for the test DB with no forms of their own: -eu (no -eele/-eelor), -iesc.
const LEMMAS: &[&str] = &["mu-zeu", "ha-ra-ba-gi-esc"];

// Words the prefix rule may find (the pipeline passes every dexonline form).
const KNOWN: &[&str] = &[
    "abilitat", "analiză", "am", "ace", "are", "acurat", "curat", "aviz", "activ", "astă",
];

// (form, concise, lemma, plural, expected) for the rule function itself
const EE_RULE: &[(&str, &str, &str, bool, Option<&str>)] = &[
    ("idee", "idee", "idee", false, Some("ideẹ")),
    ("bombee", "bombee", "bombeu", true, Some("bombeẹ")),
    ("bombeele", "bombeele", "bombeu", true, Some("bombeẹle")),
    ("bombeelor", "bombeelor", "bombeu", true, Some("bombeẹlor")),
    ("bombeurilor", "bombeurilor", "bombeu", true, Some("bombeẹlor")),
    ("pneurilor", "pneurilor", "pneu", true, None),
    ("bombeuri", "bombeuri", "bombeu", true, Some("bombeẹ")),
    ("piureuri", "piureuri", "piure", true, None), // lemma in -e, not -eu
    ("liceul", "liceul", "liceu", false, None),    // -eu lemma: only -ee/-eele/-eelor
    ("logoree", "logoree", "logoree", true, Some("logorei")),
    ("ideea", "ideē", "idee", false, Some("ideė")),
    ("ideei", "ideei", "idee", false, Some("ideii")),
    ("orhideele", "orhideele", "orhidee", true, Some("orhideile")),
    ("orhideelor", "orhideelor", "orhidee", true, Some("orhideilor")),
    ("xee", "xee", "xa", true, None),        // plural of another lemma
    ("aceea", "aceē", "acela", false, None), // -eea of another lemma
    ("aceea", "aceē", "aceea", false, None), // the pronoun as a headword
    ("niceea", "niceē", "niceea", false, Some("niceė")),
    ("spontaneea", "spontaneē", "spontaneu", false, Some("spontaneė")),
    ("galileei", "galileei", "galileea", false, Some("galileii")),
    ("spontaneei", "spontaneei", "spontaneu", false, Some("spontaneii")),
    ("zzeei", "zzeei", "zzea", false, None),
    ("femeie", "femeẹ", "femeie", false, None),
];

// (input text, ee switch, expected output text, expected unknown words, ai switch)
const TEXTS: &[(&str, bool, &str, &[&str], bool)] = &[
    ("Soare", false, "Sōre", &[], true),
    ("SOARE", false, "SŌRE", &[], true),
    ("Iepure, fier și ceai.", false, "Ẹpure, fẹr și cēi.", &[], true),
    ("Real teatru", false, "Real teatru", &[], true),
    // unknown forms of a known -iesc lemma
    (
        "harabagiasca harabagiască harabagiești",
        false,
        "harabagiėsca harabagiėscă harabagiești",
        &["harabagiasca", "harabagiască", "harabagiești"],
        true,
    ),
    ("zzgiasca", false, "zzgiasca", &["zzgiasca"], true), // no known lemma: rules alone
    ("şoarece", false, "șōrece", &[], true),              // cedilla input
    ("casa mare", false, "casa mare", &[], true),         // no candidate: never looked up
    ("zzoa soare", false, "zzō sōre", &["zzoa"], true),   // unknown: rules alone
    ("Zzoa zzia", false, "Zzō zzia", &["Zzoa", "zzia"], true),
    ("", false, "", &[], true),
    ("O idee, o alee, bombee.", false, "O idee, o alee, bombee.", &[], true),
    ("O idee, o alee, bombee.", true, "O ideẹ, o aleẹ, bombeẹ.", &[], true),
    ("Logoree, IDEE, femeie", true, "Logoreẹ, IDEẸ, femeẹ", &[], true),
    ("Ideea orhideei", false, "Ideē orhideei", &[], true),
    ("Ideea orhideei, ideii", true, "Ideė orhideii, ideii", &[], true),
    ("Orhideele orhideelor", false, "Orhideele orhideelor", &[], true),
    ("Orhideele orhideelor", true, "Orhideile orhideilor", &[], true),
    ("Liceele liceelor", false, "Liceele liceelor", &[], true),
    ("Liceele liceelor", true, "Liceẹle liceẹlor", &[], true),
    (
        "Bombeurile liceurilor, pneurile",
        false,
        "Bombeurile liceurilor, pneurile",
        &[],
        true,
    ),
    (
        "Bombeurile liceurilor, pneurile",
        true,
        "Bombeẹle liceẹlor, pneurile",
        &[],
        true,
    ),
    ("Bombeuri, pneuri", false, "Bombeuri, pneuri", &[], true),
    ("Bombeuri, pneuri", true, "Bombeẹ, pneuri", &[], true),
    (
        "Niceea, Andreea, spontaneea, aceea",
        false,
        "Niceē, Andreē, spontaneē, aceē",
        &[],
        true,
    ),
    (
        "Niceea, Andreea, spontaneea, aceea",
        true,
        "Niceė, Andreė, spontaneė, aceē",
        &[],
        true,
    ),
    ("Galileei, spontaneei", true, "Galileii, spontaneii", &[], true),
    ("zzee soare", true, "zzeẹ sōre", &["zzee"], true),
    ("zzee", false, "zzee", &[], true), // -ee not a candidate with the switch off
    ("zzeele zzeelor", true, "zzeile zzeilor", &["zzeele", "zzeelor"], true),
    ("Câinele mâncă, CÂND?", false, "Cîinele mîncă, CÎND?", &[], true), // â -> î, on by default
    ("Pâinea și soarele", false, "Pîinē și sōrele", &[], true),
    ("zzâia", false, "zzîė", &["zzâia"], true), // unknown words too
    ("zziăr", false, "zziăr", &[], true),       // iăr alone: nothing to change
    (
        "Apreciarea apreciărilor, tăiarea",
        false,
        "Aprecierē aprecierilor, tăẹrē",
        &[],
        true,
    ),
    (
        "fiare, variare, piei, conciliare",
        false,
        "fėre, variare, pẹi, conciliare",
        &[],
        true,
    ),
    ("agreare", false, "agreare", &[], true), // backed beats rules alone
    // root român/rumân: â kept
    (
        "Român, ROMÂNESC, aromână, dacoromân, rumân, rumânesc",
        false,
        "Român, ROMÂNESC, aromână, dacoromân, rumân, rumânesc",
        &[],
        true,
    ),
    ("Câinele, pâinea", false, "Câinele, pâinē", &[], false), // switch off
    ("Câinele, pâinea", false, "Cîinele, pîinē", &[], true),
];

// (concise text, expected standard text, -ee switch)
const REVERTS: &[(&str, &str, bool)] = &[
    ("Ẹpure, fẹr și cēi.", "Iepure, fier și ceai.", false),
    ("SŌRE", "SOARE", false),
    ("Ėrbă", "Iarbă", false),
    ("studiėză, SFIĖSCĂ, fiėsca", "studiază, SFIASCĂ, fiasca", false),
    ("Cîinele mîncă pîinē", "Cîinele mîncă pîinea", false), // î is left as is
    ("aleẹ, orhideile", "aleie, orhideile", false),         // -ee switch off: character map
    (
        "Ideẹ, orhideile, ORHIDEĖ, orhideii",
        "Idee, orhideele, ORHIDEEA, orhideei",
        true,
    ),
    ("aleẹ, cheẹ", "alee, cheie", true),        // the DEX-MDA-DOOM form
    ("apogeẹ, careẹ", "apogee, careuri", true), // -ee over -euri, if both DEX-MDA-DOOM
    ("bombeẹle, liceẹlor", "bombeele, liceelor", true),
    ("femeẹ, zzeẹ, studiėză", "femeie, zzeie, studiază", true), // not in ee_revert, unknown
    ("Zeii", "Zeii", true),                                     // a form with no digraph beats zeei
];

static DB: LazyLock<Tables> = LazyLock::new(|| {
    let mut records: Vec<Record> = WORDS.iter().map(|(s, _)| Record::new(&[s])).collect();
    records.extend(
        DERIVED
            .iter()
            .map(|(f, lemma, _)| Record::new(&[f, &lemma.replace('-', ""), lemma, "S"])),
    );
    records.extend(
        EE_WORDS
            .iter()
            .map(|(f, lemma, n, _)| Record::new(&[f, &lemma.replace('-', ""), lemma, n])),
    );
    for table in [HOMOGRAPH_RECORDS, EE_REVERT_RECORDS, LONG_INFINITIVE_RECORDS] {
        records.extend(table.iter().map(|r| Record::new(r)));
    }
    records.extend(LEMMAS.iter().map(|s| Record::new(&[s])));
    build(records, &HashSet::new()).0
});

/// Collects the failing cases, so that one run reports all of them.
#[derive(Default)]
struct Failures(Vec<String>);

impl Failures {
    fn check<T: PartialEq + Debug>(&mut self, case: impl Debug, got: T, expected: T) {
        if got != expected {
            self.0.push(format!("{case:?}: got {got:?}, expected {expected:?}"));
        }
    }

    fn assert(self) {
        assert!(self.0.is_empty(), "{} failing:\n{}", self.0.len(), self.0.join("\n"));
    }
}

fn strings(xs: &[&str]) -> Vec<String> {
    xs.iter().map(|x| x.to_string()).collect()
}

#[test]
fn rules() {
    let mut f = Failures::default();
    for (syl, expected) in WORDS {
        f.check(syl, concise_from_syllables(syl), expected.to_string());
    }
    f.assert();
}

#[test]
fn derived() {
    let mut f = Failures::default();
    for (form, lemma, expected) in DERIVED {
        f.check(form, concise_from_reference(form, lemma), expected.to_string());
    }
    f.assert();
}

#[test]
fn syllabify_table() {
    let known: HashSet<String> = KNOWN.iter().map(|x| x.to_string()).collect();
    let mut f = Failures::default();
    for (word, hyphenation, expected, source, pos) in SYLLABIFY {
        let (got, got_source) = syllabify(word, hyphenation, &known, *pos == "V");
        f.check(
            (word, hyphenation),
            (got, got_source.name()),
            (expected.to_string(), *source),
        );
    }
    f.assert();
}

#[test]
fn ee_rule() {
    let mut f = Failures::default();
    for (form, concise, lemma, plural, expected) in EE_RULE {
        f.check(
            (form, concise),
            concise_ee(form, concise, lemma, *plural),
            expected.map(String::from),
        );
    }
    f.assert();
}

#[test]
fn ee_db() {
    let mut expected: HashMap<&str, &str> = HashMap::new();
    for (form, _, number, exp) in EE_WORDS {
        // singular reading wins
        if *number == "S" || !expected.contains_key(form) {
            expected.insert(form, exp);
        }
    }
    let mut f = Failures::default();
    for (form, exp) in expected {
        let row = &DB.forms[form];
        f.check(form, row.concise_ee.as_ref().unwrap_or(&row.concise).as_str(), exp);
    }
    f.assert();
}

#[test]
fn convert_table() {
    let mut f = Failures::default();
    for (text, ee, expected, unknown, ai) in TEXTS {
        let r = convert(text, &*DB, Options { ee: *ee, ai: *ai });
        f.check(
            (text, ee, ai),
            (r.text, r.unknown),
            (expected.to_string(), strings(unknown)),
        );
    }
    f.assert();
}

#[test]
fn revert_table() {
    let mut f = Failures::default();
    for (text, expected, ee) in REVERTS {
        f.check((text, ee), revert(text, &*DB, *ee).text, expected.to_string());
    }
    f.assert();
}

#[test]
fn round_trip_whole_db() {
    // base rules only: the -ee rule is not reversible by character map
    assert!(!DB.forms.is_empty());
    // a form of a long infinitive in -iare may revert to its -iere spelling
    let long_inf: HashSet<&str> = LONG_INFINITIVE_RECORDS
        .iter()
        .filter(|r| r[4..].contains(&"F") && long_infinitive(r[1]))
        .map(|r| r[0])
        .collect();
    let mut f = Failures::default();
    for (form, row) in &DB.forms {
        let allowed = if long_inf.contains(form.as_str()) {
            vec![form.clone(), iere_spelling(form)]
        } else {
            vec![form.clone()]
        };
        let got = revert_text(&row.concise);
        if !allowed.contains(&got) {
            f.check(form, got, allowed[0].clone());
        }
    }
    f.assert();
}

#[test]
fn lookup_words() {
    let r = lookup(&["Soare", "casa", "zzoa", "Câine", "român"], &*DB, Options::default());
    let results: HashMap<&str, &str> = [
        ("Soare", "sōre"),
        ("casa", "casa"),
        ("zzoa", "zzō"),
        ("Câine", "cîine"),
        ("român", "român"),
    ]
    .into();
    assert_eq!(
        r.results
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect::<HashMap<_, _>>(),
        results
    );
    assert_eq!(r.unknown, ["zzoa"]);
    assert!(r.alternatives.is_empty());

    let r = lookup(&["pâinea"], &*DB, Options { ee: false, ai: false });
    assert_eq!(r.results["pâinea"], "pâinē");

    let r = lookup(
        &["idee", "zzee", "zzeele", "muzeele"],
        &*DB,
        Options { ee: true, ai: true },
    );
    let results: HashMap<&str, &str> = [
        ("idee", "ideẹ"),
        ("zzee", "zzeẹ"),
        ("zzeele", "zzeile"),
        ("muzeele", "muzeẹle"),
    ]
    .into();
    assert_eq!(
        r.results
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect::<HashMap<_, _>>(),
        results
    );
    assert_eq!(r.unknown, ["zzee", "zzeele", "muzeele"]);
    assert_eq!(r.alternatives.len(), 1);
    assert_eq!(r.alternatives["zzeele"], ["zzeile", "zzeẹle"]);
}

#[test]
fn homographs() {
    let mut f = Failures::default();
    for (word, spellings) in HOMOGRAPHS {
        let s = &convert(word, &*DB, Options::default()).segments[0];
        f.check(word, s.alts.clone(), strings(spellings));
        if let Some(first) = spellings.first() {
            f.check(word, s.t.as_str(), first); // default in the text
        }
    }
    f.assert();
}

#[test]
fn revert_homographs() {
    let mut f = Failures::default();
    for (word, spellings) in REVERT_HOMOGRAPHS {
        let s = &revert(word, &*DB, true).segments[0];
        f.check(word, s.alts.clone(), strings(spellings));
        if let Some(first) = spellings.first() {
            f.check(word, s.t.as_str(), first); // default in the text
        }
    }
    f.assert();
}

#[test]
fn unknown_ee() {
    let mut f = Failures::default();
    for (word, spellings) in UNKNOWN_EE {
        let s = &convert(word, &*DB, Options { ee: true, ai: true }).segments[0];
        f.check(word, s.unknown.as_deref(), Some(*word));
        f.check(word, s.t.as_str(), spellings[0]);
        let alts = if s.alts.is_empty() {
            vec![s.t.clone()]
        } else {
            s.alts.clone()
        };
        f.check(word, alts, strings(spellings));
    }
    f.assert();
}
