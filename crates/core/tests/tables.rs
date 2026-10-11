//! grafia concisă — table-driven test suite.
//!
//! Run: cargo test --workspace
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::sync::LazyLock;

use grafia_concisa::build::{Record, Stats, Tables, build};
use grafia_concisa::rules::{
    concise_ee, concise_from_reference, concise_from_syllables, iere_spelling, long_infinitive, revert_text, syllabify,
};
use grafia_concisa::{DataFile, Dictionary, Options, RULES_HASH, convert, datafile, lookup, revert};

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
// (form, lemma syllabified, expected, "V" for a verb)
const DERIVED: &[(&str, &str, &str, &str)] = &[
    ("soarele", "soa-re", "sōrele", ""), // shared prefix: lemma decides
    ("reală", "re-al", "reală", ""),
    ("familia", "fa-mi-li-e", "familia", ""), // ending replaces a vowel: lemma decides
    ("familiei", "fa-mi-li-e", "familiei", ""),
    ("femeia", "fe-me-ie", "femeė", ""),
    ("ploaia", "ploa-ie", "plōė", ""),
    ("ideea", "i-de-e", "ideē", ""), // appended ending: diphthong
    ("creează", "cre-a", "creēză", "V"),
    ("înșeală", "în-șe-la", "înșēlă", "V"), // stem alternation: diphthong
    ("doarme", "dor-mi", "dōrme", "V"),
    ("ploaie", "plo-ua", "plōẹ", "V"), // o/e before a replaced vowel: diphthong
    ("scheaună", "sche-u-na", "schēună", "V"),
    ("taie", "tă-ia", "taẹ", "V"),
    ("argintie", "ar-gin-tiu", "argintie", ""), // -iu lemma: hiatus
    ("cheflia", "che-fliu", "cheflia", ""),
    ("sfiește", "sfi-i", "sfiește", "V"), // -ii verb: lemma decides
    ("sfiască", "sfi-i", "sfiėscă", "V"), // -ează/-ească after a hiatus i: -ėză, -ėscă
    ("priască", "pri-i", "priėscă", "V"),
    ("studiază", "stu-di-a", "studiėză", "V"),
    ("pliază", "pli-a", "pliėză", "V"),
    ("remaniază", "re-ma-ni-a", "remaniėză", "V"),
    ("fiască", "fi-esc", "fiėscă", ""),
    ("fiasca", "fi-esc", "fiėsca", ""),
    ("știa", "ști", "știa", "V"), // ending after the final i of a verb in consonant + i: hiatus
    ("știam", "ști", "știam", "V"),
    ("știai", "ști", "știai", "V"),
    ("știați", "ști", "știați", "V"),
    ("știau", "ști", "știau", "V"),
    ("neștie", "ne-ști", "neștie", "V"),
    ("preștia", "pre-ști", "preștia", "V"),
    ("locuia", "lo-cu-i", "locuė", "V"), // ...not after a vowel
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
    ("oareceea", "oa-re-ce-ea", "S", "ōreceē"),      // ...and its kin
    ("spontaneea", "spon-ta-neu", "S", "spontaneė"), // -eu adjective
    ("galileei", "ga-li-le-ea", "S", "galileii"),    // -eei: same lemmas as -eea
    ("spontaneei", "spon-ta-neu", "S", "spontaneii"),
    ("plebeea", "ple-beu", "S", "plebeė"),
    ("orhideei", "or-hi-de-e", "S", "orhideii"),
    ("orhideele", "or-hi-de-e", "P", "orhideile"),   // like femeile
    ("orhideelor", "or-hi-de-e", "P", "orhideilor"), // like femeilor
    ("agavacee", "a-ga-va-ce-e", "P", "agavacei"),   // a plural-only -ee lemma
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
    ("geognostă", "-o-gnost", "geognostă", "rules", ""), // a word end that is not the word's end: ignored
    ("soarele", "soa-re", "soarele", "rules", ""),    // another word's complete hyphenation: ignored
    ("acetaldehidă", "a-", "a-cetaldehidă", "partial", ""), // a word start found again later
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
    ("mlădii", "", "mlădi-i", "rules", "V"),
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
    ("debreiabil", "", "debreiabil", "rules", ""), // -iabil after a vowel: like its verb (de-bre-ia)
    ("copiere", "", "copi-ere", "rules", ""),      // final -iere: hiatus...
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
    ("rachiereasă", "", "rachi-ereasă", "rules", ""), // rachier + -easă
    ("pompiereasă", "", "pompi-ereasă", "rules", ""),
    ("pompiereasa", "", "pompi-ereasa", "rules", ""),
    ("rachieresele", "", "rachi-eresele", "rules", ""), // and the forms of -iereasă
    ("rachierese", "", "rachi-erese", "rules", ""),
    ("rachieresei", "", "rachi-eresei", "rules", ""),
    ("rachiereselor", "", "rachi-ereselor", "rules", ""),
    ("boiereasă", "", "boiereasă", "rules", ""),  // after a vowel: diphthong
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
    ("coardă", "", "coar-dă", "manual", ""),
    ("coacere", "", "coa-ce-re", "manual", ""),
    ("coacăț", "", "coa-căț", "manual", ""),
    ("coacăță", "", "coa-că-ță", "manual", ""),
    ("înfieri", "", "în-fi-e-ri", "manual", ""),
    ("vieri", "", "vieri", "manual", ""),
    ("viere", "", "vi-e-re", "manual", ""),
    // IA_DIPHTHONG: each stem...
    ("chiabur", "", "chiabur", "rules", ""),
    ("chiag", "", "chiag", "rules", ""),
    ("chiaun", "", "chiaun", "rules", ""),
    ("unchiaș", "", "unchiaș", "rules", ""),
    ("viață", "", "viață", "rules", ""),
    ("miazănoapte", "", "miazănoapte", "rules", ""),
    ("nămiază", "", "nămiază", "rules", ""),
    ("miau", "", "miau", "rules", ""),
    ("megiaș", "", "megiaș", "rules", ""),
    ("caragiale", "", "caragiale", "rules", ""),
    ("caragialian", "", "caragiali-an", "rules", ""), // only the stem's ia
    ("barcianu", "", "barcianu", "rules", ""),
    ("vianu", "", "vianu", "rules", ""),
    ("ghiare", "", "ghiare", "rules", ""), // like ghiară
    ("ghiarele", "", "ghiarele", "rules", ""),
    ("chiarele", "", "chiarele", "rules", ""), // chiar before e too
    // ...and each exception
    ("unghiar", "", "unghi-ar", "rules", ""),
    ("gorghias", "", "gorghi-as", "rules", ""),
    ("privileghiat", "", "privileghi-at", "rules", ""),
    ("aghian", "", "aghi-an", "rules", ""),
    ("oghial", "", "oghi-al", "rules", ""),
    ("hipiatric", "", "hipi-atric", "rules", ""),
    ("deviație", "", "devi-ați-e", "rules", ""),
    ("semiautomat", "", "semi-automat", "rules", ""),
    ("miaulis", "", "mi-aulis", "rules", ""),
    ("diluvianul", "", "diluvi-anul", "rules", ""),
    // EA_HIATUS
    ("interliceal", "", "interlice-al", "rules", ""), // -eal after c: hiatus
    ("mangeal", "", "mangeal", "rules", ""),          // -eal after g: diphthong
    ("cohlear", "", "cohle-ar", "rules", ""),
    ("balnear", "", "balne-ar", "rules", ""),
    ("agreat", "", "agre-at", "rules", ""),
    ("laureat", "", "laure-at", "rules", ""),
    ("bacaloreat", "", "bacalore-at", "rules", ""),
    ("nucleat", "", "nucle-at", "rules", ""),
    ("cuneat", "", "cune-at", "rules", ""),
    ("bracteat", "", "bracte-at", "rules", ""),
    ("ideat", "", "ide-at", "rules", ""),
    ("supleat", "", "suple-at", "rules", ""),
    ("anucleată", "", "anucle-ată", "rules", ""), // the feminine too
    ("laureată", "", "laure-ată", "rules", ""),
    ("laureatul", "", "laure-atul", "rules", ""), // whatever follows -eat
    ("delineator", "", "deline-ator", "rules", ""),
    ("cleat", "", "cleat", "rules", ""), // native: diphthong
    ("procleat", "", "procleat", "rules", ""),
    ("procleată", "", "procleată", "rules", ""),
    ("pleată", "", "pleată", "rules", ""),
    ("soreată", "", "soreată", "rules", ""),
    ("idealist", "", "ide-alist", "rules", ""),
    ("boreale", "", "bore-ale", "rules", ""),
    ("hiperboreală", "", "hiperbore-ală", "rules", ""),
    ("răboreală", "", "răboreală", "rules", ""), // native -eală
    ("stoboreală", "", "stoboreală", "rules", ""),
    ("reacție", "", "re-acți-e", "rules", ""),
    ("steatoză", "", "ste-atoză", "rules", ""),
    ("meandru", "", "me-andru", "rules", ""),
    ("beatific", "", "be-atific", "rules", ""),
    // IE_HIATUS
    ("pieton", "", "pi-eton", "rules", ""),
    ("cvietism", "", "cvi-etism", "rules", ""),
    ("societar", "", "soci-etar", "rules", ""),
    ("vienez", "", "vi-enez", "rules", ""),
    ("mielom", "", "mi-elom", "rules", ""),
    ("demielinizare", "", "demi-elinizare", "rules", ""),
    ("amielie", "", "ami-eli-e", "rules", ""),
    ("așchietoare", "", "așchi-etoare", "rules", ""),
    ("sbiera", "", "sbiera", "rules", ""), // bier after s: diphthong, like zbiera
    // medial ie after each labial or palatal consonant: diphthong
    ("vierme", "", "vierme", "rules", ""),
    ("măcieș", "", "măcieș", "rules", ""),
    ("megieșa", "", "megieșa", "rules", ""),
    ("chiem", "", "chiem", "rules", ""),
    ("ghieț", "", "ghieț", "rules", ""),
    ("brîie", "", "brîie", "rules", ""),      // after î: diphthong
    ("fiere", "", "fiere", "rules", ""),      // IERE_DIPHTHONG
    ("copieri", "", "copi-eri", "rules", ""), // IERE_ENDINGS
    ("copierii", "", "copi-erii", "rules", ""),
    ("copierile", "", "copi-erile", "rules", ""),
    // -ient + a/ă/e/i, and -ienț
    ("consciență", "", "consci-ență", "rules", ""),
    ("ambienta", "", "ambi-enta", "rules", ""),
    ("eficientă", "", "efici-entă", "rules", ""),
    ("ambiente", "", "ambi-ente", "rules", ""),
    ("eficientiza", "", "efici-entiza", "rules", ""),
    // OA_HIATUS
    ("coabita", "", "co-abita", "rules", ""),
    ("coalizat", "", "co-alizat", "rules", ""),
    ("coaliție", "", "co-aliți-e", "rules", ""),
    // PREFIXES, each before a known word
    ("reabonat", "", "re-abonat", "rules", ""),
    ("neabonat", "", "ne-abonat", "rules", ""),
    ("teleactualitate", "", "tele-actualitate", "rules", ""),
    ("semiegiptean", "", "semi-egiptean", "rules", ""),
    ("antiaerian", "", "anti-aeri-an", "rules", ""),
    ("antioxidant", "", "anti-oxidant", "rules", ""), // (anti-, multi- + ia/ie: a hiatus anyway)
    ("multianual", "", "multi-anual", "rules", ""),
    ("multiorganic", "", "multi-organic", "rules", ""),
    ("microagent", "", "micro-agent", "rules", ""),
    ("macroagregat", "", "macro-agregat", "rules", ""),
    ("fotoactualitate", "", "foto-actualitate", "rules", ""),
    ("pseudoafix", "", "pseudo-afix", "rules", ""),
    ("bioacumulat", "", "bio-acumulat", "rules", ""),
    ("neoaristotelic", "", "neo-aristotelic", "rules", ""),
    ("radioaliniament", "", "radio-alini-ament", "rules", ""),
    ("aeroamfibie", "", "aero-amfibi-e", "rules", ""),
    ("agroalimentar", "", "agro-alimentar", "rules", ""),
    ("electroacustic", "", "electro-acustic", "rules", ""),
    ("hidroagregat", "", "hidro-agregat", "rules", ""),
    ("termoacustic", "", "termo-acustic", "rules", ""),
    ("geoacustic", "", "geo-acustic", "rules", ""),
    ("protoistorie", "", "proto-istori-e", "rules", ""),
    ("socioafectiv", "", "socio-afectiv", "rules", ""),
    ("psihoanalitic", "", "psiho-analitic", "rules", ""),
    ("stereoautograf", "", "stereo-autograf", "rules", ""),
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
    &["menție", "menție", "", "S"],             // a headword (men-ți-e)...
    &["menție", "menține", "", "S", "V"],       // ...beats a derived form (mențẹ), both by rules
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
    ("agreare", &[]),                         // the rules-only agrēre is no alternative
    ("zzoa", &[]),                            // an unknown word with one spelling has no switch
    ("alineează", &["aliniėză", "alineēză"]), // obsolete: the current form, then its own (a-li-ne-a)
    ("alineez", &["aliniez", "alineez"]),
    ("alineară", &["aliniară", "alineară"]),
    ("alineare", &["aliniere", "alineare"]), // a-li-ne-a-re: a hiatus
    ("rachiareasă", &["rachierēsă", "rachiarēsă"]),
    ("rachiareasa", &["rachierēsa", "rachiarēsa"]), // unknown
    ("apreciarea", &["aprecierē", "apreciarē"]),    // a long infinitive in -iare: obsolete
    ("tăiarea", &["tăẹrē", "tăėrē"]),
    ("înjunghiare", &["înjunghiere", "înjunghiare"]), // a hiatus, like înjunghiere (not înjunghėre)
];
// (text, number of segments): plain text is merged into one segment, others stay apart
const SEGMENTS: &[(&str, usize)] = &[
    ("Soare și ia", 2),  // "Sōre și ", ia
    ("ia mare", 2),      // ia, " mare"
    ("zzoa mare", 2),    // zzō (unknown), " mare"
    ("Un fier", 1),      // plain text, then a word with one spelling: one segment
    ("alineez mare", 2), // an obsolete word stays apart
];
// Long infinitives in -iare (flag F) are written -iere: records for the test DB.
const LONG_INFINITIVE_RECORDS: &[&[&str]] = &[
    &["apreciarea", "apreciare", "a-pre-ci-a-re", "S", "F"],
    &["apreciărilor", "apreciare", "a-pre-ci-a-re", "P", "F"],
    &["tăiarea", "tăiare", "", "S", "F"],
    &["înjunghiare", "înjunghiare", "", "S", "F"], // no hyphenation: its own spelling keeps the -iere cuts
    &["fiare", "fiare", "", "S", "F"],             // rules::IARE_KEPT: no long infinitive
    &["mobiliare", "mobiliare", "", "S", "F"],
    &["variare", "variare", "", "S"],            // no flag F: no long infinitive
    &["conciliare", "conciliare", "", "S", "F"], // loses to any other record...
    &["conciliare", "conciliar", "", "P"],       // ...the adjective
];

// The -iere spelling of a long infinitive in -iare, a form or a hyphenation: (input, expected)
const IERE_SPELLINGS: &[(&str, &str)] = &[
    ("apreciare", "apreciere"),
    ("apreciărilor", "aprecierilor"),   // -iăr- too
    ("a-pre-ci-a-re", "a-pre-ci-e-re"), // across the hyphens
    ("ziariare", "ziariere"),           // only the last -iar-
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
    ("muzeurilor", &["muzeẹlor"]),
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
    &["zzbeleuri", "zzbeleu", "", "P"], // a tie: alphabetical order, not the records' order
    &["zzbelee", "zzbeleu", "", "P"],
    &["apogeele", "apogeu", "a-po-geu", "P", "", "D"], // -eele beats -eurile, -eelor beats -eurilor
    &["apogeurile", "apogeu", "a-po-geu", "P", "", "D"],
    &["apogeelor", "apogeu", "a-po-geu", "P", "", "D"],
    &["apogeurilor", "apogeu", "a-po-geu", "P", "", "D"],
];
// ...and (concise word, all forms it reverts to with the -ee switch, default first). One form: [].
const REVERT_HOMOGRAPHS: &[(&str, &[&str])] = &[
    ("bordeẹ", &["bordeie", "bordee"]), // default: the character map's
    ("Bordeẹ", &["Bordeie", "Bordee"]),
    ("aleẹ", &[]),   // decided: alee
    ("apogeẹ", &[]), // decided: apogee, not apogeuri
    ("apogeẹle", &[]),
    ("apogeẹlor", &[]),
    ("zzbeleẹ", &["zzbelee", "zzbeleuri"]), // no zzbeleie: the first alphabetically
    ("pigmeii", &["pigmeii", "pigmeei"]),   // both DEX-MDA-DOOM, pigmeii unchanged: a tie
    ("rareleẹ", &["rarelee", "rareleuri"]), // -ee over -euri only among DEX-MDA-DOOM forms
];
// The build's stats: records (one or more per stat)...
const STATS_RECORDS: &[&[&str]] = &[
    &["ia", "ia", "", "S"], // a homograph; manual
    &["ia", "lua", "lu-a", "S"],
    &["bordee", "bordeu", "bor-deu", "P", "", "D"], // an ee_revert tie
    &["bordeie", "bordei", "bor-dei", "P", "", "D"],
    &["xee", "xa", "", "P"],                                 // -ee, but no -ee rule
    &["zzoare", "zz1", "", "S"],                             // invalid
    &["zeii", "zeu", "zeu", "P"],                            // no candidate
    &["fiasca", "fiesc", "fi-esc", "S"],                     // an -iesc lemma
    &["apreciarea", "apreciare", "a-pre-ci-a-re", "S", "F"], // a long infinitive
    &["ideea", "idee", "i-de-e", "S"],                       // an -ee spelling
    &["aceea", "aceea", "a-ce-ea", "S"],                     // an -ea lemma
    &["familia", "familie", "-li-e", "S"],                   // partial
    &["menție", "menține", "", "S", "V"],                    // derived, then replaced by a headword
    &["menție", "menție", "", "S"],
    &["alee", "alee", "", "S", "", "D"], // an ee_revert entry with no tie
    &["aleie", "aleie", "", "S"],
];
// ...and the stats they give
const STATS: &[(&str, i64)] = &[
    ("derived", 7),
    ("ea_lemmas", 1),
    ("ee", 3),
    ("ee_no_rule", 2),
    ("ee_revert", 3),
    ("ee_revert_ties", 1),
    ("eu_lemmas", 2),
    ("homographs", 1),
    ("iesc_lemmas", 1),
    ("invalid", 1),
    ("lemma_dexonline", 6),
    ("lemma_manual", 1),
    ("lemma_partial", 1),
    ("lemma_rules", 5),
    ("long_infinitive", 1),
    ("no_candidate", 1),
    ("obsolete", 1),
    ("read", 16),
    ("stored", 12),
];
// Obsolete forms (rules::OBSOLETE): records for the test DB
const OBSOLETE_RECORDS: &[&[&str]] = &[
    &["alineează", "alinea", "", "S", "V", "I51.0"], // paired by inflection with...
    &["aliniază", "alinia", "-ni-a", "S", "V", "I51.0"], // ...the current lemma's form
    &["alineez", "alinea", "", "S", "V", "I54.0"],   // no digraph, but an obsolete stem
    &["aliniez", "alinia", "-ni-a", "S", "V", "I54.0"],
    &["alineat", "alinea", "", "S", "V", "I52.0"], // also the noun alineat: not obsolete
    &["aliniat", "alinia", "-ni-a", "S", "V", "I52.0"],
    &["alineat", "alineat", "", "S"],
    &["rachiereasă", "rachiereasă", "", "S", "", "I85.0"], // the current lemma first
    &["rachiareasă", "rachiareasă", "", "S", "", "I85.0"],
    &["alinea", "alinea", "", "S", "V", "I49.0"], // also the imperfect of the current alini...
    &["alinia", "alinia", "-ni-a", "S", "V", "I49.0"],
    &["alinea", "alini", "", "S", "V", "I68.0"],
    &["alineară", "alinea", "", "S", "V", "I77.0"], // alinea + -ră: its own cut ne-a
    &["aliniară", "alinia", "-ni-a", "S", "V", "I77.0"],
    &["aliniere", "aliniere", "-ni-e-re", "S", "", "I30.0"],
    &["alineare", "alineare", "", "S", "", "I30.0"],
];
// Obsolete words: (word, its current form, in its case; None if not obsolete)
const OBSOLETE_WORDS: &[(&str, Option<&str>)] = &[
    ("Alineează", Some("Aliniază")),
    ("alineez", Some("aliniez")),
    ("alineat", None), // a current word too
    ("alinea", None),  // ...so not obsolete
    ("rachiareasă", Some("rachiereasă")),
    ("rachiareasa", Some("rachiereasa")), // unknown: by its stem
    ("Apreciarea", Some("Aprecierea")),   // a long infinitive in -iare
    ("conciliare", None),                 // also the adjective conciliar
];
// Records the build skips: a form or a lemma that is not one word
const INVALID_RECORDS: &[&[&str]] = &[&["zzoare", "zz1", "", "S"], &["zzoa1re", "zzoare", "", "S"]];
// Lemmas for the test DB with no forms of their own: -eu (no -eele/-eelor), -iesc.
const LEMMAS: &[&str] = &["mu-zeu", "ha-ra-ba-gi-esc"];

// Words the prefix rule may find (the pipeline passes every dexonline form).
const KNOWN: &[&str] = &[
    "abilitat",
    "oxidant",
    "organic",
    "analiză",
    "am",
    "ace",
    "are",
    "acurat",
    "curat",
    "aviz",
    "activ",
    "astă",
    "abonat",
    "actualitate",
    "egiptean",
    "aerian",
    "anual",
    "agent",
    "agregat",
    "afix",
    "acumulat",
    "aristotelic",
    "aliniament",
    "amfibie",
    "alimentar",
    "acustic",
    "istorie",
    "afectiv",
    "analitic",
    "autograf",
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
    ("Știam", false, "Știam", &[], true),   // the verb flag reaches the derivation
    ("menție", false, "menție", &[], true), // a headword beats a derived form
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
    ("zzâia", false, "zzîė", &["zzâia"], true),          // unknown words too
    ("zziăr", false, "zziăr", &[], true),                // iăr alone: nothing to change
    ("zziăree", false, "zziăree", &[], true),            // ...nor -ee, with the -ee switch off
    ("zzoaeele", false, "zzōeele", &["zzoaeele"], true), // -eele: the -ee switch is off
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
    ("agreare", false, "agreare", &[], true),        // backed beats rules alone
    ("Rachiareasă", false, "Rachierēsă", &[], true), // obsolete: as rachiereasă
    (
        "Alineează, alineez, alineat",
        false,
        "Aliniėză, aliniez, alineat",
        &[],
        true,
    ), // obsolete, but the noun alineat
    (
        "rachiareasa rachiaresele",
        false,
        "rachierēsa rachieresele",
        &["rachiareasa", "rachiaresele"],
        true,
    ), // unknown forms of an obsolete stem
    ("mobiliare", false, "mobiliare", &[], true),    // rules::IARE_KEPT
    ("zzea", false, "zzē", &["zzea"], true),         // unknown: any digraph
    ("zzeez", true, "zzeez", &[], true),             // -ee only at the end
    // unknown forms of a known -iesc lemma: every ending
    (
        "harabagiescu harabagiescul harabagiescului harabagieștii harabagieștile harabagieștilor",
        false,
        "harabagiescu harabagiescul harabagiescului harabagieștii harabagieștile harabagieștilor",
        &[
            "harabagiescu",
            "harabagiescul",
            "harabagiescului",
            "harabagieștii",
            "harabagieștile",
            "harabagieștilor",
        ],
        true,
    ),
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
    ("Ẹ", "Ie", false),                                         // one capital: an initial capital
    ("ziėzăr", "ziiazăr", false),                               // -iėză reverts to -iază only at the end
    ("apogeẹle, apogeẹlor", "apogeele, apogeelor", true),
    ("agavacei, orhideilor", "agavacee, orhideelor", true),
];

static DB: LazyLock<Tables> = LazyLock::new(|| build(records(), &HashSet::new()).0);

/// The records of the test DB.
fn records() -> Vec<Record> {
    let mut records: Vec<Record> = WORDS.iter().map(|(s, _)| Record::new(&[s])).collect();
    records.extend(
        DERIVED
            .iter()
            .map(|(f, lemma, _, pos)| Record::new(&[f, &lemma.replace('-', ""), lemma, "S", pos])),
    );
    records.extend(
        EE_WORDS
            .iter()
            .map(|(f, lemma, n, _)| Record::new(&[f, &lemma.replace('-', ""), lemma, n])),
    );
    for table in [
        HOMOGRAPH_RECORDS,
        EE_REVERT_RECORDS,
        LONG_INFINITIVE_RECORDS,
        OBSOLETE_RECORDS,
        INVALID_RECORDS,
    ] {
        records.extend(table.iter().map(|r| Record::new(r)));
    }
    records.extend(LEMMAS.iter().map(|s| Record::new(&[s])));
    records
}

const DATE: &str = "2026-10-04";

// The test database through the data file
static FILE: LazyLock<DataFile<Vec<u8>>> =
    LazyLock::new(|| DataFile::new(datafile::write(&DB, DATE).expect("write")).expect("read"));

/// The test database, in memory and through the data file.
fn dicts() -> [(&'static str, &'static dyn Dictionary); 2] {
    [("tables", &*DB), ("file", &*FILE)]
}

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
    for (form, lemma, expected, pos) in DERIVED {
        f.check(
            form,
            concise_from_reference(form, lemma, *pos == "V"),
            expected.to_string(),
        );
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
    for (name, dict) in dicts() {
        for (form, exp) in &expected {
            let row = dict.form(form);
            f.check(
                (name, form),
                row.map(|r| r.concise_ee.unwrap_or(r.concise)),
                Some(exp.to_string()),
            );
        }
    }
    f.assert();
}

#[test]
fn convert_table() {
    let mut f = Failures::default();
    for (name, dict) in dicts() {
        for (text, ee, expected, unknown, ai) in TEXTS {
            let r = convert(text, dict, Options { ee: *ee, ai: *ai });
            f.check(
                (name, text, ee, ai),
                (r.text, r.unknown),
                (expected.to_string(), strings(unknown)),
            );
        }
    }
    f.assert();
}

#[test]
fn revert_table() {
    let mut f = Failures::default();
    for (name, dict) in dicts() {
        for (text, expected, ee) in REVERTS {
            f.check((name, text, ee), revert(text, dict, *ee).text, expected.to_string());
        }
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
        // ...and an obsolete form to its current form
        let allowed = if long_inf.contains(form.as_str()) {
            vec![form.clone(), iere_spelling(form)]
        } else if let Some(current) = &row.current {
            vec![current.to_string()]
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
    let pairs = |r: &grafia_concisa::Lookup| -> HashMap<String, String> {
        r.results.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
    };
    let expected = |xs: &[(&str, &str)]| -> HashMap<String, String> {
        xs.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()
    };
    for (name, dict) in dicts() {
        let r = lookup(&["Soare", "casa", "zzoa", "Câine", "român"], dict, Options::default());
        let results = [
            ("Soare", "sōre"),
            ("casa", "casa"),
            ("zzoa", "zzō"),
            ("Câine", "cîine"),
            ("român", "român"),
        ];
        assert_eq!(pairs(&r), expected(&results), "{name}");
        assert_eq!(r.unknown, ["zzoa"], "{name}");
        assert!(r.alternatives.is_empty(), "{name}");

        let r = lookup(&["pâinea"], dict, Options { ee: false, ai: false });
        assert_eq!(r.results["pâinea"], "pâinē", "{name}");

        let r = lookup(
            &["idee", "zzee", "zzeele", "muzeele"],
            dict,
            Options { ee: true, ai: true },
        );
        let results = [
            ("idee", "ideẹ"),
            ("zzee", "zzeẹ"),
            ("zzeele", "zzeile"),
            ("muzeele", "muzeẹle"),
        ];
        assert_eq!(pairs(&r), expected(&results), "{name}");
        assert_eq!(r.unknown, ["zzee", "zzeele", "muzeele"], "{name}");
        assert_eq!(r.alternatives.len(), 1, "{name}");
        assert_eq!(r.alternatives["zzeele"], ["zzeile", "zzeẹle"], "{name}");
    }
}

#[test]
fn homographs() {
    let mut f = Failures::default();
    for (name, dict) in dicts() {
        for (word, spellings) in HOMOGRAPHS {
            let s = &convert(word, dict, Options::default()).segments[0];
            f.check((name, word), s.alts.clone(), strings(spellings));
            if let Some(first) = spellings.first() {
                f.check((name, word), s.t.as_str(), first); // default in the text
            }
        }
    }
    f.assert();
}

#[test]
fn obsolete_words() {
    let mut f = Failures::default();
    for (name, dict) in dicts() {
        for (word, current) in OBSOLETE_WORDS {
            let s = &convert(word, dict, Options::default()).segments[0];
            let got = s.obsolete.as_ref().map(|[old, new]| (old.as_str(), new.as_str()));
            f.check((name, word), got, current.map(|c| (*word, c)));
            let r = lookup(&[word], dict, Options::default());
            let lowercase = current.map(|c| c.to_lowercase());
            f.check((name, word, "lookup"), r.obsolete.get(*word).cloned(), lowercase);
        }
    }
    f.assert();
}

#[test]
fn iere_spellings() {
    let mut f = Failures::default();
    for (input, expected) in IERE_SPELLINGS {
        f.check(input, iere_spelling(input), expected.to_string());
    }
    f.assert();
}

#[test]
fn segments() {
    let mut f = Failures::default();
    for (name, dict) in dicts() {
        for (text, count) in SEGMENTS {
            f.check(
                (name, text),
                convert(text, dict, Options::default()).segments.len(),
                *count,
            );
        }
    }
    f.assert();
}

#[test]
fn build_stats() {
    let records = STATS_RECORDS.iter().map(|r| Record::new(r));
    let expected: Stats = STATS.iter().map(|(k, v)| (k.to_string(), *v)).collect();
    assert_eq!(build(records, &HashSet::new()).1, expected);
}

#[test]
fn revert_homographs() {
    let mut f = Failures::default();
    for (name, dict) in dicts() {
        for (word, spellings) in REVERT_HOMOGRAPHS {
            let s = &revert(word, dict, true).segments[0];
            f.check((name, word), s.alts.clone(), strings(spellings));
            if let Some(first) = spellings.first() {
                f.check((name, word), s.t.as_str(), first); // default in the text
            }
        }
    }
    f.assert();
}

#[test]
fn unknown_ee() {
    let mut f = Failures::default();
    for (name, dict) in dicts() {
        for (word, spellings) in UNKNOWN_EE {
            let s = &convert(word, dict, Options { ee: true, ai: true }).segments[0];
            f.check((name, word), s.unknown.as_deref(), Some(*word));
            f.check((name, word), s.t.as_str(), spellings[0]);
            let alts = if s.alts.is_empty() {
                vec![s.t.clone()]
            } else {
                s.alts.clone()
            };
            f.check((name, word), alts, strings(spellings));
        }
    }
    f.assert();
}

#[test]
fn data_file() {
    let mut f = Failures::default();
    // every table reads back as written...
    for (form, row) in &DB.forms {
        f.check(form, FILE.form(form), Some(row.clone()));
    }
    for lemma in &DB.eu_lemmas {
        f.check(lemma, FILE.eu_lemma(lemma), true);
    }
    for lemma in &DB.ea_lemmas {
        f.check(lemma, FILE.ea_lemma(lemma), true);
    }
    for (lemma, reference) in &DB.iesc_lemmas {
        f.check(lemma, FILE.iesc_lemma(lemma), Some(reference.clone()));
    }
    for (concise, entry) in &DB.ee_revert {
        f.check(concise, FILE.ee_revert(concise), Some(entry.clone()));
    }
    // ...with nothing else
    // (zeii: a record with no candidate; zzoare, zzoa1re: INVALID_RECORDS)
    for key in ["", "casa", "soar", "soarez", "zzoa", "zeii", "zzoare", "zzoa1re"] {
        f.check(key, FILE.form(key), None);
    }
    f.check("zzeu", FILE.eu_lemma("zzeu"), false);
    f.check("zzea", FILE.ea_lemma("zzea"), false);
    f.check("zzgiesc", FILE.iesc_lemma("zzgiesc"), None);
    f.check("idee", FILE.iesc_lemma("idee"), None); // a lemma, but not in -iesc
    f.check("zzeẹ", FILE.ee_revert("zzeẹ"), None);
    f.check("date", FILE.date(), DATE);
    f.check("rules", FILE.rules(), RULES_HASH);
    f.assert();
}

#[test]
fn bad_data_files() {
    let bytes = datafile::write(&DB, DATE).unwrap();
    let changed = |at: usize| {
        let mut b = bytes.clone();
        b[at] ^= 1;
        b
    };
    let cases = [
        ("empty", Vec::new()),
        ("magic", [b"SQLite format 3\0".as_slice(), &bytes[16..]].concat()),
        ("version", changed(8)),
        ("a section's length", changed(20)),
        ("a byte in the middle", changed(bytes.len() / 2)),
        ("truncated", bytes[..bytes.len() - 1].to_vec()),
        ("extra byte", [&bytes[..], &[0]].concat()),
    ];
    for (case, bytes) in cases {
        assert!(DataFile::new(bytes).is_err(), "{case}");
    }
    let error = DataFile::new(Vec::new()).err().expect("an error");
    assert_eq!(error.to_string(), "not a grafia concisă data file");
}
