"""Build a small development word_db.sqlite without MySQL.

Usage: uv run python pipeline/seed_db.py [--out PATH]
"""
import argparse
from pathlib import Path

from grafia_concisa.db import build

ROOT = Path(__file__).resolve().parent.parent

SEED = """
soa-re sea-ră sea-ra mea ta-ta ie-pu-re ie-pu-raș fier
ceai cea-sor-nic cea-i-nă-ri-e ia-urt iar-bă foar-fe-cu-ță doam-nă
leoar-că a-ri-pioa-ră beai iei ploa-ie șoa-re-ce
a-ce-ea
fi-e-ca-re re-al te-a-tru pi-an bo-a co-a-li-ți-e vi-e
""".split() + [
    # (form, lemma, lemma hyphenation: complete, partial or "", number)
    ("idee", "idee", "i-de-e", "S"),
    ("ideea", "idee", "i-de-e", "S"),
    ("ideei", "idee", "i-de-e", "S"),
    ("alee", "alee", "a-le-e", "S"),
    ("bombee", "bombeu", "bom-beu", "P"),
    ("bombeele", "bombeu", "bom-beu", "P"),
    ("bombeelor", "bombeu", "bom-beu", "P"),
    ("logoree", "logoree", "lo-go-re-e", "S"),
    ("orhidee", "orhidee", "or-hi-de-e", "S"),
    ("orhideea", "orhidee", "or-hi-de-e", "S"),
    ("orhideei", "orhidee", "or-hi-de-e", "S"),
    ("orhideele", "orhidee", "or-hi-de-e", "P"),
    ("orhideelor", "orhidee", "or-hi-de-e", "P"),
    ("femeia", "femeie", "fe-me-ie", "S"),
    ("niceea", "niceea", "", "S"),
    ("spontaneea", "spontaneu", "", "S"),
    ("familia", "familie", "-li-e", "S"),
    ("soarele", "soare", "", "S"),
]


def main() -> None:
    p = argparse.ArgumentParser()
    p.add_argument("--out", type=Path, default=ROOT / "word_db.sqlite")
    args = p.parse_args()
    print(f"{args.out}: {build(args.out, SEED)}")


if __name__ == "__main__":
    main()
