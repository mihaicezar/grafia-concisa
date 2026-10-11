//! The records of the development data file, built without MariaDB.
use grafia_concisa::build::Record;

/// Syllabified headwords.
const HEADWORDS: &str = "
soa-re sea-ră sea-ra mea ta-ta ie-pu-re ie-pu-raș fier
ceai cea-sor-nic cea-i-nă-ri-e ia-urt iar-bă foar-fe-cu-ță doam-nă
leoar-că a-ri-pioa-ră beai iei ploa-ie șoa-re-ce
a-ce-ea
fi-e-ca-re re-al te-a-tru pi-an bo-a co-a-li-ți-e vi-e
";

/// (form, lemma, the lemma's hyphenation: complete, partial or "", number)
const FORMS: &[[&str; 4]] = &[
    ["idee", "idee", "i-de-e", "S"],
    ["ideea", "idee", "i-de-e", "S"],
    ["ideei", "idee", "i-de-e", "S"],
    ["alee", "alee", "a-le-e", "S"],
    ["bombee", "bombeu", "bom-beu", "P"],
    ["bombeele", "bombeu", "bom-beu", "P"],
    ["bombeelor", "bombeu", "bom-beu", "P"],
    ["logoree", "logoree", "lo-go-re-e", "S"],
    ["orhidee", "orhidee", "or-hi-de-e", "S"],
    ["orhideea", "orhidee", "or-hi-de-e", "S"],
    ["orhideei", "orhidee", "or-hi-de-e", "S"],
    ["orhideele", "orhidee", "or-hi-de-e", "P"],
    ["orhideelor", "orhidee", "or-hi-de-e", "P"],
    ["femeia", "femeie", "fe-me-ie", "S"],
    ["niceea", "niceea", "", "S"],
    ["spontaneea", "spontaneu", "", "S"],
    ["familia", "familie", "-li-e", "S"],
    ["soarele", "soare", "", "S"],
];

pub fn records() -> impl Iterator<Item = Record> {
    let headwords = HEADWORDS.split_whitespace().map(|h| Record::new(&[h]));
    headwords.chain(FORMS.iter().map(|r| Record::new(r)))
}
