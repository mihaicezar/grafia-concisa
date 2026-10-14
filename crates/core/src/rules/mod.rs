//! Orthographic rules of grafia concisă.
//!
//! A digraph ie/ia/ea/oa is compressed when both vowels are in the same
//! syllable (no hiatus). Within a syllable the scan is left to right and
//! matches never overlap.
//!
//! Optional rule "ee", -eẹ/aẹ/oẹ (switch): -ee lemmas follow the femeie pattern.
//! ```text
//!   - final -ee, singular:             idee    -> ideẹ
//!   - final -ee, plural of -eu lemma:  bombee  -> bombeẹ
//!   - final -eele/-eelor, -eu lemma:   bombeele -> bombeẹle, bombeelor -> bombeẹlor
//!   - final -euri/-eurile/-eurilor, -eu lemma of more than 4 letters:
//!                                      bombeuri -> bombeẹ, bombeurile -> bombeẹle,
//!                                      bombeurilor -> bombeẹlor (not pneuri)
//!   - final -ee, plural of -ee lemma:  logoree -> logorei
//!   - final -eea, -ee or -eu lemma:    ideea   -> ideė     (femeia), spontaneea -> spontaneė
//!   - final -eea, -eea headword:       Niceea  -> Niceė    (not aceea, oareceea)
//!   - final -eei, same lemmas as -eea: ideei   -> ideii    (femeii), Galileei -> Galileii
//!   - final -eele, -ee lemma:          orhideele  -> orhideile  (femeile)
//!   - final -eelor, -ee lemma:         orhideelor -> orhideilor (femeilor)
//!   - a kept lemma (in -eu, in -ee with a masculine, or in NAMES) keeps its e:
//!                                      zeei -> zeẹi, zeele -> zeẹle, zee (pl.) -> zeẹ,
//!                                      spontaneei -> spontaneẹi, Andreei -> Andreẹi
//!   - final -oae, any word:            Băboae -> Băbōẹ
//!   - a lemma in AE_OE:                Nae -> Naẹ, aloe -> aloẹ, aloea -> aloė, aloei -> aloii,
//!                                      aloele -> aloile; a name: Zoei -> Zoẹi, Zoele -> Zoẹle
//! ```
//! When one spelling is both a singular and a plural, the singular wins.
//!
//! Optional rule "ai" (switch, on by default): â is written î, except in words
//! with the root român/rumân (câine -> cîine; român, aromân, rumânesc unchanged).
//!
//! Long infinitives in -iare are not standard: a form of one is written as the
//! -iere form, with its cuts (apreciare -> apreciere, apreciărilor -> aprecierilor).
//!
//! Apart from the -ee rule, a form with two syllabifications has two spellings
//! (a homograph: ia = the verb lua or the articulated ie). One is the default,
//! the other an alternative the user can switch to.
//!
//! Words are indexed by character (a `Vec<char>`), not by byte; the code was
//! ported from Python, whose semantics some comments still cite.
//!
//! The rules are split by concern; every item is re-exported here, so callers use `rules::X`.
//! The files use each other's items through `use super::*`.
//!
//! - `words`: regexes, normalization, case, â -> î;
//! - `lists`: the hand lists (`SYLLABIFICATIONS`, `NAMES`, `AE_OE`, `OBSOLETE`, ...);
//! - `syllables`: the syllabification (dexonline's cuts, the fallback rules);
//! - `spelling`: the concise spelling of a form from its lemma's cuts;
//! - `obsolete_forms`: obsolete lemmas and long infinitives in -iare;
//! - `ee`: the -ee switch and the lemmas of unknown words;
//! - `convert`: standard -> concise over a text;
//! - `revert`: concise -> standard.
mod convert;
mod ee;
mod lists;
mod obsolete_forms;
mod revert;
mod spelling;
mod syllables;
mod words;

pub use convert::*;
pub use ee::*;
pub use lists::*;
pub use obsolete_forms::*;
pub use revert::*;
pub use spelling::*;
pub use syllables::*;
pub use words::*;
