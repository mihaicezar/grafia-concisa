"""grafia concisă — table-driven test suite.

Run: uv run python -m unittest discover -s server
"""
import sqlite3
import tempfile
import unittest
from pathlib import Path

from grafia_concisa.db import build
from grafia_concisa.rules import (
    concise_ee,
    concise_from_reference,
    concise_from_syllables,
    iere_spelling,
    long_infinitive,
    revert_text,
    syllabify,
)
from server import create_app

# (syllabified, expected concise). Add a case = add a line.
WORDS = [
    # diphthongs, stressed
    ("soa-re", "sōre"),
    ("sea-ră", "sēră"),
    ("mea", "mē"),
    ("fier", "fẹr"),
    ("iar-bă", "ėrbă"),
    ("ceai", "cēi"),
    # diphthongs, unstressed
    ("cea-sor-nic", "cēsornic"),
    ("ia-urt", "ėurt"),
    ("ie-pu-raș", "ẹpuraș"),
    ("foar-fe-cu-ță", "fōrfecuță"),
    ("cea-i-nă-ri-e", "cēinărie"),
    # hiatus: unchanged
    ("fi-e-ca-re", "fiecare"),
    ("re-al", "real"),
    ("te-a-tru", "teatru"),
    ("pi-an", "pian"),
    ("bo-a", "boa"),
    ("vi-e", "vie"),
    ("co-a-li-ți-e", "coaliție"),
    # hiatus + diphthong in one word
    ("a-ce-ea", "aceē"),
    # triphthongs: left to right, no overlap
    ("leoar-că", "leōrcă"),
    ("a-ri-pioa-ră", "aripiōră"),
    ("beai", "bēi"),
    ("iei", "ẹi"),
    ("ploa-ie", "plōẹ"),
    ("șoa-re-ce", "șōrece"),
    ("ie-pu-re", "ẹpure"),
    ("pâi-nea", "pâinē"),  # â is the â -> î switch's job, applied after the lookup
]

# Forms without their own syllabification: cuts come from the lemma.
# (form, lemma syllabified, expected)
DERIVED = [
    ("soarele", "soa-re", "sōrele"),      # shared prefix: lemma decides
    ("reală", "re-al", "reală"),
    ("familia", "fa-mi-li-e", "familia"),  # ending replaces a vowel: lemma decides
    ("familiei", "fa-mi-li-e", "familiei"),
    ("femeia", "fe-me-ie", "femeė"),
    ("ploaia", "ploa-ie", "plōė"),
    ("ideea", "i-de-e", "ideē"),           # appended ending: diphthong
    ("creează", "cre-a", "creēză"),
    ("înșeală", "în-șe-la", "înșēlă"),     # stem alternation: diphthong
    ("doarme", "dor-mi", "dōrme"),
    ("ploaie", "plo-ua", "plōẹ"),           # o/e before a replaced vowel: diphthong
    ("scheaună", "sche-u-na", "schēună"),
    ("taie", "tă-ia", "taẹ"),
    ("argintie", "ar-gin-tiu", "argintie"),  # -iu lemma: hiatus
    ("cheflia", "che-fliu", "cheflia"),
    ("sfiește", "sfi-i", "sfiește"),       # -ii verb: lemma decides
    ("sfiască", "sfi-i", "sfiėscă"),       # -ează/-ească after a hiatus i: -ėză, -ėscă
    ("priască", "pri-i", "priėscă"),
    ("studiază", "stu-di-a", "studiėză"),
    ("pliază", "pli-a", "pliėză"),
    ("remaniază", "re-ma-ni-a", "remaniėză"),
    ("fiască", "fi-esc", "fiėscă"),
    ("fiasca", "fi-esc", "fiėsca"),
]

# -ee rule: (form, lemma syllabified, number, expected with switch)
EE_WORDS = [
    ("idee", "i-de-e", "S", "ideẹ"),
    ("alee", "a-le-e", "S", "aleẹ"),
    ("bombee", "bom-beu", "P", "bombeẹ"),
    ("bombeele", "bom-beu", "P", "bombeẹle"),
    ("bombeelor", "bom-beu", "P", "bombeẹlor"),
    ("liceele", "li-ceu", "P", "liceẹle"),
    ("liceelor", "li-ceu", "P", "liceẹlor"),
    ("bombeuri", "bom-beu", "P", "bombeẹ"),         # -eu lemma > 4 letters
    ("bombeurile", "bom-beu", "P", "bombeẹle"),
    ("liceurilor", "li-ceu", "P", "liceẹlor"),
    ("pneurile", "pneu", "P", "pneurile"),          # pneu: 4 letters
    ("pneuri", "pneu", "P", "pneuri"),
    ("logoree", "lo-go-re-e", "P", "logorei"),  # plural read first...
    ("logoree", "lo-go-re-e", "S", "logoreẹ"),  # ...singular wins
    ("femeie", "fe-me-ie", "S", "femeẹ"),       # base rules already
    ("ideea", "i-de-e", "S", "ideė"),           # like femeia
    ("ideei", "i-de-e", "S", "ideii"),          # like femeii
    ("orhideea", "or-hi-de-e", "S", "orhideė"),
    ("niceea", "ni-ce-ea", "S", "niceė"),          # headword in -eea
    ("andreea", "an-dre-ea", "S", "andreė"),
    ("aceea", "a-ce-ea", "S", "aceē"),             # ...except the pronoun
    ("spontaneea", "spon-ta-neu", "S", "spontaneė"),  # -eu adjective
    ("galileei", "ga-li-le-ea", "S", "galileii"),  # -eei: same lemmas as -eea
    ("spontaneei", "spon-ta-neu", "S", "spontaneii"),
    ("plebeea", "ple-beu", "S", "plebeė"),
    ("orhideei", "or-hi-de-e", "S", "orhideii"),
    ("orhideele", "or-hi-de-e", "P", "orhideile"),    # like femeile
    ("orhideelor", "or-hi-de-e", "P", "orhideilor"),  # like femeilor
]

# (word, dexonline hyphenation, expected syllabified, expected source).
# Outside dexonline's span, rules place only the cuts between vowels.
SYLLABIFY = [
    ("familie", "fa-mi-li-e", "fa-mi-li-e", "dexonline"),
    ("familie", "-li-e", "fami-li-e", "partial"),           # -x: word end
    ("acetaldehidă", "a-ce-tal-", "a-ce-tal-dehidă", "partial"),  # x-: word start
    ("aripioară", "-pioa-", "ari-pioa-ră", "partial"),       # -x-: word interior
    ("biologie", "-lo-gi-e, -log-i-e", "bio-lo-gi-e", "partial"),  # first variant
    ("soare", "-xy-", "soare", "rules"),                    # does not fit: ignored
    ("soare", "", "soare", "rules"),                        # oa, ea: diphthong
    ("seară", "", "seară", "rules"),
    ("iarbă", "", "iarbă", "rules"),                        # ie/ia at word start
    ("femeie", "", "femeie", "rules"),                      # ie/ia after a vowel
    ("familie", "", "famili-e", "rules"),                   # final, after a consonant
    ("vie", "", "vi-e", "rules"),
    ("fier", "", "fier", "rules"),                          # medial, after p/b/f/v/m
    ("piept", "", "piept", "rules"),
    ("client", "", "cli-ent", "rules"),                     # medial, other consonant
    ("social", "", "soci-al", "rules"),
    ("idee", "", "ide-e", "rules"),                         # e-e: hiatus
    ("aceea", "", "ace-ea", "rules"),
    ("ambient", "", "ambi-ent", "rules"),                   # -ient, -ienț...: hiatus
    ("experiență", "", "experi-ență", "rules"),
    ("clientelă", "", "cli-entelă", "rules"),
    ("farniente", "far-nien-te", "far-nien-te", "dexonline"),  # dexonline prevails
    ("sfii", "", "sfi-i", "rules", "V"),                    # final -ii in a verb: hiatus
    ("prii", "", "pri-i", "rules", "V"),
    ("copii", "", "copii", "rules"),                        # ...not in other words
    ("bienal", "", "bi-enal", "rules"),                     # prefix bi-
    ("real", "", "re-al", "rules"),                         # ea: EA_HIATUS neologisms
    ("boreal", "", "bore-al", "rules"),
    ("leal", "", "le-al", "rules"),
    ("linear", "", "line-ar", "rules"),
    ("nuclear", "", "nucle-ar", "rules"),
    ("alineat", "", "aline-at", "rules"),
    ("realizat", "", "re-alizat", "rules"),
    ("cereale", "", "cere-ale", "rules"),
    ("denuclearizat", "", "denucle-arizat", "rules"),
    ("pancreatic", "", "pancre-atic", "rules"),
    ("reactor", "", "re-actor", "rules"),
    ("aleator", "", "ale-ator", "rules"),
    ("deal", "", "deal", "rules"),                          # ...not native words
    ("podbeal", "", "podbeal", "rules"),
    ("beat", "", "beat", "rules"),
    ("asear", "", "asear", "rules"),
    ("dogoreală", "", "dogoreală", "rules"),
    ("bianual", "", "bi-anual", "rules"),
    ("coagulat", "", "co-agulat", "rules"),                 # oa: OA_HIATUS neologisms
    ("protozoar", "", "protozo-ar", "rules"),
    ("mozoară", "", "mozoară", "rules"),
    ("biet", "", "biet", "rules"),
    ("biată", "", "biată", "rules"),                        # IA_DIPHTHONG stem
    ("fiabil", "", "fi-abil", "rules"),                     # -iabil, -eabil: hiatus
    ("agreabil", "", "agre-abil", "rules"),
    ("copiere", "", "copi-ere", "rules"),                   # final -iere: hiatus...
    ("veghiere", "", "veghi-ere", "rules"),
    ("miere", "", "miere", "rules"),                        # ...except IERE_DIPHTHONG
    ("înmiere", "", "înmiere", "rules"),
    ("xerocopiere", "", "xerocopi-ere", "rules"),
    ("învierea", "", "învi-erea", "rules"),                 # articulated: -ierea
    ("mierea", "", "mierea", "rules"),
    ("copierilor", "", "copi-erilor", "rules"),             # ...and all its endings
    ("pieri", "", "pieri", "rules"),                        # (piere: IERE_DIPHTHONG)
    ("schierea", "", "schi-erea", "rules"),
    ("tăiere", "", "tăiere", "rules"),                      # vowel + -iere: diphthong
    ("înmuiere", "", "înmuiere", "rules"),
    ("bărbier", "", "bărbi-er", "rules"),                   # except final -ier/-ieră after
    ("plumier", "", "plumi-er", "rules"),                   # a vowel and a consonant...
    ("legumieră", "", "legumi-eră", "rules"),
    ("zbier", "", "zbier", "rules"),                        # (not fier, zbier)
    ("bărbierit", "", "bărbi-erit", "rules"),               # ...IE_HIATUS stems...
    ("zbieret", "", "zbieret", "rules"),
    ("igienă", "", "igi-enă", "rules"),
    ("mielită", "", "mi-elită", "rules"),
    ("miel", "", "miel", "rules"),
    ("așchietor", "", "așchi-etor", "rules"),
    ("sovietism", "", "sovi-etism", "rules"),
    ("zapciesc", "", "zapci-esc", "rules"),                 # ...and adjectives in -iesc
    ("surugiesc", "", "surugi-esc", "rules"),
    ("ciocoiesc", "", "ciocoiesc", "rules"),                # (after a vowel: diphthong)
    ("apropiat", "", "apropi-at", "rules"),                 # medial ia after a consonant:
    ("pian", "", "pi-an", "rules"),                         # hiatus (-ia* endings)
    ("deviant", "", "devi-ant", "rules"),
    ("fiare", "", "fiare", "rules"),                        # final -iare after p/b/f/v/m, c/g
    ("variare", "", "vari-are", "rules"),                   # ...but not after other consonants
    ("specializat", "", "speci-alizat", "rules"),
    ("îmbiat", "", "îmbi-at", "rules"),
    ("biatlon", "", "bi-atlon", "rules"),
    ("miazmă", "", "mi-azmă", "rules"),
    ("chiar", "", "chiar", "rules"),                        # ...except IA_DIPHTHONG stems
    ("piatră", "", "piatră", "rules"),
    ("amiază", "", "amiază", "rules"),
    ("maghiar", "", "maghiar", "rules"),
    ("chiajna", "", "chiajna", "rules"),
    ("obială", "", "obială", "rules"),
    ("microbial", "", "microbi-al", "rules"),
    ("schiagramă", "", "schi-agramă", "rules"),
    ("liturghiar", "", "liturghi-ar", "rules"),
    ("eleghiacesc", "", "eleghi-acesc", "rules"),
    ("miază", "mi-a-ză", "mia-ză", "manual"),
    ("cartier", "", "carti-er", "rules"),                   # -tier: after t anyway
    ("reabilitat", "", "re-abilitat", "rules"),             # prefix + known word
    ("autoanaliză", "", "auto-analiză", "rules"),
    ("neam", "", "neam", "rules"),                          # rest too short
    ("autoare", "", "autoare", "rules"),
    ("preacurat", "", "preacurat", "rules"),                # prea- + curat, not pre- + acurat
    ("preaviz", "", "pre-aviz", "rules"),
    ("reazem", "", "reazem", "rules"),                      # rest not a known word
    ("coace", "", "coace", "rules"),                        # rest too short
    ("coactiv", "", "co-activ", "rules"),                   # co- prefix
    ("neant", "", "ne-ant", "manual"),                      # SYLLABIFICATIONS beat all
    ("mielea", "", "mi-e-lea", "manual"),
    ("răspieri", "", "răs-pie-ri", "manual"),
    ("coastă", "", "coas-tă", "manual"),
    ("coastă", "co-as-tă", "coas-tă", "manual"),
]

# Homographs: extra records for the test DB (form, lemma, hyphenation, number[, V])...
HOMOGRAPH_RECORDS = [
    ("ia", "ia", "", "S"),                 # interjection
    ("ia", "lua", "lu-a", "S"),            # verb: el ia
    ("ia", "ie", "", "S"),                 # articulated ie (SYLLABIFICATIONS: i-e, ie)
    ("ie", "ie", "", "S"),
    ("miază", "miază", "", "S"),           # SYLLABIFICATIONS: mia-ză, mi-a-ză
    ("trepied", "trepied", "", "S"),       # rules alone: trepẹd...
    ("trepied", "trepied", "tre-pi-ed", "S"),  # ...dexonline: hiatus
    ("liază", "liază", "li-a-ză", "S"),    # noun...
    ("liază", "lia", "", "S", "V"),        # ...or verb: -ėză, both spellings count
    ("piei", "piei", "", "S"),             # SYLLABIFICATIONS: piei...
    ("piei", "pia", "pi-a", "S"),          # ...so no other lemma adds a spelling
    ("vier", "vier", "", "S"),             # SYLLABIFICATIONS: vier, vi-er
    ("vierul", "vier", "", "S"),
    ("agreare", "agreare", "", "S"),       # a headword by rules alone...
    ("agreare", "agrea", "a-gre-a", "S", "V"),  # ...loses to a backed derived form
]
# ...and (word, all spellings, default first). No alternatives: [].
HOMOGRAPHS = [
    ("ia", ["ė", "ia"]),
    ("ie", ["ie", "ẹ"]),
    ("Ia", ["Ė", "Ia"]),
    ("miază", ["mėză", "miază"]),          # noon (default), myiasis
    ("trepied", []),                       # a rules-only spelling is no alternative...
    ("liază", ["liază", "liėză"]),         # ...except one with -ėză/-ėscă
    ("piei", []),                          # SYLLABIFICATIONS: the form's only spelling
    ("vier", ["vẹr", "vier"]),             # the boar (default), păzitor de vie
    ("vierul", ["vẹrul", "vierul"]),
    ("agreare", []),                       # the rules-only agrēre is no alternative
]
# Long infinitives in -iare (flag F) are written -iere: records for the test DB.
LONG_INFINITIVE_RECORDS = [
    ("apreciarea", "apreciare", "a-pre-ci-a-re", "S", "F"),
    ("apreciărilor", "apreciare", "a-pre-ci-a-re", "P", "F"),
    ("tăiarea", "tăiare", "", "S", "F"),
    ("fiare", "fiare", "", "S", "F"),      # rules.IARE_KEPT: no long infinitive
    ("variare", "variare", "", "S"),       # no flag F: no long infinitive
    ("conciliare", "conciliare", "", "S", "F"),  # loses to any other record...
    ("conciliare", "conciliar", "", "P"),        # ...the adjective
]

# Unknown words with the -ee switch: (word, all spellings, default first).
UNKNOWN_EE = [
    ("zzeele", ["zzeile", "zzeẹle"]),      # plural of zzee (default) or of zzeu
    ("Zzeelor", ["Zzeilor", "Zzeẹlor"]),
    ("zzee", ["zzeẹ"]),                    # read as a singular
    ("Andreei", ["Andreii"]),              # genitive of the known andreea
    ("aleei", ["aleii"]),                  # no aleea, but alee is known
    ("aceei", ["aceei"]),                  # aceea is known, but keeps -eea
    ("plebeei", ["plebeei", "plebeii"]),   # plebeea is a form, not a lemma
    ("Zzeei", ["Zzeei", "Zzeii"]),         # no known lemma: unchanged (default) or -eii
    ("muzeele", ["muzeẹle"]),              # plural of a known -eu lemma: one spelling
    ("Muzeelor", ["Muzeẹlor"]),
    ("muzeurile", ["muzeẹle"]),            # known -eu lemma
    ("Muzeuri", ["Muzeẹ"]),
    ("zzeurilor", ["zzeurilor"]),          # no known lemma: unchanged
]
# Reverse -ee: extra records for the test DB (form, lemma, hyphenation, number, V, D = DEX-MDA-DOOM).
EE_REVERT_RECORDS = [
    ("alee", "alee", "", "S", "", "D"),
    ("aleie", "aleie", "", "S"),                      # no DEX-MDA-DOOM: aleẹ -> alee
    ("cheie", "cheie", "", "S", "", "D"),
    ("chee", "chee", "", "S"),                        # no DEX-MDA-DOOM: cheẹ -> cheie
    ("apogee", "apogeu", "a-po-geu", "P", "", "D"),   # both DEX-MDA-DOOM: -ee beats -euri
    ("apogeuri", "apogeu", "a-po-geu", "P", "", "D"),
    ("caree", "careu", "ca-reu", "P"),                # only -euri DEX-MDA-DOOM
    ("careuri", "careu", "ca-reu", "P", "", "D"),
    ("bordee", "bordeu", "bor-deu", "P", "", "D"),    # both DEX-MDA-DOOM, neither in -euri: a tie
    ("bordeie", "bordei", "bor-dei", "P", "", "D"),
    ("zeii", "zeu", "zeu", "P", "", "D"),             # no candidate, but spelled like zeei's -ee
    ("zeei", "zee", "ze-e", "S"),                     # spelling: zeii (DEX-MDA-DOOM) wins
    ("pigmeei", "pigmee", "pig-me-e", "S", "D"),      # flags in any order: D alone
    ("pigmeii", "pigmeu", "pig-meu", "P", "D"),
    ("rarelee", "rareleu", "ra-re-leu", "P"),         # neither DEX-MDA-DOOM: -ee and -euri tie
    ("rareleuri", "rareleu", "ra-re-leu", "P"),
]
# ...and (concise word, all forms it reverts to with the -ee switch, default first). One form: [].
REVERT_HOMOGRAPHS = [
    ("bordeẹ", ["bordeie", "bordee"]),     # default: the character map's
    ("Bordeẹ", ["Bordeie", "Bordee"]),
    ("aleẹ", []),                          # decided: alee
    ("pigmeii", ["pigmeii", "pigmeei"]),   # both DEX-MDA-DOOM, pigmeii unchanged: a tie
    ("rareleẹ", ["rarelee", "rareleuri"]),  # -ee over -euri only among DEX-MDA-DOOM forms
]
# Lemmas for the test DB with no forms of their own: -eu (no -eele/-eelor), -iesc.
LEMMAS = ["mu-zeu", "ha-ra-ba-gi-esc"]

# Words the prefix rule may find (the pipeline passes every dexonline form).
KNOWN = {"abilitat", "analiză", "am", "ace", "are", "acurat", "curat", "aviz", "activ", "astă"}

# (form, concise, lemma, plural, expected) for the rule function itself
EE_RULE = [
    ("idee", "idee", "idee", False, "ideẹ"),
    ("bombee", "bombee", "bombeu", True, "bombeẹ"),
    ("bombeele", "bombeele", "bombeu", True, "bombeẹle"),
    ("bombeelor", "bombeelor", "bombeu", True, "bombeẹlor"),
    ("bombeurilor", "bombeurilor", "bombeu", True, "bombeẹlor"),
    ("pneurilor", "pneurilor", "pneu", True, None),
    ("bombeuri", "bombeuri", "bombeu", True, "bombeẹ"),
    ("piureuri", "piureuri", "piure", True, None),     # lemma in -e, not -eu
    ("liceul", "liceul", "liceu", False, None),  # -eu lemma: only -ee/-eele/-eelor
    ("logoree", "logoree", "logoree", True, "logorei"),
    ("ideea", "ideē", "idee", False, "ideė"),
    ("ideei", "ideei", "idee", False, "ideii"),
    ("orhideele", "orhideele", "orhidee", True, "orhideile"),
    ("orhideelor", "orhideelor", "orhidee", True, "orhideilor"),
    ("xee", "xee", "xa", True, None),           # plural of another lemma
    ("aceea", "aceē", "acela", False, None),    # -eea of another lemma
    ("aceea", "aceē", "aceea", False, None),    # the pronoun as a headword
    ("niceea", "niceē", "niceea", False, "niceė"),
    ("spontaneea", "spontaneē", "spontaneu", False, "spontaneė"),
    ("galileei", "galileei", "galileea", False, "galileii"),
    ("spontaneei", "spontaneei", "spontaneu", False, "spontaneii"),
    ("zzeei", "zzeei", "zzea", False, None),
    ("femeie", "femeẹ", "femeie", False, None),
]

# (input text, ee switch, expected output text, expected unknown words[, ai switch];
# without it, ai is left to its default, on)
TEXTS = [
    ("Soare", False, "Sōre", []),
    ("SOARE", False, "SŌRE", []),
    ("Iepure, fier și ceai.", False, "Ẹpure, fẹr și cēi.", []),
    ("Real teatru", False, "Real teatru", []),
    ("harabagiasca harabagiască harabagiești", False,     # unknown forms of a known -iesc lemma
     "harabagiėsca harabagiėscă harabagiești", ["harabagiasca", "harabagiască", "harabagiești"]),
    ("zzgiasca", False, "zzgiasca", ["zzgiasca"]),          # no known lemma: rules alone
    ("şoarece", False, "șōrece", []),  # cedilla input
    ("casa mare", False, "casa mare", []),  # no candidate: never looked up
    ("zzoa soare", False, "zzō sōre", ["zzoa"]),  # unknown: rules alone
    ("Zzoa zzia", False, "Zzō zzia", ["Zzoa", "zzia"]),
    ("", False, "", []),
    ("O idee, o alee, bombee.", False, "O idee, o alee, bombee.", []),
    ("O idee, o alee, bombee.", True, "O ideẹ, o aleẹ, bombeẹ.", []),
    ("Logoree, IDEE, femeie", True, "Logoreẹ, IDEẸ, femeẹ", []),
    ("Ideea orhideei", False, "Ideē orhideei", []),
    ("Ideea orhideei, ideii", True, "Ideė orhideii, ideii", []),
    ("Orhideele orhideelor", False, "Orhideele orhideelor", []),
    ("Orhideele orhideelor", True, "Orhideile orhideilor", []),
    ("Liceele liceelor", False, "Liceele liceelor", []),
    ("Liceele liceelor", True, "Liceẹle liceẹlor", []),
    ("Bombeurile liceurilor, pneurile", False, "Bombeurile liceurilor, pneurile", []),
    ("Bombeurile liceurilor, pneurile", True, "Bombeẹle liceẹlor, pneurile", []),
    ("Bombeuri, pneuri", False, "Bombeuri, pneuri", []),
    ("Bombeuri, pneuri", True, "Bombeẹ, pneuri", []),
    ("Niceea, Andreea, spontaneea, aceea", False, "Niceē, Andreē, spontaneē, aceē", []),
    ("Niceea, Andreea, spontaneea, aceea", True, "Niceė, Andreė, spontaneė, aceē", []),
    ("Galileei, spontaneei", True, "Galileii, spontaneii", []),
    ("zzee soare", True, "zzeẹ sōre", ["zzee"]),
    ("zzee", False, "zzee", []),  # -ee not a candidate with the switch off
    ("zzeele zzeelor", True, "zzeile zzeilor", ["zzeele", "zzeelor"]),
    ("Câinele mâncă, CÂND?", False, "Cîinele mîncă, CÎND?", []),  # â -> î, on by default
    ("Pâinea și soarele", False, "Pîinē și sōrele", []),
    ("zzâia", False, "zzîė", ["zzâia"]),
    ("zziăr", False, "zziăr", []),                                 # iăr alone: nothing to change                           # unknown words too
    ("Apreciarea apreciărilor, tăiarea", False, "Aprecierē aprecierilor, tăẹrē", []),
    ("fiare, variare, piei, conciliare", False, "fėre, variare, pẹi, conciliare", []),
    ("agreare", False, "agreare", []),                             # backed beats rules alone
    ("Român, ROMÂNESC, aromână, dacoromân, rumân, rumânesc", False,  # root român/rumân: â kept
     "Român, ROMÂNESC, aromână, dacoromân, rumân, rumânesc", []),
    ("Câinele, pâinea", False, "Câinele, pâinē", [], False),       # switch off
    ("Câinele, pâinea", False, "Cîinele, pîinē", [], True),
]

# (concise text, expected standard text[, -ee switch])
REVERTS = [
    ("Ẹpure, fẹr și cēi.", "Iepure, fier și ceai."),
    ("SŌRE", "SOARE"),
    ("Ėrbă", "Iarbă"),
    ("studiėză, SFIĖSCĂ, fiėsca", "studiază, SFIASCĂ, fiasca"),
    ("Cîinele mîncă pîinē", "Cîinele mîncă pîinea"),  # î is left as is
    ("aleẹ, orhideile", "aleie, orhideile"),            # -ee switch off: character map
    ("Ideẹ, orhideile, ORHIDEĖ, orhideii", "Idee, orhideele, ORHIDEEA, orhideei", True),
    ("aleẹ, cheẹ", "alee, cheie", True),                # the DEX-MDA-DOOM form
    ("apogeẹ, careẹ", "apogee, careuri", True),         # -ee over -euri, if both DEX-MDA-DOOM
    ("bombeẹle, liceẹlor", "bombeele, liceelor", True),
    ("femeẹ, zzeẹ, studiėză", "femeie, zzeie, studiază", True),  # not in ee_revert, unknown
    ("Zeii", "Zeii", True),                             # a form with no digraph beats zeei
]


class Suite(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.tmp = tempfile.TemporaryDirectory(ignore_cleanup_errors=True)  # Windows: DB still open
        cls.db = Path(cls.tmp.name) / "word_db.sqlite"
        build(
            cls.db,
            [s for s, _ in WORDS]
            + [(f, lemma.replace("-", ""), lemma, "S") for f, lemma, _ in DERIVED]
            + [(f, lemma.replace("-", ""), lemma, n) for f, lemma, n, _ in EE_WORDS]
            + HOMOGRAPH_RECORDS
            + EE_REVERT_RECORDS
            + LONG_INFINITIVE_RECORDS
            + LEMMAS,
        )
        cls.client = create_app(cls.db).test_client()

    @classmethod
    def tearDownClass(cls):
        cls.tmp.cleanup()

    def post(self, path, payload):
        return self.client.post(path, json=payload)

    def test_rules(self):
        for syl, expected in WORDS:
            with self.subTest(syl):
                self.assertEqual(concise_from_syllables(syl), expected)

    def test_derived(self):
        for form, lemma, expected in DERIVED:
            with self.subTest(form):
                self.assertEqual(concise_from_reference(form, lemma), expected)

    def test_syllabify(self):
        for word, hyphenation, expected, source, *pos in SYLLABIFY:
            with self.subTest(word=word, hyphenation=hyphenation):
                self.assertEqual(syllabify(word, hyphenation, KNOWN, pos == ["V"]), (expected, source))

    def test_ee_rule(self):
        for form, concise, lemma, plural, expected in EE_RULE:
            with self.subTest(concise):
                self.assertEqual(concise_ee(form, concise, lemma, plural), expected)

    def test_ee_db(self):
        with sqlite3.connect(self.db) as c:
            rows = dict(c.execute("SELECT form, COALESCE(concise_ee, concise_form) FROM forms"))
        expected: dict[str, str] = {}
        for form, _, number, exp in EE_WORDS:  # singular reading wins
            if number == "S" or form not in expected:
                expected[form] = exp
        for form, exp in expected.items():
            with self.subTest(form):
                self.assertEqual(rows[form], exp)

    def test_convert(self):
        for text, ee, expected, unknown, *ai in TEXTS:
            with self.subTest(text=text, ee=ee, ai=ai):
                payload = {"text": text, "ee": ee} | ({"ai": ai[0]} if ai else {})
                r = self.post("/api/convert", payload).get_json()
                self.assertEqual(r["text"], expected)
                self.assertEqual(r["unknown"], unknown)

    def test_revert(self):
        for text, expected, *ee in REVERTS:
            with self.subTest(text=text, ee=ee):
                payload = {"text": text} | ({"ee": ee[0]} if ee else {})
                self.assertEqual(self.post("/api/revert", payload).get_json()["text"], expected)

    def test_round_trip_whole_db(self):
        # base rules only: the -ee rule is not reversible by character map
        with sqlite3.connect(self.db) as c:
            rows = c.execute("SELECT form, concise_form FROM forms").fetchall()
        self.assertTrue(rows)
        # a form of a long infinitive in -iare may revert to its -iere spelling
        long_inf = {r[0] for r in LONG_INFINITIVE_RECORDS if "F" in r[4:] and long_infinitive(r[1])}
        for form, concise in rows:
            with self.subTest(form):
                allowed = (form, iere_spelling(form)) if form in long_inf else (form,)
                self.assertIn(revert_text(concise), allowed)

    def test_lookup(self):
        r = self.post("/api/lookup", {"words": ["Soare", "casa", "zzoa", "Câine", "român"]}).get_json()
        self.assertEqual(r, {"results": {"Soare": "sōre", "casa": "casa", "zzoa": "zzō",
                                         "Câine": "cîine", "român": "român"},
                             "unknown": ["zzoa"], "alternatives": {}})
        r = self.post("/api/lookup", {"words": ["pâinea"], "ai": False}).get_json()
        self.assertEqual(r["results"], {"pâinea": "pâinē"})
        r = self.post("/api/lookup", {"words": ["idee", "zzee", "zzeele", "muzeele"],
                                      "ee": True}).get_json()
        self.assertEqual(r, {"results": {"idee": "ideẹ", "zzee": "zzeẹ", "zzeele": "zzeile",
                                         "muzeele": "muzeẹle"},
                             "unknown": ["zzee", "zzeele", "muzeele"],
                             "alternatives": {"zzeele": ["zzeile", "zzeẹle"]}})

    def test_homographs(self):
        for word, spellings in HOMOGRAPHS:
            with self.subTest(word):
                segments = self.post("/api/convert", {"text": word}).get_json()["segments"]
                self.assertEqual(segments[0].get("alts", []), spellings)
                if spellings:
                    self.assertEqual(segments[0]["t"], spellings[0])  # default in the text

    def test_revert_homographs(self):
        for word, spellings in REVERT_HOMOGRAPHS:
            with self.subTest(word):
                segments = self.post("/api/revert", {"text": word, "ee": True}).get_json()["segments"]
                self.assertEqual(segments[0].get("alts", []), spellings)
                if spellings:
                    self.assertEqual(segments[0]["t"], spellings[0])  # default in the text

    def test_unknown_ee(self):
        for word, spellings in UNKNOWN_EE:
            with self.subTest(word):
                segments = self.post("/api/convert", {"text": word, "ee": True}).get_json()["segments"]
                self.assertEqual(segments[0]["unknown"], word)
                self.assertEqual(segments[0]["t"], spellings[0])
                self.assertEqual(segments[0].get("alts", spellings[:1]), spellings)

    def test_bad_requests(self):
        for path, payload in [
            ("/api/convert", {}),
            ("/api/convert", {"text": 1}),
            ("/api/revert", ["x"]),
            ("/api/lookup", {"words": "soare"}),
            ("/api/convert", {"text": "a" * 100_001}),
            ("/api/convert", {"text": "idee", "ee": "yes"}),
            ("/api/lookup", {"words": [], "ai": 1}),
        ]:
            with self.subTest(path=path, payload=str(payload)[:30]):
                self.assertEqual(self.post(path, payload).status_code, 400)

    def test_health_and_index(self):
        self.assertEqual(self.client.get("/healthz").status_code, 200)
        with self.client.get("/") as r:
            self.assertEqual(r.status_code, 200)


if __name__ == "__main__":
    unittest.main()
