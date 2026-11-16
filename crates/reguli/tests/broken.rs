//! The loading checks of the rule files: broken files, and the messages they give.
//!
//! Run: cargo test -p grafia-reguli
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use grafia_reguli::{
    CLASE, LEME, LISTS, Level, Message, PASI, SILABISIRI, TABLES, default_dir, files, list, load, load_texts, meta,
    read_texts, rust, shown, table,
};

/// The steps of a pasi.yaml that consult the four tables, on its lines 1 to 4.
macro_rules! tables {
    () => {
        "- {pas: ea, tipar: [ea], tabel: hiat-ea.yaml, rezultat: hiat}
- {pas: oa, tipar: [oa], tabel: hiat-oa.yaml, rezultat: hiat}
- {pas: ia, tipar: [ia], ante: [C], tabel: diftong-ia.yaml, rezultat: diftong}
- {pas: ie, tipar: [ie], ante: [L, P], tabel: hiat-ie.yaml, rezultat: hiat}
"
    };
}

/// A last step of pasi.yaml, which decides every pair.
macro_rules! last {
    () => {
        "\n- {pas: restul, rezultat: diftong, exemple: [soare]}"
    };
}

// (file, its text, the messages it gives, by line). The other files are those of reguli/.
// Add a case = add a line.
#[rustfmt::skip]
const BROKEN: &[(&str, &str, &[&str])] = &[
    // YAML syntax, with a Romanian hint
    ("a-i.yaml", "radacini_cu_a: [român, rumân\n", &["reguli/a-i.yaml:2:1: eroare: YAML greșit: lipsește o virgulă între două cuvinte, sau „]” la sfârșitul listei."]),
    ("a-i.yaml", "radacini_cu_a: [român, , rumân]", &["reguli/a-i.yaml:1:24: eroare: YAML greșit: lipsește un cuvânt: două virgule la rând, sau o listă neterminată."]),
    ("a-i.yaml", "radacini_cu_a: român: rumân", &["reguli/a-i.yaml:1:21: eroare: YAML greșit: „:” nu poate sta aici; verifică indentarea, sau pune cuvântul între ghilimele."]),
    ("a-i.yaml", "radacini_cu_a:\n\t- român", &["reguli/a-i.yaml:2:2: eroare: YAML greșit: un tab; indentează numai cu spații."]),
    ("a-i.yaml", "radacini_cu_a: ['român]", &["reguli/a-i.yaml:1:17: eroare: YAML greșit: ghilimelele nu sunt închise."]),
    ("invechite.yaml", "leme:\n  - lema: a-li-ne-a\n   actuala: alinia\niare_pastrate: []", &["reguli/invechite.yaml:3:11: eroare: YAML greșit: indentarea nu se potrivește cu a rândurilor de deasupra."]),
    ("a-i.yaml", "radacini_cu_a: [român] rumân", &["reguli/a-i.yaml:1:24: eroare: YAML greșit: după valoare urmează altceva decât o cheie nouă; verifică parantezele și indentarea."]),
    ("a-i.yaml", "radacini_cu_a: @român", &["reguli/a-i.yaml:1:16: eroare: YAML greșit: acest caracter nu poate începe o valoare; pune cuvântul între ghilimele."]),
    ("a-i.yaml", "radacini_cu_a: 'român'x", &["reguli/a-i.yaml:1:23: eroare: YAML greșit: text în plus după ghilimele."]),
    // YAML syntax, with the parser's own text
    ("a-i.yaml", "radacini_cu_a: *r", &["reguli/a-i.yaml:1:16: eroare: YAML greșit: „while parsing node, found unknown anchor”."]),
    // YAML that the rule files don't use
    ("a-i.yaml", "- român", &["reguli/a-i.yaml:1:1: eroare: fișierul e format din perechi „cheie: valoare”."]),
    ("a-i.yaml", "radacini_cu_a: &r [român]", &["reguli/a-i.yaml:1:19: eroare: ancorele YAML (&) nu se folosesc în fișierele de reguli; scrie valoarea întreagă."]),
    ("a-i.yaml", "radacini_cu_a: !t [român]", &["reguli/a-i.yaml:1:19: eroare: etichetele YAML (!) nu se folosesc în fișierele de reguli."]),
    ("silabisiri.yaml", "[neant]: ne-ant", &["reguli/silabisiri.yaml:1:1: eroare: o cheie e un singur cuvânt, nu o listă."]),
    ("a-i.yaml", "radacini_cu_a: [român]\n---\nradacini_cu_a: [rumân]", &["reguli/a-i.yaml:2:1: eroare: „---” desparte documente YAML, iar un fișier de reguli are unul singur; șterge rândul."]),
    ("a-i.yaml", "radacini_cu_a: [român]\n... # gata", &["reguli/a-i.yaml:2:1: eroare: „...” desparte documente YAML, iar un fișier de reguli are unul singur; șterge rândul."]),
    // YAML 1.2: n, no, on, y, true are words, not booleans
    ("silabisiri.yaml", "n: n\nno: [no]\non: on\ny: y\ntrue: true", &[]),
    // keys
    ("a-i.yaml", "radacini_cu_a: [român]\nradacini_cu_a: [rumân]", &["reguli/a-i.yaml:2:1: eroare: „radacini_cu_a” apare de două ori ca cheie (întâi la rândul 1); păstreaz-o o singură dată."]),
    ("silabisiri.yaml", "vier: vier\nvier: vi-er", &["reguli/silabisiri.yaml:2:1: eroare: „vier” apare de două ori ca cheie (întâi la rândul 1); păstreaz-o o singură dată."]),
    ("a-i.yaml", "radacini_cu_a: [român]\nradacini: [rumân]", &["reguli/a-i.yaml:2:1: eroare: „radacini” nu e o cheie cunoscută aici; cheile sunt radacini_cu_a."]),
    ("a-i.yaml", "# gol", &["reguli/a-i.yaml: eroare: lipsește cheia „radacini_cu_a”."]),
    ("ee.yaml", "nume: []\nae_oe: []\neea_neschimbate: []\nfara_masculn: []", &["reguli/ee.yaml: eroare: lipsește cheia „fara_masculin”.", "reguli/ee.yaml:4:1: eroare: „fara_masculn” nu e o cheie cunoscută aici; cheile sunt nume, ae_oe, eea_neschimbate și fara_masculin."]),
    // a list
    ("a-i.yaml", "radacini_cu_a: român", &["reguli/a-i.yaml:1:1: eroare: „radacini_cu_a” e o listă de cuvinte: radacini_cu_a: [cuvânt, cuvânt]."]),
    ("a-i.yaml", "radacini_cu_a:", &["reguli/a-i.yaml:1:1: eroare: „radacini_cu_a” e o listă de cuvinte: radacini_cu_a: [cuvânt, cuvânt]."]),
    ("a-i.yaml", "radacini_cu_a: {român: rumân}", &["reguli/a-i.yaml:1:1: eroare: „radacini_cu_a” e o listă de cuvinte: radacini_cu_a: [cuvânt, cuvânt]."]),
    ("a-i.yaml", "radacini_cu_a: [[român], rumân]", &["reguli/a-i.yaml:1:17: eroare: un element din „radacini_cu_a” nu e un cuvânt."]),
    ("a-i.yaml", "radacini_cu_a: [român, rumân, român]", &["reguli/a-i.yaml:1:31: eroare: „român” apare de două ori în „radacini_cu_a” (întâi la rândul 1); păstrează-l o singură dată."]),
    ("a-i.yaml", "radacini_cu_a:\n  - român\n  - rumân\n  - român", &["reguli/a-i.yaml:4:5: eroare: „român” apare de două ori în „radacini_cu_a” (întâi la rândul 2); păstrează-l o singură dată."]),
    ("a-i.yaml", "radacini_cu_a: []", &[]),
    // the letters of a word
    ("necunoscute.yaml", "iesc: [eşti]", &["reguli/necunoscute.yaml:1:8: eroare: „eşti” are ş cu sedilă; scrie ș, cu virgulă."]),
    ("necunoscute.yaml", "iesc: [aţi]", &["reguli/necunoscute.yaml:1:8: eroare: „aţi” are ţ cu sedilă; scrie ț, cu virgulă."]),
    ("necunoscute.yaml", "iesc: [ascã]", &["reguli/necunoscute.yaml:1:8: eroare: „ascã” are ã; scrie ă."]),
    ("necunoscute.yaml", "iesc: [asca\u{306}]", &["reguli/necunoscute.yaml:1:8: eroare: „asca\u{306}” are o literă descompusă: o literă urmată de un semn diacritic separat; scrie-o ca un singur caracter."]),
    ("necunoscute.yaml", "iesc: [Ești]", &["reguli/necunoscute.yaml:1:8: eroare: „Ești” are majuscula „E”; scrie cuvintele cu litere mici."]),
    ("necunoscute.yaml", "iesc: [ŞTI]", &["reguli/necunoscute.yaml:1:8: eroare: „ŞTI” are majuscula „Ş”; scrie cuvintele cu litere mici."]),
    ("necunoscute.yaml", "iesc: [escé]", &["reguli/necunoscute.yaml:1:8: eroare: „escé” are „é”, care nu e o literă a alfabetului românesc."]),
    ("necunoscute.yaml", "iesc: [esc2]", &["reguli/necunoscute.yaml:1:8: eroare: „esc2” are „2”, care nu e o literă a alfabetului românesc."]),
    ("necunoscute.yaml", "iesc: ['es cu']", &["reguli/necunoscute.yaml:1:8: eroare: „es cu” are un spațiu; un cuvânt nu are spații."]),
    ("necunoscute.yaml", "iesc: [es-cu]", &["reguli/necunoscute.yaml:1:8: eroare: „es-cu” are o cratimă; cratimele despart doar silabele unei silabisiri."]),
    ("necunoscute.yaml", "iesc: [qwxyz, abcdefghijklmnoprstuvz, ăâșțî]", &[]),
    // î inside a word: a warning only, since neînțeles is right; not at its start or end
    ("necunoscute.yaml", "iesc: [eîti]", &["reguli/necunoscute.yaml:1:8: avertisment: „eîti” are î în interiorul cuvântului; în grafia standard acolo se scrie â (câine), afară de compuse ca neînțeles."]),
    ("necunoscute.yaml", "iesc: [îști, escî, î, îî]", &[]),
    ("silabisiri.yaml", "neînțeles: ne-în-țe-les", &["reguli/silabisiri.yaml:1:1: avertisment: „neînțeles” are î în interiorul cuvântului; în grafia standard acolo se scrie â (câine), afară de compuse ca neînțeles."]),
    ("invechite.yaml", "leme:\n  - {lema: ne-în-țe-les, actuala: neînțelegere, tulpina: neînț}\niare_pastrate: []", &["reguli/invechite.yaml:2:12: avertisment: „ne-în-țe-les” are î în interiorul cuvântului; în grafia standard acolo se scrie â (câine), afară de compuse ca neînțeles.", "reguli/invechite.yaml:2:35: avertisment: „neînțelegere” are î în interiorul cuvântului; în grafia standard acolo se scrie â (câine), afară de compuse ca neînțeles."]),
    // silabisiri.yaml
    ("silabisiri.yaml", "# nimic", &["reguli/silabisiri.yaml: eroare: fișierul nu are nicio silabisire."]),
    ("silabisiri.yaml", "", &["reguli/silabisiri.yaml: eroare: fișierul nu are nicio silabisire."]),
    ("silabisiri.yaml", "neant: ne-an", &["reguli/silabisiri.yaml:1:8: eroare: „ne-an” nu are literele lui „neant”."]),
    ("silabisiri.yaml", "neant: ne--ant", &["reguli/silabisiri.yaml:1:8: eroare: „ne--ant” are o silabă goală: o cratimă la început, la sfârșit sau două la rând."]),
    ("silabisiri.yaml", "neant: -neant", &["reguli/silabisiri.yaml:1:8: eroare: „-neant” are o silabă goală: o cratimă la început, la sfârșit sau două la rând."]),
    ("silabisiri.yaml", "neant: neant-", &["reguli/silabisiri.yaml:1:8: eroare: „neant-” are o silabă goală: o cratimă la început, la sfârșit sau două la rând."]),
    ("silabisiri.yaml", "neant: ne-anţ", &["reguli/silabisiri.yaml:1:8: eroare: „ne-anţ” are ţ cu sedilă; scrie ț, cu virgulă."]),
    ("silabisiri.yaml", "ne-ant: ne-ant", &["reguli/silabisiri.yaml:1:1: eroare: „ne-ant” are o cratimă; cratimele despart doar silabele unei silabisiri."]),
    ("silabisiri.yaml", "vier: []", &["reguli/silabisiri.yaml:1:1: eroare: „vier” nu are nicio silabisire."]),
    ("silabisiri.yaml", "vier: {vier: vi-er}", &["reguli/silabisiri.yaml:1:1: eroare: „vier” e urmat de o silabisire sau de o listă de silabisiri: vier: si-la-bi-si-re."]),
    ("silabisiri.yaml", "vier: [[vier]]", &["reguli/silabisiri.yaml:1:8: eroare: o silabisire a lui „vier” nu e un cuvânt despărțit în silabe."]),
    ("silabisiri.yaml", "vier: [vier, vier]", &["reguli/silabisiri.yaml:1:14: eroare: silabisirea „vier” apare de două ori la „vier”."]),
    ("silabisiri.yaml", "vier: [vier, vi-er, vie-r]", &[]),
    // invechite.yaml
    ("invechite.yaml", "leme: a-li-ne-a\niare_pastrate: []", &["reguli/invechite.yaml:1:1: eroare: „leme” e o listă de leme învechite, fiecare cu lema, actuala și tulpina ei."]),
    ("invechite.yaml", "leme: [a-li-ne-a]\niare_pastrate: []", &["reguli/invechite.yaml:1:8: eroare: o lemă din „leme” are cheile lema, actuala, tulpina și, dacă e cazul, fara_forme."]),
    ("invechite.yaml", "leme: [{}]\niare_pastrate: []", &["reguli/invechite.yaml:1:8: eroare: lipsește cheia „lema”.", "reguli/invechite.yaml:1:8: eroare: lipsește cheia „actuala”.", "reguli/invechite.yaml:1:8: eroare: lipsește cheia „tulpina”."]),
    ("invechite.yaml", "leme:\n  - {lema: a-li-ne-a, actual: alinia, tulpina: aline}\niare_pastrate: []", &["reguli/invechite.yaml:2:6: eroare: lipsește cheia „actuala”.", "reguli/invechite.yaml:2:23: eroare: „actual” nu e o cheie cunoscută aici; cheile sunt lema, actuala, tulpina și fara_forme."]),
    ("invechite.yaml", "leme:\n  - {lema: a-li-ne-a, actuala: [alinia], tulpina: aline}\niare_pastrate: []", &["reguli/invechite.yaml:2:23: eroare: „actuala” are un singur cuvânt, nu o listă."]),
    ("invechite.yaml", "leme:\n  - {lema: a-li-ne-a, actuala: alinia, tulpina: aline, fara_forme: da}\niare_pastrate: []", &["reguli/invechite.yaml:2:68: eroare: „fara_forme” e true sau false, nu „da”."]),
    ("invechite.yaml", "leme:\n  - {lema: a-li-ne-a, actuala: alinia, tulpina: aline, fara_forme: false}\n  - {lema: a-li-ni, actuala: alina, tulpina: alin, fara_forme: true}\niare_pastrate: []", &[]),
    ("invechite.yaml", "leme:\n  - {lema: a-li-ne-a, actuala: alinia, tulpina: alni}\niare_pastrate: []", &["reguli/invechite.yaml:2:49: eroare: tulpina „alni” nu e începutul lemei „alinea”."]),
    ("invechite.yaml", "leme:\n  - {lema: a-li-ne-a, actuala: a-li-nia, tulpina: aline}\niare_pastrate: []", &["reguli/invechite.yaml:2:32: eroare: „a-li-nia” are o cratimă; cratimele despart doar silabele unei silabisiri."]),
    ("invechite.yaml", "leme:\n  - {lema: a-li-ne-ş, actuala: alinia, tulpina: aline}\niare_pastrate: []", &["reguli/invechite.yaml:2:12: eroare: „a-li-ne-ş” are ş cu sedilă; scrie ș, cu virgulă."]),
    ("invechite.yaml", "leme:\n  - {lema: a-li-ne-a-, actuala: alinia, tulpina: aline}\niare_pastrate: []", &["reguli/invechite.yaml:2:12: eroare: „a-li-ne-a-” are o silabă goală: o cratimă la început, la sfârșit sau două la rând."]),
    ("invechite.yaml", "leme:\n  - {lema: a-li-ne-a, actuala: alinia, tulpina: aline}\n  - {lema: a-lin-e-a, actuala: alina, tulpina: alin}\niare_pastrate: []", &["reguli/invechite.yaml:3:12: eroare: lema „alinea” apare de două ori în „leme” (întâi la rândul 2); păstreaz-o o singură dată."]),
    ("invechite.yaml", "leme: []\niare_pastrate: [fiare, fiere]", &["reguli/invechite.yaml:2:24: eroare: „fiere” nu se termină în -iare, ca toate cuvintele din „iare_pastrate”."]),
    // ee.yaml
    ("ee.yaml", "nume: [andreea, darclee, maria]\nae_oe: []\neea_neschimbate: []\nfara_masculin: []", &["reguli/ee.yaml:1:26: eroare: „maria” nu se termină în -eea, -ee sau -oe, ca toate cuvintele din „nume”."]),
    ("ee.yaml", "nume: []\nae_oe: [nae, noe, vita]\neea_neschimbate: []\nfara_masculin: []", &["reguli/ee.yaml:2:19: eroare: „vita” nu se termină în -ae sau -oe, ca toate cuvintele din „ae_oe”."]),
    ("ee.yaml", "nume: []\nae_oe: []\neea_neschimbate: [aceea, acee]\nfara_masculin: []", &["reguli/ee.yaml:3:26: eroare: „acee” nu se termină în -eea, ca toate cuvintele din „eea_neschimbate”."]),
    ("ee.yaml", "nume: []\nae_oe: []\neea_neschimbate: []\nfara_masculin: [coree, coreea]", &["reguli/ee.yaml:4:24: eroare: „coreea” nu se termină în -ee, ca toate cuvintele din „fara_masculin”."]),
    // a name in -oe of nume follows the -ae/-oe rule too, and is not written again in ae_oe
    ("ee.yaml", "nume: [zoe]\nae_oe: []\neea_neschimbate: []\nfara_masculin: []", &[]),
    ("ee.yaml", "nume: [zoe]\nae_oe: [nae, zoe]\neea_neschimbate: []\nfara_masculin: []", &["reguli/ee.yaml:2:14: eroare: „zoe” e deja în „nume”, ale cărui cuvinte în -oe sunt și în „ae_oe”; șterge-l de aici."]),
    ("ee.yaml", "nume: [andreea]\nae_oe: [nae, zoe]\neea_neschimbate: []\nfara_masculin: []", &[]),
    // neconvertite.yaml
    ("neconvertite.yaml", "nume_latine: [traian, trian, traion, ian, adrian]", &["reguli/neconvertite.yaml:1:23: eroare: „trian” nu se termină într-o vocală + -ian, ca toate numele din „nume_latine”.", "reguli/neconvertite.yaml:1:30: eroare: „traion” nu se termină într-o vocală + -ian, ca toate numele din „nume_latine”.", "reguli/neconvertite.yaml:1:38: eroare: „ian” nu se termină într-o vocală + -ian, ca toate numele din „nume_latine”.", "reguli/neconvertite.yaml:1:43: eroare: „adrian” nu se termină într-o vocală + -ian, ca toate numele din „nume_latine”."]),
    ("neconvertite.yaml", "nume_latine: [traian, oian, ăian, âian, eian, îian, uian]", &[]),
    // a-i.yaml
    ("a-i.yaml", "radacini_cu_a: [român, roman]", &["reguli/a-i.yaml:1:24: eroare: „roman” nu are â, deci nu e o rădăcină care își păstrează â."]),
    // a table: a list of entries
    ("hiat-oa.yaml", "tipar: [zoar$]", &["reguli/hiat-oa.yaml:1:6: eroare: fișierul e o listă de intrări, fiecare începând cu „- ”."]),
    ("hiat-oa.yaml", "", &["reguli/hiat-oa.yaml: eroare: fișierul nu are nicio intrare."]),
    ("hiat-oa.yaml", "\u{feff}\n  \n# nimic", &["reguli/hiat-oa.yaml: eroare: fișierul nu are nicio intrare."]),
    ("hiat-oa.yaml", "# nimic", &["reguli/hiat-oa.yaml: eroare: fișierul nu are nicio intrare."]),
    ("hiat-oa.yaml", "[]", &["reguli/hiat-oa.yaml: eroare: fișierul nu are nicio intrare."]),
    ("hiat-oa.yaml", "- zoar$", &["reguli/hiat-oa.yaml:1:3: eroare: o intrare are cheile tipar, ante, non_ante, post, non_post, exemple și contraexemple."]),
    ("hiat-oa.yaml", "- {}", &["reguli/hiat-oa.yaml:1:3: eroare: lipsește cheia „tipar”.", "reguli/hiat-oa.yaml:1:3: eroare: lipsește cheia „exemple”."]),
    ("hiat-oa.yaml", "- {ante: [z]}", &["reguli/hiat-oa.yaml:1:4: eroare: lipsește cheia „tipar”.", "reguli/hiat-oa.yaml:1:4: eroare: lipsește cheia „exemple”."]),
    ("hiat-oa.yaml", "- {tipar: [zoar$], exemple: [protozoar], rezultat: hiat}", &["reguli/hiat-oa.yaml:1:42: eroare: „rezultat” nu e o cheie cunoscută aici; cheile sunt tipar, exemple, ante, non_ante, post, non_post și contraexemple."]),
    ("hiat-oa.yaml", "- {tipar: zoar$, exemple: [protozoar]}", &["reguli/hiat-oa.yaml:1:4: eroare: „tipar” e o listă de cuvinte: tipar: [cuvânt, cuvânt]."]),
    ("hiat-oa.yaml", "- {tipar: [], exemple: [protozoar]}", &["reguli/hiat-oa.yaml:1:4: eroare: „tipar” e o listă goală."]),
    ("hiat-oa.yaml", "- {tipar: [zoar$], exemple: [protozoar], ante: []}", &["reguli/hiat-oa.yaml:1:42: eroare: „ante” e o listă goală."]),
    ("hiat-oa.yaml", "- {tipar: [zoar$, zoar$], exemple: [protozoar]}", &["reguli/hiat-oa.yaml:1:19: eroare: „zoar$” apare de două ori în „tipar” (întâi la rândul 1); păstrează-l o singură dată."]),
    ("hiat-oa.yaml", "- {tipar: [zoar$], exemple: [protozoar, cazoar, protozoar]}", &["reguli/hiat-oa.yaml:1:49: eroare: „protozoar” apare de două ori în „exemple” (întâi la rândul 1); păstrează-l o singură dată."]),
    // a pattern holds the table's pair, and ^, $ only at its ends
    ("hiat-oa.yaml", "- {tipar: [zoar$, zear], exemple: [protozoar]}", &["reguli/hiat-oa.yaml:1:19: eroare: tiparul „zear” nu are perechea oa, singura pe care o decide tabelul acesta."]),
    ("diftong-ia.yaml", "- {tipar: [chiar, chai], exemple: [chiar]}", &["reguli/diftong-ia.yaml:1:19: eroare: tiparul „chai” nu are perechea ia, singura pe care o decide tabelul acesta."]),
    ("hiat-oa.yaml", "- {tipar: [zo^ar, z$oar, ^$], exemple: [protozoar]}", &["reguli/hiat-oa.yaml:1:12: eroare: „zo^ar”: într-un tipar, ^ (începutul cuvântului) stă doar la început, iar $ (sfârșitul cuvântului) doar la sfârșit.", "reguli/hiat-oa.yaml:1:19: eroare: „z$oar”: într-un tipar, ^ (începutul cuvântului) stă doar la început, iar $ (sfârșitul cuvântului) doar la sfârșit.", "reguli/hiat-oa.yaml:1:26: eroare: tiparul „^$” nu are perechea oa, singura pe care o decide tabelul acesta."]),
    ("hiat-oa.yaml", "- {tipar: [zoaR, zoaş, 'zo ar'], exemple: [protozoar]}", &["reguli/hiat-oa.yaml:1:12: eroare: „zoaR” are majuscula „R”; scrie cuvintele cu litere mici.", "reguli/hiat-oa.yaml:1:18: eroare: „zoaş” are ş cu sedilă; scrie ș, cu virgulă.", "reguli/hiat-oa.yaml:1:24: eroare: „zo ar” are un spațiu; un cuvânt nu are spații."]),
    ("hiat-oa.yaml", "- {tipar: [zoa\u{306}r], exemple: [protozoar]}", &["reguli/hiat-oa.yaml:1:12: eroare: „zoa\u{306}r” are o literă descompusă: o literă urmată de un semn diacritic separat; scrie-o ca un singur caracter."]),
    // a context: ^ only at the start of an item of ante, $ only at the end of an item of post
    ("hiat-oa.yaml", "- {tipar: [oar], ante: [z$, ^], non_ante: [m^], exemple: [protozoar], contraexemple: [mozoar]}", &["reguli/hiat-oa.yaml:1:25: eroare: „z$”: în ante, ^ (începutul cuvântului) stă doar la început, iar $ deloc, căci ante e înaintea tiparului.", "reguli/hiat-oa.yaml:1:44: eroare: „m^”: în non_ante, ^ (începutul cuvântului) stă doar la început, iar $ deloc, căci non_ante e înaintea tiparului."]),
    ("hiat-oa.yaml", "- {tipar: [zoa], post: [^r, $], non_post: [r$, $$], exemple: [protozoar], contraexemple: [zoare]}", &["reguli/hiat-oa.yaml:1:25: eroare: „^r”: în post, $ (sfârșitul cuvântului) stă doar la sfârșit, iar ^ deloc, căci post e după tipar.", "reguli/hiat-oa.yaml:1:48: eroare: „$$”: în non_post, $ (sfârșitul cuvântului) stă doar la sfârșit, iar ^ deloc, căci non_post e după tipar."]),
    ("hiat-oa.yaml", "- {tipar: [oar], ante: [z, ^z, ^], post: [$, e$, ă], exemple: [protozoar]}", &[]),
    // an excluding context needs counterexamples, and no word is both
    ("hiat-oa.yaml", "- {tipar: [oar$], non_ante: [m], exemple: [protozoar]}", &["reguli/hiat-oa.yaml:1:4: eroare: lipsește cheia „contraexemple”: non_ante și non_post cer cuvinte pe care le exclud."]),
    ("hiat-oa.yaml", "- {tipar: [zoa], non_post: [ră], exemple: [protozoar], contraexemple: []}", &["reguli/hiat-oa.yaml:1:56: eroare: „contraexemple” e o listă goală."]),
    ("hiat-oa.yaml", "- {tipar: [zoar$], exemple: [protozoar, cazoar], contraexemple: [cazoar, mozoară]}", &["reguli/hiat-oa.yaml:1:66: eroare: „cazoar” e și în exemple, și în contraexemple; păstrează-l într-una singură."]),
    // the words of an entry
    ("hiat-oa.yaml", "- {tipar: [zoar$], exemple: [protozoar, Cazoar], contraexemple: [mozoară, neînzoar]}", &["reguli/hiat-oa.yaml:1:41: eroare: „Cazoar” are majuscula „C”; scrie cuvintele cu litere mici.", "reguli/hiat-oa.yaml:1:75: avertisment: „neînzoar” are î în interiorul cuvântului; în grafia standard acolo se scrie â (câine), afară de compuse ca neînțeles."]),
    // entries on several lines, each one's messages at its line
    ("hiat-oa.yaml", "# oa\n- tipar: [zoar$]\n  exemple: [protozoar]\n\n- tipar: [zear]\n  exemple: [zear]\n- tipar: [coa]\n  non_post: [x]\n  exemple: [coagulat]", &["reguli/hiat-oa.yaml:5:11: eroare: tiparul „zear” nu are perechea oa, singura pe care o decide tabelul acesta.", "reguli/hiat-oa.yaml:7:3: eroare: lipsește cheia „contraexemple”: non_ante și non_post cer cuvinte pe care le exclud."]),
    // an empty value or element, at its key: it starts where the next one does
    ("necunoscute.yaml", "iesc: ['']", &["reguli/necunoscute.yaml:1:1: eroare: „iesc” are un element gol."]),
    ("a-i.yaml", "radacini_cu_a:\n  - român\n  -\n  - rumân", &["reguli/a-i.yaml:1:1: eroare: „radacini_cu_a” are un element gol."]),
    ("silabisiri.yaml", "neant:\npiei: piei", &["reguli/silabisiri.yaml:1:1: eroare: „neant” nu are nicio silabisire."]),
    ("silabisiri.yaml", "vier: [vier, '']", &["reguli/silabisiri.yaml:1:1: eroare: „vier” are o silabisire goală."]),
    ("invechite.yaml", "leme:\n  - lema:\n    actuala: alinia\n    tulpina: aline\niare_pastrate: []", &["reguli/invechite.yaml:2:5: eroare: „lema” nu are nicio valoare."]),
    ("invechite.yaml", "leme:\n  - lema: a-li-ne-a\n    actuala: alinia\n    tulpina: aline\n    fara_forme:\niare_pastrate: []", &["reguli/invechite.yaml:5:5: eroare: „fara_forme” nu are nicio valoare."]),
    // a byte order mark (UTF-8 with BOM) is no character of the file
    ("a-i.yaml", "\u{feff}# rădăcinile\nradacini_cu_a: [român, rumân]", &[]),
    ("silabisiri.yaml", "\u{feff}neant: ne-ant", &[]),
    ("necunoscute.yaml", "\u{feff}iesc: [eşti]", &["reguli/necunoscute.yaml:1:8: eroare: „eşti” are ş cu sedilă; scrie ș, cu virgulă."]),
    // without forms, the current lemma has at least the stem's letters: its stem is as long
    ("invechite.yaml", "leme:\n  - {lema: ra-chi-a-rea-să, actuala: rachier, tulpina: rachiare, fara_forme: true}\niare_pastrate: []", &["reguli/invechite.yaml:2:38: eroare: „rachier” are mai puțin de 8 litere, câte are tulpina „rachiare”: tulpina actuală e începutul lui, de aceeași lungime."]),
    ("invechite.yaml", "leme:\n  - {lema: ra-chi-a-rea-să, actuala: rachiere, tulpina: rachiare, fara_forme: true}\n  - {lema: ra-chi-a-rea-să-ă, actuala: rachier, tulpina: rachiare}\niare_pastrate: []", &[]),
    // clase.yaml: a class is a capital other than C and V, with letters and digraphs
    ("clase.yaml", "L: [p, b]\nP: [c, ch]\nCC: [x]\nC: [p]\nl: [p]", &["reguli/clase.yaml:3:1: eroare: „CC” nu e un nume de clasă: o clasă se numește cu o singură majusculă, alta decât C și V, care sunt date.", "reguli/clase.yaml:4:1: eroare: „C” nu e un nume de clasă: o clasă se numește cu o singură majusculă, alta decât C și V, care sunt date.", "reguli/clase.yaml:5:1: eroare: „l” nu e un nume de clasă: o clasă se numește cu o singură majusculă, alta decât C și V, care sunt date."]),
    ("clase.yaml", "L: [p, b, p]\nP: [c, chh, Gh]", &["reguli/clase.yaml:1:11: eroare: „p” apare de două ori în „L” (întâi la rândul 1); păstrează-l o singură dată.", "reguli/clase.yaml:2:8: eroare: „chh” are mai mult de două litere; o clasă are litere și digrafe (ch, gh).", "reguli/clase.yaml:2:13: eroare: „Gh” are majuscula „G”; scrie cuvintele cu litere mici."]),
    ("clase.yaml", "L: []\nP: c", &["reguli/clase.yaml:1:1: eroare: „L” e o listă goală.", "reguli/clase.yaml:2:1: eroare: „P” e o listă de cuvinte: P: [cuvânt, cuvânt]."]),
    // a context may hold the classes C, V and those of clase.yaml, not a pattern
    ("hiat-oa.yaml", "- {tipar: [oar], ante: [V, ^CC, LX], exemple: [protozoar]}", &["reguli/hiat-oa.yaml:1:33: eroare: „LX”: clasa „X” nu e definită; clasele sunt C, V și cele din clase.yaml (L, P)."]),
    ("hiat-oa.yaml", "- {tipar: [Coar], exemple: [protozoar]}", &["reguli/hiat-oa.yaml:1:12: eroare: „Coar” are majuscula „C”; scrie cuvintele cu litere mici."]),
    // pasi.yaml: a list of steps, each a mapping of its keys
    ("pasi.yaml", "", &["reguli/pasi.yaml: eroare: fișierul nu are nicio intrare."]),
    ("pasi.yaml", "[]", &["reguli/pasi.yaml: eroare: fișierul nu are nicio intrare.", "reguli/pasi.yaml: eroare: niciun pas nu consultă tabelul diftong-ia.yaml; adaugă un pas cu „tabel: diftong-ia.yaml”.", "reguli/pasi.yaml: eroare: niciun pas nu consultă tabelul hiat-ea.yaml; adaugă un pas cu „tabel: hiat-ea.yaml”.", "reguli/pasi.yaml: eroare: niciun pas nu consultă tabelul hiat-ie.yaml; adaugă un pas cu „tabel: hiat-ie.yaml”.", "reguli/pasi.yaml: eroare: niciun pas nu consultă tabelul hiat-oa.yaml; adaugă un pas cu „tabel: hiat-oa.yaml”."]),
    // a step without its name still counts for the checks of the others: its table, the last step
    ("pasi.yaml", concat!("- {tipar: [ea], tabel: hiat-ea.yaml, rezultat: hiat}\n", "- {pas: oa, tipar: [oa], tabel: hiat-oa.yaml, rezultat: hiat}\n- {pas: ia, tipar: [ia], tabel: diftong-ia.yaml, rezultat: diftong}\n- {pas: ie, tipar: [ie], tabel: hiat-ie.yaml, rezultat: hiat}", "\n- {rezultat: diftong, exemple: [soare]}"), &["reguli/pasi.yaml:1:4: eroare: lipsește cheia „pas”.", "reguli/pasi.yaml:5:4: eroare: lipsește cheia „pas”."]),
    ("pasi.yaml", "pas: restul", &["reguli/pasi.yaml:1:4: eroare: fișierul e o listă de intrări, fiecare începând cu „- ”."]),
    ("pasi.yaml", concat!(tables!(), "- restul", last!()), &["reguli/pasi.yaml:5:3: eroare: un pas are cheile pas, rezultat, tipar, ante, non_ante, post, non_post, prefixe, rest_cunoscut, non_prefixe, doar, tabel, exemple și contraexemple."]),
    ("pasi.yaml", concat!(tables!(), "- {tipar: [ee], exemple: [idee]}", last!()), &["reguli/pasi.yaml:5:4: eroare: lipsește cheia „pas”.", "reguli/pasi.yaml:5:4: eroare: lipsește cheia „rezultat”."]),
    ("pasi.yaml", concat!(tables!(), "- {pas: e-e, tipar: [ee], rezultat: [hiat], exemple: [idee], tabelul: x}", last!()), &["reguli/pasi.yaml:5:27: eroare: „rezultat” are un singur cuvânt, nu o listă.", "reguli/pasi.yaml:5:62: eroare: „tabelul” nu e o cheie cunoscută aici; cheile sunt pas, rezultat, tipar, ante, non_ante, post, non_post, prefixe, rest_cunoscut, non_prefixe, doar, tabel, exemple și contraexemple."]),
    // the values of rezultat, doar and rest_cunoscut
    ("pasi.yaml", concat!(tables!(), "- {pas: e-e, tipar: [ee], rezultat: hiatus, exemple: [idee]}", last!()), &["reguli/pasi.yaml:5:37: eroare: „rezultat” e hiat sau diftong, nu „hiatus”."]),
    ("pasi.yaml", concat!(tables!(), "- {pas: ii, tipar: [ii$], doar: verbe, rezultat: hiat, exemple: [sfii], contraexemple: [copii]}", last!()), &["reguli/pasi.yaml:5:33: eroare: „doar” e verb, nu „verbe”."]),
    ("pasi.yaml", concat!(tables!(), "- {pas: p, prefixe: [re], rest_cunoscut: patru, rezultat: hiat, exemple: [reabilitat], contraexemple: [neam]}\n- {pas: q, prefixe: [ne], rest_cunoscut: 0, rezultat: hiat, exemple: [neabonat], contraexemple: [neam]}", last!()), &["reguli/pasi.yaml:5:42: eroare: „rest_cunoscut” e un număr de litere, nu „patru”.", "reguli/pasi.yaml:6:42: eroare: „rest_cunoscut” e un număr de litere, nu „0”."]),
    // what a key asks of the others
    ("pasi.yaml", concat!(tables!(), "- {pas: p, prefixe: [re], rezultat: hiat, exemple: [reabilitat]}", last!()), &["reguli/pasi.yaml:5:12: eroare: lipsește cheia „rest_cunoscut”: după prefix urmează un cuvânt cunoscut."]),
    ("pasi.yaml", concat!(tables!(), "- {pas: p, prefixe: [re], rest_cunoscut: 4, tipar: [ea], post: [l], rezultat: hiat, exemple: [reabilitat], contraexemple: [neam]}", last!()), &["reguli/pasi.yaml:5:12: eroare: un pas cu prefixe nu are tipar, ante, non_ante, post sau non_post: perechea e ultima vocală a prefixului și litera de după el."]),
    ("pasi.yaml", concat!(tables!(), "- {pas: p, prefixe: [re], rest_cunoscut: 4, ante: [C], rezultat: hiat, exemple: [reabilitat], contraexemple: [neam]}", last!()), &["reguli/pasi.yaml:5:12: eroare: un pas cu prefixe nu are tipar, ante, non_ante, post sau non_post: perechea e ultima vocală a prefixului și litera de după el."]),
    ("pasi.yaml", concat!(tables!(), "- {pas: ier, tipar: [ier, ab], rezultat: hiat, exemple: [plumier]}", last!()), &["reguli/pasi.yaml:5:27: eroare: tiparul „ab” nu are două vocale alăturate, deci nicio pereche pe care s-o decidă pasul."]),
    ("pasi.yaml", concat!(tables!(), "- {pas: e-e, tipar: [ee], rest_cunoscut: 4, non_prefixe: [prea], rezultat: hiat, exemple: [idee], contraexemple: [preacurat]}", last!()), &["reguli/pasi.yaml:5:27: eroare: „rest_cunoscut” cere „prefixe”.", "reguli/pasi.yaml:5:45: eroare: „non_prefixe” cere „prefixe”."]),
    ("pasi.yaml", concat!(tables!(), "- {pas: x, ante: [C], non_post: [l], rezultat: hiat, exemple: [idee], contraexemple: [ideal]}", last!()), &["reguli/pasi.yaml:5:12: eroare: „ante” cere un tipar: contextele stau în jurul lui.", "reguli/pasi.yaml:5:23: eroare: „non_post” cere un tipar: contextele stau în jurul lui."]),
    ("pasi.yaml", concat!(tables!(), "- {pas: e-e, tipar: [ee], rezultat: hiat}", last!()), &["reguli/pasi.yaml:5:4: eroare: lipsește cheia „exemple”."]),
    ("pasi.yaml", concat!(tables!(), "- {pas: ier, tipar: [ier], non_ante: [^C], rezultat: hiat, exemple: [plumier]}", last!()), &["reguli/pasi.yaml:5:4: eroare: lipsește cheia „contraexemple”: non_ante, non_post, non_prefixe, rest_cunoscut și doar cer cuvinte pe care le exclud."]),
    ("pasi.yaml", concat!(tables!(), "- {pas: ii, tipar: [ii$], doar: verb, rezultat: hiat, exemple: [sfii]}", last!()), &["reguli/pasi.yaml:5:4: eroare: lipsește cheia „contraexemple”: non_ante, non_post, non_prefixe, rest_cunoscut și doar cer cuvinte pe care le exclud."]),
    // patterns, contexts, prefixes and words
    ("pasi.yaml", concat!(tables!(), "- {pas: ier, tipar: [ier, ^b, ^iar], rezultat: hiat, exemple: [plumier, iarbă]}", last!()), &["reguli/pasi.yaml:5:27: eroare: tiparul „^b” nu are două vocale alăturate, deci nicio pereche pe care s-o decidă pasul."]),
    ("pasi.yaml", concat!(tables!(), "- {pas: lab, tipar: [ie], ante: [L, LAB, ^CV], post: [Q$], rezultat: diftong, exemple: [fier]}", last!()), &["reguli/pasi.yaml:5:37: eroare: „LAB”: clasa „A” nu e definită; clasele sunt C, V și cele din clase.yaml (L, P).", "reguli/pasi.yaml:5:55: eroare: „Q$”: clasa „Q” nu e definită; clasele sunt C, V și cele din clase.yaml (L, P)."]),
    ("pasi.yaml", concat!(tables!(), "- {pas: p, prefixe: [Re, ne-], rest_cunoscut: 4, non_prefixe: [preş], rezultat: hiat, exemple: [reabilitat], contraexemple: [neam, neşant]}", last!()), &["reguli/pasi.yaml:5:22: eroare: „Re” are majuscula „R”; scrie cuvintele cu litere mici.", "reguli/pasi.yaml:5:26: eroare: „ne-” are o cratimă; cratimele despart doar silabele unei silabisiri.", "reguli/pasi.yaml:5:64: eroare: „preş” are ş cu sedilă; scrie ș, cu virgulă.", "reguli/pasi.yaml:5:132: eroare: „neşant” are ş cu sedilă; scrie ș, cu virgulă."]),
    ("pasi.yaml", concat!(tables!(), "- {pas: p, prefixe: [re, re], rest_cunoscut: 4, rezultat: hiat, exemple: [reabilitat], contraexemple: [neam, reabilitat]}", last!()), &["reguli/pasi.yaml:5:26: eroare: „re” apare de două ori în „prefixe” (întâi la rândul 5); păstrează-l o singură dată.", "reguli/pasi.yaml:5:110: eroare: „reabilitat” e și în exemple, și în contraexemple; păstrează-l într-una singură."]),
    // a table: one that exists, consulted by one step for its pair, its way, with no examples
    ("pasi.yaml", concat!(tables!(), "- {pas: x, tipar: [ea], tabel: hiat-xx.yaml, rezultat: hiat}", last!()), &["reguli/pasi.yaml:5:32: eroare: „hiat-xx.yaml” nu e un tabel de excepții; tabelele sunt diftong-ia.yaml, hiat-ea.yaml, hiat-ie.yaml și hiat-oa.yaml."]),
    ("pasi.yaml", concat!("- {pas: ea, tipar: [ee], tabel: hiat-ea.yaml, rezultat: diftong, exemple: [real]}\n", "- {pas: oa, tipar: [oa], tabel: hiat-oa.yaml, rezultat: hiat}\n- {pas: ia, tipar: [ia], tabel: diftong-ia.yaml, rezultat: diftong}\n- {pas: ie, tipar: [ie], tabel: hiat-ie.yaml, rezultat: hiat}", last!()), &["reguli/pasi.yaml:1:33: eroare: tabelul hiat-ea.yaml face perechea ea hiat, iar pasul are rezultat: diftong.", "reguli/pasi.yaml:1:33: eroare: niciun tipar al pasului nu are perechea ea, singura pe care o decide tabelul hiat-ea.yaml.", "reguli/pasi.yaml:1:66: eroare: un pas cu tabel nu are „exemple”: le au intrările tabelului."]),
    ("pasi.yaml", concat!(tables!(), "- pas: ea2\n  tipar: [ea]\n  tabel: hiat-ea.yaml\n  rezultat: hiat", last!()), &["reguli/pasi.yaml:7:10: eroare: tabelul hiat-ea.yaml e consultat deja de pasul de la rândul 1; un tabel e consultat de un singur pas."]),
    ("pasi.yaml", concat!("- {pas: ea, tipar: [ea], tabel: hiat-ea.yaml, rezultat: hiat}\n- {pas: ia, tipar: [ia], tabel: diftong-ia.yaml, rezultat: diftong}\n- {pas: ie, tipar: [ie], tabel: hiat-ie.yaml, rezultat: hiat}", last!()), &["reguli/pasi.yaml: eroare: niciun pas nu consultă tabelul hiat-oa.yaml; adaugă un pas cu „tabel: hiat-oa.yaml”."]),
    // no name twice, and a last step with no condition
    ("pasi.yaml", concat!(tables!(), "- {pas: restul, tipar: [ee], rezultat: hiat, exemple: [idee]}", last!()), &["reguli/pasi.yaml:6:9: eroare: pasul „restul” apare de două ori (întâi la rândul 5); dă-i alt nume."]),
    ("pasi.yaml", concat!(tables!(), "- {pas: e-e, tipar: [ee], rezultat: hiat, exemple: [idee]}"), &["reguli/pasi.yaml:5:4: eroare: ultimul pas („e-e”) decide orice pereche nedecisă de cei dinainte, deci nu are tipar, prefixe, ante, non_ante, post, non_post, doar sau tabel."]),
    ("pasi.yaml", concat!(tables!(), "- {pas: restul, prefixe: [re], rest_cunoscut: 4, rezultat: diftong, exemple: [reabilitat], contraexemple: [neam]}"), &["reguli/pasi.yaml:5:4: eroare: ultimul pas („restul”) decide orice pereche nedecisă de cei dinainte, deci nu are tipar, prefixe, ante, non_ante, post, non_post, doar sau tabel."]),
    ("pasi.yaml", concat!(tables!(), "- {pas: restul, doar: verb, rezultat: diftong, exemple: [sfii], contraexemple: [copii]}"), &["reguli/pasi.yaml:5:4: eroare: ultimul pas („restul”) decide orice pereche nedecisă de cei dinainte, deci nu are tipar, prefixe, ante, non_ante, post, non_post, doar sau tabel."]),
    ("pasi.yaml", concat!(tables!(), "- {pas: restul, ante: [C], rezultat: diftong, exemple: [soare]}"), &["reguli/pasi.yaml:5:4: eroare: ultimul pas („restul”) decide orice pereche nedecisă de cei dinainte, deci nu are tipar, prefixe, ante, non_ante, post, non_post, doar sau tabel.", "reguli/pasi.yaml:5:17: eroare: „ante” cere un tipar: contextele stau în jurul lui."]),
    ("pasi.yaml", "- {pas: ea, tipar: [ea], tabel: hiat-ea.yaml, rezultat: hiat}\n- {pas: oa, tipar: [oa], tabel: hiat-oa.yaml, rezultat: hiat}\n- {pas: ia, tipar: [ia], tabel: diftong-ia.yaml, rezultat: diftong}\n- {pas: ie, tabel: hiat-ie.yaml, rezultat: hiat}", &["reguli/pasi.yaml:4:4: eroare: ultimul pas („ie”) decide orice pereche nedecisă de cei dinainte, deci nu are tipar, prefixe, ante, non_ante, post, non_post, doar sau tabel.", "reguli/pasi.yaml:4:20: eroare: niciun tipar al pasului nu are perechea ie, singura pe care o decide tabelul hiat-ie.yaml."]),
    ("pasi.yaml", tables!(), &["reguli/pasi.yaml:4:4: eroare: ultimul pas („ie”) decide orice pereche nedecisă de cei dinainte, deci nu are tipar, prefixe, ante, non_ante, post, non_post, doar sau tabel."]),
    ("pasi.yaml", concat!(tables!(), "- pas: e-e\n  tipar: [ee]\n  rezultat: hiat\n  exemple: [idee]", last!()), &[]),
    // files that are no YAML, or hidden, are left alone; a YAML file must be a rule file
    ("notes.txt", "x", &[]),
    (".ee.yaml", "x", &[]),
    ("ee.yml", "x: y", &["reguli/ee.yml: eroare: „ee.yml” nu e un fișier de reguli; fișierele de reguli sunt silabisiri.yaml, invechite.yaml, ee.yaml, neconvertite.yaml, a-i.yaml, necunoscute.yaml, clase.yaml, pasi.yaml, diftong-ia.yaml, hiat-ea.yaml, hiat-ie.yaml și hiat-oa.yaml."]),
    // a file that is not a rule file
    ("silabisri.yaml", "neant: ne-ant", &["reguli/silabisri.yaml: eroare: „silabisri.yaml” nu e un fișier de reguli; fișierele de reguli sunt silabisiri.yaml, invechite.yaml, ee.yaml, neconvertite.yaml, a-i.yaml, necunoscute.yaml, clase.yaml, pasi.yaml, diftong-ia.yaml, hiat-ea.yaml, hiat-ie.yaml și hiat-oa.yaml."]),
];

/// The rule files of the workspace's reguli/, by name, as load() reads them.
fn reguli() -> BTreeMap<String, String> {
    let (texts, messages) = read_texts(&default_dir()).expect("reguli/");
    assert!(messages.is_empty(), "{messages:?}");
    texts
        .into_iter()
        .map(|(name, text)| (name, text.expect("a rule file in UTF-8")))
        .collect()
}

fn strings(messages: &[Message]) -> Vec<String> {
    messages.iter().map(ToString::to_string).collect()
}

#[test]
fn reguli_loads() {
    let (rules, messages) = load_texts(&reguli());
    assert_eq!(strings(&messages), Vec::<String>::new());
    assert!(rules.is_some());
}

/// Each list of LISTS has a constant and a key of its own, in a file of files(), and its plus
/// names a list of the same file. Each table of TABLES has a file of its own, of no list, and a
/// constant of its own, and its pair is two vowels.
#[test]
fn lists() {
    assert_eq!(
        files(),
        [
            "silabisiri.yaml",
            "invechite.yaml",
            "ee.yaml",
            "neconvertite.yaml",
            "a-i.yaml",
            "necunoscute.yaml",
            "clase.yaml",
            "pasi.yaml",
            "diftong-ia.yaml",
            "hiat-ea.yaml",
            "hiat-ie.yaml",
            "hiat-oa.yaml"
        ]
    );
    for (i, t) in TABLES.iter().enumerate() {
        assert!(
            TABLES[..i].iter().all(|o| o.file != t.file && o.constant != t.constant),
            "{t:?}"
        );
        assert!(
            LISTS.iter().all(|l| l.file != t.file && l.constant != t.constant),
            "{t:?}"
        );
        assert!(![SILABISIRI, LEME.0, CLASE, PASI].contains(&t.file), "{t:?}");
        assert!(
            t.pair.chars().count() == 2 && t.pair.chars().all(|c| "aeiou".contains(c)),
            "{t:?}"
        );
        assert_eq!(table(t.file).map(|o| o.constant), Some(t.constant));
    }
    assert!(table("ee.yaml").is_none());
    for (i, l) in LISTS.iter().enumerate() {
        assert!(
            LISTS[..i]
                .iter()
                .all(|o| o.constant != l.constant && (o.file, o.key) != (l.file, l.key)),
            "{l:?}"
        );
        assert!(
            ![SILABISIRI, CLASE, PASI].contains(&l.file) && (l.file, l.key) != LEME,
            "{l:?}"
        );
        assert!(l.plus.is_none_or(|(key, _)| list(l.file, key).is_some()), "{l:?}");
    }
    assert!(list("ee.yaml", "ae_oe").is_some_and(|l| l.plus == Some(("nume", "oe"))));
    assert!(list("ee.yaml", "zz").is_none() && list("a-i.yaml", "nume").is_none());
}

#[test]
fn broken_files() {
    let mut failures = Vec::new();
    for (file, text, expected) in BROKEN {
        let mut texts = reguli();
        texts.insert(file.to_string(), text.to_string());
        let (rules, messages) = load_texts(&texts);
        let got = strings(&messages);
        if got != *expected {
            failures.push(format!("{file} {text:?}:\n  got      {got:?}\n  expected {expected:?}"));
        }
        // the rules come only with no error
        if rules.is_some() == messages.iter().any(|m| m.level == Level::Error) {
            failures.push(format!("{file} {text:?}: rules {}", rules.is_some()));
        }
    }
    assert!(
        failures.is_empty(),
        "{} failing:\n{}",
        failures.len(),
        failures.join("\n")
    );
}

#[test]
fn missing_file() {
    let mut texts = reguli();
    texts.remove("ee.yaml");
    let (rules, messages) = load_texts(&texts);
    assert_eq!(strings(&messages), ["reguli/ee.yaml: eroare: lipsește fișierul."]);
    assert!(rules.is_none());
}

/// A copy of reguli/ in the tests' temporary directory, as `name`, with `files` written over it.
fn reguli_dir(name: &str, files: &[(&str, &str)]) -> PathBuf {
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join(name);
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("a temporary reguli/");
    for (name, text) in reguli() {
        fs::write(dir.join(name), text).expect("a rule file");
    }
    for (name, text) in files {
        fs::write(dir.join(name), text).expect("a rule file");
    }
    dir
}

/// load() reads the YAML files of a directory, and names it as given: a file that is no YAML,
/// or hidden, is left alone; a file not in UTF-8 is an error, and so is a directory that can't
/// be read.
#[test]
fn load_dir() {
    assert_eq!(shown(&default_dir()), "reguli");
    let dir = reguli_dir("reguli", &[]);
    let shown_dir = shown(&dir);
    assert_ne!(shown_dir, "reguli");
    for (name, bytes) in [
        (".DS_Store", &b"\x00\x01\xff"[..]),
        (".ee.yaml.swp", b"\xff"),
        ("ee.yaml~", b"nume: [Zo\xc5\x9f]"),
        ("note.txt", b"x"),
    ] {
        fs::write(dir.join(name), bytes).expect("a file");
    }
    let (rules, messages) = load(&dir);
    assert_eq!((rules.is_some(), strings(&messages)), (true, vec![]));
    assert_eq!(rules, load_texts(&reguli()).0);

    fs::write(dir.join("a-i.yaml"), b"radacini_cu_a: [rom\xe2n]").expect("a rule file");
    let (rules, messages) = load(&dir);
    assert!(rules.is_none());
    assert_eq!(
        strings(&messages),
        [format!(
            "{shown_dir}/a-i.yaml: eroare: fișierul nu e scris în UTF-8; salvează-l ca UTF-8."
        )]
    );

    let missing = dir.join("lipsă");
    let (rules, messages) = load(&missing);
    assert!(rules.is_none());
    assert_eq!(messages.len(), 1, "{messages:?}");
    let text = messages[0].to_string();
    let start = format!("{}/: eroare: dosarul nu poate fi citit (", shown(&missing));
    assert!(text.starts_with(&start), "{text}");
    assert!(
        messages[0]
            .annotation()
            .starts_with("::error::dosarul nu poate fi citit (")
    );
}

// The binary, cargo run -p grafia-reguli, on reguli/ with these files written over it (with none,
// on the workspace's reguli/ itself): (the files, whether GITHUB_ACTIONS is set, the lines it
// prints, {dir} being the directory as shown, whether it succeeds). Add a case = add a line.
type Run = (
    &'static [(&'static str, &'static str)],
    bool,
    &'static [&'static str],
    bool,
);
#[rustfmt::skip]
const RUNS: &[Run] = &[
    (&[], false, &["reguli/: fără erori."], true),
    (&[], true, &["reguli/: fără erori."], true),
    (&[("necunoscute.yaml", "iesc: [eîti]")], false, &["{dir}/necunoscute.yaml:1:8: avertisment: „eîti” are î în interiorul cuvântului; în grafia standard acolo se scrie â (câine), afară de compuse ca neînțeles.", "{dir}/: fără erori, 1 avertisment."], true),
    // the messages by file, in the order of files(), and the files that are no rule files last
    (&[("zz.yaml", "x: y"), ("necunoscute.yaml", "iesc: [eşti, eîti]"), ("a-i.yaml", "radacini_cu_a: [roman]"), ("silabisiri.yaml", "neant: ne-an")], false, &["{dir}/silabisiri.yaml:1:8: eroare: „ne-an” nu are literele lui „neant”.", "{dir}/a-i.yaml:1:17: eroare: „roman” nu are â, deci nu e o rădăcină care își păstrează â.", "{dir}/necunoscute.yaml:1:8: eroare: „eşti” are ş cu sedilă; scrie ș, cu virgulă.", "{dir}/necunoscute.yaml:1:14: avertisment: „eîti” are î în interiorul cuvântului; în grafia standard acolo se scrie â (câine), afară de compuse ca neînțeles.", "{dir}/zz.yaml: eroare: „zz.yaml” nu e un fișier de reguli; fișierele de reguli sunt silabisiri.yaml, invechite.yaml, ee.yaml, neconvertite.yaml, a-i.yaml, necunoscute.yaml, clase.yaml, pasi.yaml, diftong-ia.yaml, hiat-ea.yaml, hiat-ie.yaml și hiat-oa.yaml.", "{dir}/: 4 erori, 1 avertisment."], false),
    // in CI, each message is also an annotation
    (&[("a-i.yaml", "radacini_cu_a: [roman]"), ("necunoscute.yaml", "iesc: [eîti]")], true, &["{dir}/a-i.yaml:1:17: eroare: „roman” nu are â, deci nu e o rădăcină care își păstrează â.", "::error file={dir}/a-i.yaml,line=1,col=17::„roman” nu are â, deci nu e o rădăcină care își păstrează â.", "{dir}/necunoscute.yaml:1:8: avertisment: „eîti” are î în interiorul cuvântului; în grafia standard acolo se scrie â (câine), afară de compuse ca neînțeles.", "::warning file={dir}/necunoscute.yaml,line=1,col=8::„eîti” are î în interiorul cuvântului; în grafia standard acolo se scrie â (câine), afară de compuse ca neînțeles.", "{dir}/: 1 eroare, 1 avertisment."], false),
    // an OS's or an editor's files are left alone
    (&[(".DS_Store", "\u{0}"), ("ee.yaml~", "nume: [Zoş]"), ("note.txt", "x")], true, &["{dir}/: fără erori."], true),
];

#[test]
fn binary() {
    let mut failures = Vec::new();
    for (i, (files, github, expected, success)) in RUNS.iter().enumerate() {
        let mut command = Command::new(env!("CARGO_BIN_EXE_grafia-reguli"));
        let mut dir = "reguli".to_string();
        if !files.is_empty() {
            let path = reguli_dir(&format!("run-{i}"), files);
            dir = shown(&path);
            command.arg(path);
        }
        command.env_remove("GITHUB_ACTIONS");
        if *github {
            command.env("GITHUB_ACTIONS", "true");
        }
        let output = command.output().expect("grafia-reguli runs");
        let stdout = String::from_utf8(output.stdout).expect("UTF-8");
        let got: Vec<&str> = stdout.lines().collect();
        let expected: Vec<String> = expected.iter().map(|l| l.replace("{dir}", &dir)).collect();
        if got != expected || output.status.success() != *success {
            failures.push(format!(
                "run {i}: success {}\n  got      {got:?}\n  expected {expected:?}",
                output.status.success()
            ));
        }
    }
    assert!(
        failures.is_empty(),
        "{} failing:\n{}",
        failures.len(),
        failures.join("\n")
    );
}

// (file, level, line, col, text, the annotation). Add a case = add a line.
#[rustfmt::skip]
const ANNOTATIONS: &[(&str, Level, usize, usize, &str, &str)] = &[
    ("ee.yaml", Level::Error, 3, 30, "„zoş” are ş cu sedilă", "::error file=reguli/ee.yaml,line=3,col=30::„zoş” are ş cu sedilă"),
    ("ee.yaml", Level::Warning, 1, 8, "„eîti” are î", "::warning file=reguli/ee.yaml,line=1,col=8::„eîti” are î"),
    ("ee.yaml", Level::Error, 0, 0, "lipsește fișierul.", "::error file=reguli/ee.yaml::lipsește fișierul."),
    ("ee.yaml", Level::Error, 2, 1, "100% a,b:c\r\nd", "::error file=reguli/ee.yaml,line=2,col=1::100%25 a,b:c%0D%0Ad"),
    // the directory itself: no file
    ("", Level::Error, 0, 0, "dosarul nu poate fi citit.", "::error::dosarul nu poate fi citit."),
];

#[test]
fn annotations() {
    for (file, level, line, col, text, expected) in ANNOTATIONS {
        let m = Message {
            dir: "reguli".into(),
            file: file.to_string(),
            line: *line,
            col: *col,
            level: *level,
            text: text.to_string(),
        };
        assert_eq!(m.annotation(), *expected);
    }
}

/// The Rust code of a small set of rule files: AE_OE holds the names in -oe of NAMES too; a
/// table, its pair and its result, with each entry's patterns and contexts; the classes; the
/// steps, with their tables; no word, comment or name of a step; and the lines of the rules and
/// the names of the steps, apart.
#[test]
fn rust_code() {
    let mut texts = BTreeMap::new();
    for (name, text) in [
        ("silabisiri.yaml", "neant: ne-ant\nvier: [vier, vi-er]"),
        (
            "invechite.yaml",
            "leme:\n  - {lema: a-li-ne-a, actuala: alinia, tulpina: aline}\n  - {lema: ra-chi-a-rea-să, actuala: rachiereasă, tulpina: rachiare, fara_forme: true}\niare_pastrate: [fiare]",
        ),
        (
            "ee.yaml",
            "nume: [andreea, zoe]\nae_oe: [nae, noe]\neea_neschimbate: []\nfara_masculin: [coree]",
        ),
        ("neconvertite.yaml", "nume_latine: [traian]"),
        ("a-i.yaml", "radacini_cu_a: [român, rumân]"),
        ("necunoscute.yaml", "iesc: [ești, asca]"),
        (
            "diftong-ia.yaml",
            "# chiar
- tipar: [chiar, ^biat]
  non_post: [l]
  exemple: [chiar, biată]
  contraexemple: [biatlon]

- tipar: [miaz]
  ante: [^, a]
  exemple: [amiază, miazănoapte]",
        ),
        (
            "hiat-ea.yaml",
            "- {tipar: [eal$], non_ante: [g, d], exemple: [real], contraexemple: [deal, mangeal]}",
        ),
        ("hiat-ie.yaml", "- {tipar: [gien], exemple: [igienă]}"),
        ("hiat-oa.yaml", "- {tipar: [zoar$], post: [e, $], exemple: [protozoar]}"),
        ("clase.yaml", "L: [p, b]  # labiale\nP: [c, ch]"),
        (
            "pasi.yaml",
            "- {pas: e-e, tipar: [ee], rezultat: hiat, exemple: [idee]}
- {pas: prefix, prefixe: [re, co], rest_cunoscut: 4, non_prefixe: [prea], rezultat: hiat, exemple: [reabilitat], contraexemple: [preacurat]}
- {pas: ea, tipar: [ea], tabel: hiat-ea.yaml, rezultat: hiat}
- {pas: oa, tipar: [oa], tabel: hiat-oa.yaml, rezultat: hiat}
- {pas: ii, tipar: [ii$], ante: [C], doar: verb, rezultat: hiat, exemple: [sfii], contraexemple: [copii]}
- {pas: ia, tipar: [ia], ante: [C], tabel: diftong-ia.yaml, rezultat: diftong}
- {pas: ier, tipar: [ier], ante: [L, P], non_ante: [^C], post: [$], rezultat: hiat, exemple: [plumier], contraexemple: [fier]}
- {pas: ie, tipar: [ie], ante: [L, P], tabel: hiat-ie.yaml, rezultat: hiat}
- {pas: restul, rezultat: diftong, exemple: [soare]}",
        ),
    ] {
        texts.insert(name.to_string(), text.to_string());
    }
    let expected = r#"// Generated by grafia-reguli from reguli/ (crates/core/codegen.rs): edit reguli/, not this.
pub const SYLLABIFICATIONS: &[(&str, &[&str])] = &[
    ("neant", &["ne-ant"]),
    ("vier", &["vier", "vi-er"]),
];
pub const OBSOLETE: &[(&str, &str, &str, bool)] = &[
    ("a-li-ne-a", "alinia", "aline", false),
    ("ra-chi-a-rea-să", "rachiereasă", "rachiare", true),
];
pub const IARE_KEPT: &[&str] = &["fiare"];
pub const NAMES: &[&str] = &["andreea", "zoe"];
pub const AE_OE: &[&str] = &["nae", "noe", "zoe"];
pub const EEA_UNCHANGED: &[&str] = &[];
pub const EE_NOT_MASCULINE: &[&str] = &["coree"];
pub const NO_CONVERT: &[&str] = &["traian"];
pub const A_ROOTS: &[&str] = &["român", "rumân"];
pub const IESC_ENDINGS: &[&str] = &["ești", "asca"];
pub static IA_DIPHTHONG: Table = Table { file: "diftong-ia.yaml", pair: ['i', 'a'], cut: false, entries: &[
    Entry { tipar: &["chiar", "^biat"], ante: &[], non_ante: &[], post: &[], non_post: &["l"] },
    Entry { tipar: &["miaz"], ante: &["^", "a"], non_ante: &[], post: &[], non_post: &[] },
] };
pub static EA_HIATUS: Table = Table { file: "hiat-ea.yaml", pair: ['e', 'a'], cut: true, entries: &[
    Entry { tipar: &["eal$"], ante: &[], non_ante: &["g", "d"], post: &[], non_post: &[] },
] };
pub static IE_HIATUS: Table = Table { file: "hiat-ie.yaml", pair: ['i', 'e'], cut: true, entries: &[
    Entry { tipar: &["gien"], ante: &[], non_ante: &[], post: &[], non_post: &[] },
] };
pub static OA_HIATUS: Table = Table { file: "hiat-oa.yaml", pair: ['o', 'a'], cut: true, entries: &[
    Entry { tipar: &["zoar$"], ante: &[], non_ante: &[], post: &["e", "$"], non_post: &[] },
] };
pub const CLASSES: &[(char, &[&str])] = &[
    ('L', &["p", "b"]),
    ('P', &["c", "ch"]),
];
pub static STEPS: [Step; 9] = [
    Step { when: Entry { tipar: &["ee"], ante: &[], non_ante: &[], post: &[], non_post: &[] }, prefixe: &[], rest_cunoscut: 0, non_prefixe: &[], verb: false, tabel: None, cut: true },
    Step { when: Entry { tipar: &[], ante: &[], non_ante: &[], post: &[], non_post: &[] }, prefixe: &["re", "co"], rest_cunoscut: 4, non_prefixe: &["prea"], verb: false, tabel: None, cut: true },
    Step { when: Entry { tipar: &["ea"], ante: &[], non_ante: &[], post: &[], non_post: &[] }, prefixe: &[], rest_cunoscut: 0, non_prefixe: &[], verb: false, tabel: Some(&EA_HIATUS), cut: true },
    Step { when: Entry { tipar: &["oa"], ante: &[], non_ante: &[], post: &[], non_post: &[] }, prefixe: &[], rest_cunoscut: 0, non_prefixe: &[], verb: false, tabel: Some(&OA_HIATUS), cut: true },
    Step { when: Entry { tipar: &["ii$"], ante: &["C"], non_ante: &[], post: &[], non_post: &[] }, prefixe: &[], rest_cunoscut: 0, non_prefixe: &[], verb: true, tabel: None, cut: true },
    Step { when: Entry { tipar: &["ia"], ante: &["C"], non_ante: &[], post: &[], non_post: &[] }, prefixe: &[], rest_cunoscut: 0, non_prefixe: &[], verb: false, tabel: Some(&IA_DIPHTHONG), cut: false },
    Step { when: Entry { tipar: &["ier"], ante: &["L", "P"], non_ante: &["^C"], post: &["$"], non_post: &[] }, prefixe: &[], rest_cunoscut: 0, non_prefixe: &[], verb: false, tabel: None, cut: true },
    Step { when: Entry { tipar: &["ie"], ante: &["L", "P"], non_ante: &[], post: &[], non_post: &[] }, prefixe: &[], rest_cunoscut: 0, non_prefixe: &[], verb: false, tabel: Some(&IE_HIATUS), cut: true },
    Step { when: Entry { tipar: &[], ante: &[], non_ante: &[], post: &[], non_post: &[] }, prefixe: &[], rest_cunoscut: 0, non_prefixe: &[], verb: false, tabel: None, cut: false },
];
"#;
    let expected_meta = r#"// Generated by grafia-reguli from reguli/ (crates/core/codegen.rs): the lines of the rules, not in RULES_HASH.
pub const SYLLABIFICATION_LINES: &[usize] = &[1, 2];
pub static TABLES: [&Table; 4] = [&IA_DIPHTHONG, &EA_HIATUS, &IE_HIATUS, &OA_HIATUS];
pub const ENTRY_LINES: [&[usize]; 4] = [
    &[2, 7],
    &[1],
    &[1],
    &[1],
];
pub const STEP_NAMES: [&str; 9] = ["e-e", "prefix", "ea", "oa", "ii", "ia", "ier", "ie", "restul"];
pub const STEP_LINES: [usize; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
"#;
    let (rules, messages) = load_texts(&texts);
    assert_eq!(strings(&messages), Vec::<String>::new());
    let rules = rules.expect("rules");
    assert_eq!(rust(&rules), expected);
    assert_eq!(meta(&rules), expected_meta);
}
