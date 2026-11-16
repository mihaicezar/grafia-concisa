//! Each rule file, read into `Rules`, with the checks of its lists, its tables and its steps.
use std::collections::HashMap;

use marked_yaml::Node;
use marked_yaml::types::{MarkedMappingNode, MarkedSequenceNode};

use crate::yaml::{At, File, VOWELS, duplicates, join, node_at, value};
use crate::{
    CLASE, CONTEXTS, CONTRAEXEMPLE, EXEMPLE, Entry, GIVEN_CLASSES, LEME, LISTS, Level, List, Message, Obsolete, PASI,
    Rules, SILABISIRI, Shape, Step, TABLES, TIPAR, Table, Texts, Value, files, is_yaml, list, table,
};

/// The rules of the files in `texts`, by name (None: a file that couldn't be read, already in
/// `messages`), unless a message is an error, and the messages, by file, line and column, in
/// the directory `dir` (as `shown` names it). Names that `is_yaml` rejects are left alone.
pub(crate) fn read(texts: &Texts, mut messages: Vec<Message>, dir: &str) -> (Option<Rules>, Vec<Message>) {
    let files = files();
    for name in texts.keys().filter(|n| is_yaml(n) && !files.contains(&n.as_str())) {
        messages.push(Message {
            dir: String::new(),
            file: name.clone(),
            line: 0,
            col: 0,
            level: Level::Error,
            text: format!(
                "„{name}” nu e un fișier de reguli; fișierele de reguli sunt {}.",
                join(&files, "și")
            ),
        });
    }
    let mut rules = Rules::default();
    // the capitals of the classes of clase.yaml, read before the files whose contexts use them
    let mut classes = String::new();
    for &name in &files {
        let mut f = File {
            name,
            messages: &mut messages,
            classes: classes.clone(),
        };
        let Some(text) = texts.get(name) else {
            f.error((0, 0), "lipsește fișierul.".into());
            continue;
        };
        if name == PASI {
            if let Some(seq) = text.as_deref().and_then(|text| f.sequence(text)) {
                rules.pasi = steps(&mut f, &seq);
            }
            continue;
        }
        if let Some(t) = table(name) {
            if let Some(seq) = text.as_deref().and_then(|text| f.sequence(text)) {
                rules.tables.insert(name, entries(&mut f, &seq, t));
            }
            continue;
        }
        let Some(map) = text.as_deref().and_then(|text| f.mapping(text)) else {
            continue;
        };
        if name == SILABISIRI {
            silabisiri(&mut f, &map, &mut rules);
            continue;
        }
        if name == CLASE {
            rules.clase = clase(&mut f, &map);
            classes = rules.clase.iter().map(|(c, _)| c.text.as_str()).collect();
            continue;
        }
        let lists: Vec<&List> = LISTS.iter().filter(|l| l.file == name).collect();
        let mut keys: Vec<&str> = Vec::new();
        if name == LEME.0 {
            keys.push(LEME.1);
        }
        keys.extend(lists.iter().map(|l| l.key));
        let fields = f.fields(&map, &keys, &[], (0, 0));
        if let Some((key, node)) = fields.get(LEME.1) {
            rules.leme = leme(&mut f, key, node);
        }
        for list in &lists {
            if let Some((key, node)) = fields.get(list.key) {
                let words = words(&mut f, key, node, list);
                rules.lists.insert(list.constant, words);
            }
        }
        // the words that a list holds from another one are not written in it again
        for here in &lists {
            let Some((key, ending)) = here.plus else { continue };
            let there = list(name, key).expect("plus names a list of the same file");
            if let (Some(written), Some(theirs)) = (rules.lists.get(here.constant), rules.lists.get(there.constant)) {
                for w in written {
                    if w.text.ends_with(ending) && theirs.iter().any(|t| t.text == w.text) {
                        f.error(
                            w.at(),
                            format!(
                                "„{}” e deja în „{key}”, ale cărui cuvinte în -{ending} sunt și în „{}”; șterge-l de aici.",
                                w.text, here.key
                            ),
                        );
                    }
                }
            }
        }
    }
    for m in &mut messages {
        if m.dir.is_empty() {
            m.dir = dir.to_string();
        }
    }
    let order = |m: &Message| files.iter().position(|n| *n == m.file).unwrap_or(files.len());
    messages.sort_by_key(|m| (order(m), m.line, m.col));
    let errors = messages.iter().any(|m| m.level == Level::Error);
    ((!errors).then_some(rules), messages)
}

/// What is wrong with `w`, a word of `list`, if it isn't of the list's shape.
fn misfit(list: &List, w: &str) -> Option<String> {
    let key = list.key;
    match list.shape {
        Shape::Any => None,
        Shape::Ending(endings) => (!endings.iter().any(|e| w.ends_with(e))).then(|| {
            let dashed: Vec<String> = endings.iter().map(|e| format!("-{e}")).collect();
            let dashed: Vec<&str> = dashed.iter().map(String::as_str).collect();
            format!(
                "„{w}” nu se termină în {}, ca toate cuvintele din „{key}”.",
                join(&dashed, "sau")
            )
        }),
        Shape::VowelIan => {
            let vowel_ian = w
                .strip_suffix("ian")
                .and_then(|s| s.chars().last())
                .is_some_and(|c| VOWELS.contains(c));
            (!vowel_ian).then(|| format!("„{w}” nu se termină într-o vocală + -ian, ca toate numele din „{key}”."))
        }
        Shape::WithA => {
            (!w.contains('â')).then(|| format!("„{w}” nu are â, deci nu e o rădăcină care își păstrează â."))
        }
    }
}

/// The words of `list`, the value of `key`: their letters, their shape when their letters
/// are right, and no word twice.
fn words(f: &mut File, key: &Value, node: &Node, list: &List) -> Vec<Value> {
    let words = f.list(key, node);
    for w in &words {
        if f.letters(w, false) {
            f.inner_i(w);
            if let Some(error) = misfit(list, &w.text) {
                f.error(w.at(), error);
            }
        }
    }
    twice(f, &key.text, &words);
    words
}

/// An error for each value written twice in the list of `key`.
fn twice(f: &mut File, key: &str, values: &[Value]) {
    for (first, again) in duplicates(values, |v| v.text.clone()) {
        f.error(
            again.at(),
            format!(
                "„{}” apare de două ori în „{key}” (întâi la rândul {}); păstrează-l o singură dată.",
                again.text, first.line
            ),
        );
    }
}

/// The entries of the exception table `t`, each a mapping of the keys `TIPAR`, `CONTEXTS`,
/// `EXEMPLE` and `CONTRAEXEMPLE`: their patterns, the items of their contexts, their words, no
/// value twice in a list, and no word both an example and a counterexample.
fn entries(f: &mut File, seq: &MarkedSequenceNode, t: &Table) -> Vec<Entry> {
    // an empty table would drop its exceptions without a word
    if seq.is_empty() {
        f.error((0, 0), "fișierul nu are nicio intrare.".into());
    }
    let mut entries = Vec::new();
    for item in seq.iter() {
        let Some(map) = item.as_mapping() else {
            f.error(
                node_at(item),
                format!(
                    "o intrare are cheile {TIPAR}, {}, {EXEMPLE} și {CONTRAEXEMPLE}.",
                    CONTEXTS.join(", ")
                ),
            );
            continue;
        };
        let (line, col) = map.keys().next().map_or(node_at(item), |k| value(k).at());
        let optional: Vec<&'static str> = CONTEXTS.iter().copied().chain([CONTRAEXEMPLE]).collect();
        let fields = f.fields(map, &[TIPAR, EXEMPLE], &optional, (line, col));
        let entry = entry(f, &fields, (line, col));
        // the excluding contexts need words that they exclude
        let excludes = CONTEXTS
            .iter()
            .zip(&entry.contexts)
            .any(|(k, items)| k.starts_with("non_") && !items.is_empty());
        if excludes && !fields.contains_key(CONTRAEXEMPLE) {
            f.error(
                (line, col),
                format!("lipsește cheia „{CONTRAEXEMPLE}”: non_ante și non_post cer cuvinte pe care le exclud."),
            );
        }
        for p in &entry.tipar {
            if f.pattern(p, TIPAR) && !p.text.contains(t.pair) {
                f.error(
                    p.at(),
                    format!(
                        "tiparul „{}” nu are perechea {}, singura pe care o decide tabelul acesta.",
                        p.text, t.pair
                    ),
                );
            }
        }
        check_entry(f, &entry);
        entries.push(entry);
    }
    entries
}

/// The values of the fields of a mapping, by key (`File::fields`).
type Fields<'n> = HashMap<&'static str, (Value, &'n Node)>;

/// The patterns, contexts and words of an entry of a table or of a step, from its `fields`; it
/// starts at `at`.
fn entry(f: &mut File, fields: &Fields, (line, col): At) -> Entry {
    let mut list = |key: &str| words_of(f, fields, key);
    Entry {
        line,
        col,
        tipar: list(TIPAR),
        contexts: CONTEXTS.map(&mut list),
        exemple: list(EXEMPLE),
        contraexemple: list(CONTRAEXEMPLE),
    }
}

/// The values of the list of `key` in `fields`, if it has one: not empty, and no value twice.
fn words_of(f: &mut File, fields: &Fields, key: &str) -> Vec<Value> {
    match fields.get(key) {
        Some((k, node)) => {
            let values = f.list(k, node);
            if node.as_sequence().is_some_and(|s| s.is_empty()) {
                f.error(k.at(), format!("„{key}” e o listă goală."));
            }
            twice(f, key, &values);
            values
        }
        None => Vec::new(),
    }
}

/// Checks the items of the contexts of an entry of a table or of a step, and its words: their
/// letters, and no word both an example and a counterexample.
fn check_entry(f: &mut File, entry: &Entry) {
    for (key, items) in CONTEXTS.iter().zip(&entry.contexts) {
        for item in items {
            f.pattern(item, key);
        }
    }
    for w in entry.exemple.iter().chain(&entry.contraexemple) {
        if f.letters(w, false) {
            f.inner_i(w);
        }
    }
    for c in &entry.contraexemple {
        if entry.exemple.iter().any(|e| e.text == c.text) {
            f.error(
                c.at(),
                format!(
                    "„{}” e și în {EXEMPLE}, și în {CONTRAEXEMPLE}; păstrează-l într-una singură.",
                    c.text
                ),
            );
        }
    }
}

/// `clase.yaml`: each class, a capital other than C and V, with its letters and digraphs, none
/// twice.
fn clase(f: &mut File, map: &MarkedMappingNode) -> Vec<(Value, Vec<Value>)> {
    let mut clase = Vec::new();
    for (key, node) in map.iter() {
        let name = value(key);
        let mut chars = name.text.chars();
        let capital = match (chars.next(), chars.next()) {
            (Some(c), None) => c.is_ascii_uppercase() && !GIVEN_CLASSES.contains(&c),
            _ => false,
        };
        if !capital {
            f.error(
                name.at(),
                format!(
                    "„{}” nu e un nume de clasă: o clasă se numește cu o singură majusculă, alta decât C și V, care sunt date.",
                    name.text
                ),
            );
            continue;
        }
        let letters = f.list(&name, node);
        if node.as_sequence().is_some_and(|s| s.is_empty()) {
            f.error(name.at(), format!("„{}” e o listă goală.", name.text));
        }
        for l in &letters {
            if f.letters(l, false) && l.text.chars().count() > 2 {
                f.error(
                    l.at(),
                    format!(
                        "„{}” are mai mult de două litere; o clasă are litere și digrafe (ch, gh).",
                        l.text
                    ),
                );
            }
        }
        twice(f, &name.text, &letters);
        clase.push((name, letters));
    }
    clase
}

/// The keys of a step: the required ones, then the others.
const STEP_REQUIRED: [&str; 2] = ["pas", "rezultat"];
const STEP_OPTIONAL: [&str; 12] = [
    TIPAR,
    "ante",
    "non_ante",
    "post",
    "non_post",
    "prefixe",
    "rest_cunoscut",
    "non_prefixe",
    "doar",
    "tabel",
    EXEMPLE,
    CONTRAEXEMPLE,
];

/// `pasi.yaml`: the steps, in order, each with the checks of an entry of a table and what each
/// of its keys asks of the others; no name twice, each table consulted by one step, and a last
/// step with no condition, so that every pair is decided.
fn steps(f: &mut File, seq: &MarkedSequenceNode) -> Vec<Step> {
    // with no step, no pair would be decided
    if seq.is_empty() {
        f.error((0, 0), "fișierul nu are nicio intrare.".into());
    }
    let mut steps = Vec::new();
    for item in seq.iter() {
        let Some(map) = item.as_mapping() else {
            f.error(
                node_at(item),
                format!(
                    "un pas are cheile {}.",
                    join(&[&STEP_REQUIRED[..], &STEP_OPTIONAL[..]].concat(), "și")
                ),
            );
            continue;
        };
        let at: At = map.keys().next().map_or(node_at(item), |k| value(k).at());
        let fields = f.fields(map, &STEP_REQUIRED, &STEP_OPTIONAL, at);
        steps.push(step(f, &fields, at));
    }
    // a step without its name (an error already) has an empty one
    let names: Vec<Value> = steps
        .iter()
        .filter(|s| !s.pas.text.is_empty())
        .map(|s| s.pas.clone())
        .collect();
    for (first, again) in duplicates(&names, |n| n.text.clone()) {
        f.error(
            again.at(),
            format!(
                "pasul „{}” apare de două ori (întâi la rândul {}); dă-i alt nume.",
                again.text, first.line
            ),
        );
    }
    // each table is consulted by one step: the line of the step that names it first, by file
    let mut named: HashMap<&str, usize> = HashMap::new();
    for s in &steps {
        let Some(t) = &s.tabel else { continue };
        match named.get(t.text.as_str()) {
            Some(line) => f.error(
                t.at(),
                format!(
                    "tabelul {} e consultat deja de pasul de la rândul {line}; un tabel e consultat de un singur pas.",
                    t.text
                ),
            ),
            None => {
                named.insert(&t.text, s.entry.line);
            }
        }
    }
    for t in &TABLES {
        if !named.contains_key(t.file) {
            f.error(
                (0, 0),
                format!(
                    "niciun pas nu consultă tabelul {0}; adaugă un pas cu „tabel: {0}”.",
                    t.file
                ),
            );
        }
    }
    if let Some(last) = steps.last() {
        let e = &last.entry;
        let conditions = !e.tipar.is_empty()
            || e.contexts.iter().any(|c| !c.is_empty())
            || !last.prefixe.is_empty()
            || last.verb
            || last.tabel.is_some();
        if conditions {
            f.error(
                (e.line, e.col),
                format!(
                    "ultimul pas („{}”) decide orice pereche nedecisă de cei dinainte, deci nu are tipar, prefixe, ante, non_ante, post, non_post, doar sau tabel.",
                    last.pas.text
                ),
            );
        }
    }
    steps
}

/// A step of `pasi.yaml`, from its `fields`, with its checks; it starts at `at`. Without its
/// name (an error already), its name is empty, so that the checks of the other steps still see it.
fn step(f: &mut File, fields: &Fields, at: At) -> Step {
    let mut scalar = |k: &str| fields.get(k).and_then(|(key, node)| f.scalar(key, node));
    let (pas, rezultat, rest, doar, tabel) = (
        scalar("pas"),
        scalar("rezultat"),
        scalar("rest_cunoscut"),
        scalar("doar"),
        scalar("tabel"),
    );
    let entry = entry(f, fields, at);
    let prefixe = words_of(f, fields, "prefixe");
    let non_prefixe = words_of(f, fields, "non_prefixe");
    let key_at = |k: &str| fields.get(k).map(|(key, _)| key.at());
    let hiat = match &rezultat {
        Some(v) if v.text == "hiat" => true,
        Some(v) if v.text == "diftong" => false,
        Some(v) => {
            f.error(v.at(), format!("„rezultat” e hiat sau diftong, nu „{}”.", v.text));
            false
        }
        None => false,
    };
    let verb = match &doar {
        Some(v) if v.text == "verb" => true,
        Some(v) => {
            f.error(v.at(), format!("„doar” e verb, nu „{}”.", v.text));
            false
        }
        None => false,
    };
    let rest_cunoscut = match &rest {
        Some(v) => match v.text.parse::<usize>() {
            Ok(n) if n > 0 => n,
            _ => {
                f.error(
                    v.at(),
                    format!("„rest_cunoscut” e un număr de litere, nu „{}”.", v.text),
                );
                0
            }
        },
        None => 0,
    };
    // the pair is after a prefix, or in a pattern, with its contexts around it
    if let Some(p) = key_at("prefixe") {
        if !entry.tipar.is_empty() || entry.contexts.iter().any(|c| !c.is_empty()) {
            f.error(
                p,
                "un pas cu prefixe nu are tipar, ante, non_ante, post sau non_post: perechea e ultima vocală a prefixului și litera de după el.".into(),
            );
        }
        if rest.is_none() {
            f.error(
                p,
                "lipsește cheia „rest_cunoscut”: după prefix urmează un cuvânt cunoscut.".into(),
            );
        }
    } else {
        for k in ["rest_cunoscut", "non_prefixe"] {
            if let Some(at) = key_at(k) {
                f.error(at, format!("„{k}” cere „prefixe”."));
            }
        }
        if entry.tipar.is_empty() {
            for k in CONTEXTS {
                if let Some(at) = key_at(k) {
                    f.error(at, format!("„{k}” cere un tipar: contextele stau în jurul lui."));
                }
            }
        }
    }
    for p in &entry.tipar {
        if f.pattern(p, TIPAR) && !has_pair(&p.text) {
            f.error(
                p.at(),
                format!(
                    "tiparul „{}” nu are două vocale alăturate, deci nicio pereche pe care s-o decidă pasul.",
                    p.text
                ),
            );
        }
    }
    for w in prefixe.iter().chain(&non_prefixe) {
        f.letters(w, false);
    }
    check_entry(f, &entry);
    // a table holds the examples of the step that consults it, for its pair, its way
    if let Some(t) = &tabel {
        match table(&t.text) {
            None => {
                let tables: Vec<&str> = TABLES.iter().map(|t| t.file).collect();
                f.error(
                    t.at(),
                    format!(
                        "„{}” nu e un tabel de excepții; tabelele sunt {}.",
                        t.text,
                        join(&tables, "și")
                    ),
                );
            }
            Some(table) => {
                let way = |hiat: bool| if hiat { "hiat" } else { "diftong" };
                if rezultat.is_some() && table.hiat != hiat {
                    f.error(
                        t.at(),
                        format!(
                            "tabelul {} face perechea {} {}, iar pasul are rezultat: {}.",
                            table.file,
                            table.pair,
                            way(table.hiat),
                            way(hiat)
                        ),
                    );
                }
                if !entry.tipar.iter().any(|p| p.text.contains(table.pair)) {
                    f.error(
                        t.at(),
                        format!(
                            "niciun tipar al pasului nu are perechea {}, singura pe care o decide tabelul {}.",
                            table.pair, table.file
                        ),
                    );
                }
            }
        }
        for k in [EXEMPLE, CONTRAEXEMPLE] {
            if let Some(at) = key_at(k) {
                f.error(at, format!("un pas cu tabel nu are „{k}”: le au intrările tabelului."));
            }
        }
    } else {
        if !fields.contains_key(EXEMPLE) {
            f.error(at, format!("lipsește cheia „{EXEMPLE}”."));
        }
        // the excluding conditions need words that they exclude
        let excluding = ["non_ante", "non_post", "non_prefixe", "rest_cunoscut", "doar"];
        if excluding.iter().any(|k| fields.contains_key(k)) && !fields.contains_key(CONTRAEXEMPLE) {
            f.error(
                at,
                format!(
                    "lipsește cheia „{CONTRAEXEMPLE}”: non_ante, non_post, non_prefixe, rest_cunoscut și doar cer cuvinte pe care le exclud."
                ),
            );
        }
    }
    Step {
        pas: pas.unwrap_or_else(|| Value {
            text: String::new(),
            line: at.0,
            col: at.1,
        }),
        entry,
        prefixe,
        rest_cunoscut,
        non_prefixe,
        verb,
        tabel,
        hiat,
    }
}

/// True if the letters of a pattern hold two vowels side by side: a pair it can decide.
fn has_pair(pattern: &str) -> bool {
    let letters: Vec<char> = pattern.chars().collect();
    letters
        .windows(2)
        .any(|w| VOWELS.contains(w[0]) && VOWELS.contains(w[1]))
}

/// Checks a syllabification of `word`: its letters are the word's, and no syllable is empty.
fn syllabification(f: &mut File, s: &Value, word: &str) {
    if s.text.replace('-', "") != word {
        f.error(s.at(), format!("„{}” nu are literele lui „{word}”.", s.text));
    } else if s.text.starts_with('-') || s.text.ends_with('-') || s.text.contains("--") {
        f.error(
            s.at(),
            format!(
                "„{}” are o silabă goală: o cratimă la început, la sfârșit sau două la rând.",
                s.text
            ),
        );
    }
}

/// `silabisiri.yaml`: each word, then its syllabification or the list of them.
fn silabisiri(f: &mut File, map: &MarkedMappingNode, rules: &mut Rules) {
    // an empty file would drop every explicit syllabification without a word
    if map.is_empty() {
        f.error((0, 0), "fișierul nu are nicio silabisire.".into());
    }
    for (key, node) in map.iter() {
        let word = value(key);
        let letters = f.letters(&word, false);
        if letters {
            f.inner_i(&word);
        }
        // an empty value starts where the next one does: it is reported at its word
        let readings: Vec<Value> = match node {
            Node::Scalar(s) if s.as_str().is_empty() => {
                f.error(word.at(), format!("„{}” nu are nicio silabisire.", word.text));
                Vec::new()
            }
            Node::Scalar(s) => vec![value(s)],
            Node::Sequence(seq) => {
                if seq.is_empty() {
                    f.error(word.at(), format!("„{}” nu are nicio silabisire.", word.text));
                }
                seq.iter()
                    .filter_map(|item| match item {
                        Node::Scalar(s) if s.as_str().is_empty() => {
                            f.error(word.at(), format!("„{}” are o silabisire goală.", word.text));
                            None
                        }
                        Node::Scalar(s) => Some(value(s)),
                        _ => {
                            f.error(
                                node_at(item),
                                format!("o silabisire a lui „{}” nu e un cuvânt despărțit în silabe.", word.text),
                            );
                            None
                        }
                    })
                    .collect()
            }
            Node::Mapping(_) => {
                f.error(
                    word.at(),
                    format!(
                        "„{0}” e urmat de o silabisire sau de o listă de silabisiri: {0}: si-la-bi-si-re.",
                        word.text
                    ),
                );
                Vec::new()
            }
        };
        for r in &readings {
            if f.letters(r, true) && letters {
                syllabification(f, r, &word.text);
            }
        }
        for (_, again) in duplicates(&readings, |r| r.text.clone()) {
            f.error(
                again.at(),
                format!("silabisirea „{}” apare de două ori la „{}”.", again.text, word.text),
            );
        }
        rules.silabisiri.push((word, readings));
    }
}

/// `leme` in `invechite.yaml`: each obsolete lemma, with its current lemma and its stem.
fn leme(f: &mut File, key: &Value, node: &Node) -> Vec<Obsolete> {
    let Some(seq) = node.as_sequence() else {
        f.error(
            key.at(),
            "„leme” e o listă de leme învechite, fiecare cu lema, actuala și tulpina ei.".into(),
        );
        return Vec::new();
    };
    let mut leme = Vec::new();
    for item in seq.iter() {
        let Some(map) = item.as_mapping() else {
            f.error(
                node_at(item),
                "o lemă din „leme” are cheile lema, actuala, tulpina și, dacă e cazul, fara_forme.".into(),
            );
            continue;
        };
        let at: At = map.keys().next().map_or(node_at(item), |k| value(k).at());
        let fields = f.fields(map, &["lema", "actuala", "tulpina"], &["fara_forme"], at);
        let mut scalar = |k: &str| fields.get(k).and_then(|(key, node)| f.scalar(key, node));
        let (lema, actuala, tulpina, fara_forme) = (
            scalar("lema"),
            scalar("actuala"),
            scalar("tulpina"),
            scalar("fara_forme"),
        );
        let fara_forme = match fara_forme {
            None => false,
            Some(v) if v.text == "true" => true,
            Some(v) if v.text == "false" => false,
            Some(v) => {
                f.error(v.at(), format!("„fara_forme” e true sau false, nu „{}”.", v.text));
                false
            }
        };
        let (Some(lema), Some(actuala), Some(tulpina)) = (lema, actuala, tulpina) else {
            continue;
        };
        let letters = f.letters(&lema, true);
        if letters {
            f.inner_i(&lema);
            syllabification(f, &lema, &lema.text.replace('-', ""));
        }
        let actuala_letters = f.letters(&actuala, false);
        if actuala_letters {
            f.inner_i(&actuala);
        }
        let tulpina_letters = f.letters(&tulpina, false);
        if tulpina_letters && letters && !lema.text.replace('-', "").starts_with(&tulpina.text) {
            f.error(
                tulpina.at(),
                format!(
                    "tulpina „{}” nu e începutul lemei „{}”.",
                    tulpina.text,
                    lema.text.replace('-', "")
                ),
            );
        }
        // without forms, a missing word with the stem takes the current stem: as many letters
        // of the current lemma as the stem has
        let n = tulpina.text.chars().count();
        if fara_forme && tulpina_letters && actuala_letters && actuala.text.chars().count() < n {
            f.error(
                actuala.at(),
                format!(
                    "„{}” are mai puțin de {n} litere, câte are tulpina „{}”: tulpina actuală e începutul lui, de aceeași lungime.",
                    actuala.text, tulpina.text
                ),
            );
        }
        leme.push(Obsolete {
            lema,
            actuala,
            tulpina,
            fara_forme,
        });
    }
    let lemmas: Vec<Value> = leme.iter().map(|o| o.lema.clone()).collect();
    for (first, again) in duplicates(&lemmas, |l| l.text.replace('-', "")) {
        f.error(
            again.at(),
            format!(
                "lema „{}” apare de două ori în „leme” (întâi la rândul {}); păstreaz-o o singură dată.",
                again.text.replace('-', ""),
                first.line
            ),
        );
    }
    leme
}
