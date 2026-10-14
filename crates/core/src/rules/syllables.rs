//! The syllabification of a word: dexonline's cuts, completed by the fallback rules.
use std::collections::HashSet;
use std::sync::LazyLock;

use fancy_regex::Regex;

use super::*;

pub(super) const VOWELS: &str = "aeiouăâî";
pub(super) const LABIALS: &str = "pbfvm";

pub(super) fn is_vowel(c: char) -> bool {
    VOWELS.contains(c)
}

// Matched at the start of the word's tail from the i (Python: IENT.match(word, b - 1)).
pub(super) static IENT: LazyLock<Regex> = LazyLock::new(|| re(r"^ien[tț](?:[aăei]|$)"));
// Medial ia after a consonant is a hiatus (the -ia* endings: apropiat, social,
// specializat), except in these native stems (chiar, biată, piatră, amiază).
pub static IA_DIPHTHONG: LazyLock<Regex> = LazyLock::new(|| {
    re(concat!(
        r"chiar|chiab|(?<!s)chiag|chiaun|unchiaș",     // not schiagrafie
        r"|(?<!n)(?<!litur)(?<!^gor)ghia(?![tnl]|ce)", // not unghiar, -ghiat, -ghian, -ghial,
        // liturghiar, Gorghias, eleghiacesc
        r"|^biat(?!l)|(?<!hi)piatr|viaț(?!i)",   // not biatlon, hipiatric, deviație
        r"|(?:^|[aă])miaz(?!m)|(?<!e)miau(?!l)", // not distomiază, miazmă, semiautomat, Miaulis
        r"|megiaș|caragial|barcian|vianu$|chiajn|^obial",
    ))
});
// ea is a hiatus in these neologism endings and stems (real, nuclear, creat,
// realizat); native deal, beat, asear, greșeală stay diphthongs.
pub static EA_HIATUS: LazyLock<Regex> = LazyLock::new(|| {
    re(concat!(
        r"(?<![gdb])eal$",               // real, leal, boreal, interliceal; not mangeal, deal, podbeal
        r"|(?:lin|hl|baln)ear$|nuclear", // linear, cohlear, balnear, nuclearizat
        r"|(?:gr|aur|lor|lin|ucl|un|ct|id|upl)eat|creat", // laureat(ă), nucleat(ă), delineator; not cleat, pleată
        r"|(?:real|ideal)i|(?<!ră)(?<!sto)boreal|cereal", // realizat, idealist, cereale; not răboreală
        r"|reac[tț]|steat|meandr|aleat|beatif", // reactor, reacție, steatoză, aleator
    ))
});
// Medial ie after p/b/f/v/m or c/g/ch/gh is a hiatus in these neologism stems and
// endings (igienă, pieton, sovietic, mielită, așchietor, bărbierit; not zbiera, miel, mielușor).
pub static IE_HIATUS: LazyLock<Regex> =
    LazyLock::new(|| re(r"gien|pieton|cviet|societ|soviet|vien|miel(?:o|in|it|ie)|ietoa?r|(?<=[^zs])bier"));
// oa is a hiatus in these neologism stems and endings (coagulat, coaliție, protozoar).
pub static OA_HIATUS: LazyLock<Regex> = LazyLock::new(|| re(r"coagul|coabit|coali[țz]|zoar$"));

/// Where a syllabification comes from, from the weakest to the strongest.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Source {
    Rules,
    Partial,
    Dexonline,
    Manual,
}

impl Source {
    pub fn name(self) -> &'static str {
        match self {
            Source::Rules => "rules",
            Source::Partial => "partial",
            Source::Dexonline => "dexonline",
            Source::Manual => "manual",
        }
    }
}

/// Cuts from a dexonline hyphenation and the boundary span [lo, hi] they settle.
///
/// Uses the first comma-separated variant. It may be complete ('fa-mi-li-e'),
/// a word end ('-li-e'), a word start ('a-ce-tal-') or a word interior
/// ('-pioa-'). None if it does not fit the word (or an interior fits twice).
pub(super) fn dexonline_cuts(word: &[char], hyphenation: &str) -> Option<(HashSet<usize>, usize, usize)> {
    let h = hyphenation.split(',').next().unwrap_or("").trim();
    let (lead, trail) = (h.starts_with('-'), h.ends_with('-'));
    let frag = h.trim_matches('-');
    let letters: Vec<char> = frag.chars().filter(|&c| c != '-').collect();
    if letters.is_empty() {
        return None;
    }
    let start = if lead && trail {
        let found: Vec<usize> = (0..word.len()).filter(|&i| starts_with_at(word, &letters, i)).collect();
        if found.len() != 1 {
            return None;
        }
        found[0]
    } else if lead {
        word.len().checked_sub(letters.len())?
    } else {
        0
    };
    if !starts_with_at(word, &letters, start) || (!lead && !trail && letters != word) {
        return None;
    }
    let end = start + letters.len();
    let parts: Vec<&str> = frag.split('-').collect();
    let (mut cuts, mut k) = (HashSet::new(), start);
    for part in &parts[..parts.len() - 1] {
        k += part.chars().count();
        cuts.insert(k);
    }
    if lead {
        cuts.insert(start);
    }
    if trail {
        cuts.insert(end);
    }
    Some((
        cuts,
        if lead { start } else { start + 1 },
        if trail { end } else { end - 1 },
    ))
}

/// Fallback rules: is there a cut between the vowels word[b-1] and word[b]?
///
/// `known`: normalized words that count as found (for the prefix rule).
/// `verb`: the word is a verb infinitive (for the -ii rule).
/// Incomplete by design, to be refined:
///   - e-e: hiatus (idee -> ide-e; the -ee switch builds on this);
///   - a prefix (PREFIXES) followed by a known word: hiatus (re-abilitat);
///   - -ient, -ienț/-ient + a/ă/e/i: hiatus (client, ambient, experiență);
///   - -iabil, -eabil after a consonant: hiatus (fiabil, agreabil; not debreiabil);
///   - prefix bi- before en: hiatus (bienal);
///   - ea in the EA_HIATUS neologism endings and stems: hiatus (real, nuclear,
///     creat, realizat; not deal, beat, greșeală);
///   - oa in the OA_HIATUS neologism stems and endings: hiatus (coagulat,
///     coaliție, protozoar);
///   - ii word-final after a consonant, in a verb infinitive: hiatus (sfi-i,
///     so sfiește, sfiască keep i-e, i-a; not the plural copii);
///   - ie/ia at word start or after a vowel (ă, î included): diphthong
///     (iarbă, femeie, tăia);
///   - ie/ia word-final after a consonant: hiatus (familie, vie);
///   - ia medial after a consonant: hiatus (apropiat, pian, specializat),
///     except in the IA_DIPHTHONG stems (chiar, biată, piatră) and, like
///     -iere, final -iare after p/b/f/v/m or c/g/ch/gh (fiare);
///   - ie medial after p/b/f/v/m or c/g/ch/gh: diphthong (fier, piept),
///     except the IE_HIATUS stems (igienă, bărbierit, așchietor), the
///     adjectives in -iesc (zapciesc, surugiesc), final -ier/-ieră/-iereasă (IER_ENDINGS) after a
///     vowel and a consonant (plumier, legumieră; not fier, zbier) and the
///     long infinitives in -iere with all their endings (copiere, învierea,
///     aprecierilor, schiere), though not in IERE_DIPHTHONG (miere, mierea,
///     pieri);
///     after other consonants: hiatus (prieten, cartier);
///   - anything else, including ea and oa: no cut (seară, soare).
pub(super) fn rule_cut(word: &[char], b: usize, known: &HashSet<String>, verb: bool) -> bool {
    let pair = [word[b - 1], word[b]];
    if pair == ['e', 'e'] {
        return true;
    }
    let head = text(&word[..b]);
    if word.len() - b >= 4 && PREFIXES.contains(&head.as_str()) && known.contains(&text(&word[b..])) {
        return !(head == "pre" && known.contains(&text(&word[4..])));
    }
    if pair == ['i', 'e'] && found(&IENT, &text(&word[b - 1..])) {
        return true;
    }
    if (pair == ['i', 'a'] || pair == ['e', 'a'])
        && starts_with_at(word, &['a', 'b', 'i', 'l'], b)
        && (b == 1 || !is_vowel(word[b - 2]))
    {
        return true; // fiabil, agreabil; not debreiabil (de-bre-ia)
    }
    if b == 2 && word.starts_with(&['b', 'i', 'e', 'n']) {
        return true;
    }
    if pair == ['e', 'a'] && inside(&EA_HIATUS, word, b) || pair == ['o', 'a'] && inside(&OA_HIATUS, word, b) {
        return true;
    }
    if verb && pair == ['i', 'i'] && b + 1 == word.len() && b >= 2 && !is_vowel(word[b - 2]) {
        return true;
    }
    if !(pair == ['i', 'e'] || pair == ['i', 'a']) || b == 1 || is_vowel(word[b - 2]) {
        return false;
    }
    if b + 1 == word.len() {
        return true;
    }
    let prev = word[b - 2];
    let labial_or_palatal = LABIALS.contains(prev)
        || "cg".contains(prev)
        || b >= 3 && matches!(word[b - 3..b - 1], ['c', 'h'] | ['g', 'h']);
    if pair == ['i', 'a'] {
        if labial_or_palatal && word.ends_with(&['i', 'a', 'r', 'e']) && b == word.len() - 3 {
            return false; // like -iere: fiare, fiere
        }
        return !inside(&IA_DIPHTHONG, word, b);
    }
    if labial_or_palatal {
        if starts_with_at(word, &['i', 'e', 'r'], b - 1) {
            let end = text(&word[b + 2..]);
            if IERE_ENDINGS.contains(&end.as_str()) {
                // copiere, învierea; not mierea, pieri
                return !IERE_DIPHTHONG.contains(&(text(&word[..b + 2]) + "e").as_str());
            }
            if IER_ENDINGS.contains(&end.as_str()) {
                // plumier, legumieră, rachiereasă, rachieresele; not fier, zbier
                return word[..b - 2].iter().any(|&c| is_vowel(c));
            }
        }
        return text(&word[b - 1..]) == "iesc" || inside(&IE_HIATUS, word, b);
    }
    true
}

/// True if a match of `pattern` in `word` spans the boundary b.
pub(super) fn inside(pattern: &Regex, word: &[char], b: usize) -> bool {
    let at: usize = word[..b].iter().map(|c| c.len_utf8()).sum();
    pattern
        .find_iter(&text(word))
        .any(|m| m.map(|m| m.start() < at && at < m.end()).expect("regex match"))
}

/// Syllabified `word` and its source.
///
/// A syllabification in SYLLABIFICATIONS is used as is (its first reading;
/// see syllabify_all). Otherwise, cuts inside the span covered by the
/// dexonline hyphenation are taken from it and prevail. Elsewhere only cuts
/// between two vowels are placed, by the fallback rules; those are the only
/// cuts grafia concisă uses. `known` holds the words the prefix rule may
/// find; `verb` marks a verb infinitive. Input normalized.
pub fn syllabify(word: &str, hyphenation: &str, known: &HashSet<String>, verb: bool) -> (String, Source) {
    if let Some(readings) = syllabifications(word) {
        return (readings[0].to_string(), Source::Manual);
    }
    let w = chars(word);
    let dex = if hyphenation.is_empty() {
        None
    } else {
        dexonline_cuts(&w, hyphenation)
    };
    let source = match &dex {
        None => Source::Rules,
        Some((_, lo, hi)) if *lo <= 1 && hi + 1 >= w.len() => Source::Dexonline,
        Some(_) => Source::Partial,
    };
    let (cuts, lo, hi) = dex.unwrap_or((HashSet::new(), 1, 0));
    let mut out: String = w.iter().take(1).collect();
    for b in 1..w.len() {
        let cut = if lo <= b && b <= hi {
            cuts.contains(&b)
        } else {
            is_vowel(w[b - 1]) && is_vowel(w[b]) && rule_cut(&w, b, known, verb)
        };
        if cut {
            out.push('-');
        }
        out.push(w[b]);
    }
    (out, source)
}

/// Every reading of `word`: all SYLLABIFICATIONS entries, else syllabify's one.
pub fn syllabify_all(word: &str, hyphenation: &str, known: &HashSet<String>, verb: bool) -> Vec<(String, Source)> {
    match syllabifications(word) {
        Some(readings) => readings.iter().map(|s| (s.to_string(), Source::Manual)).collect(),
        None => vec![syllabify(word, hyphenation, known, verb)],
    }
}
