# grafia concisă

Romanian orthography that writes ie/ia/ea/oa diphthongs (no hiatus) as ẹ/ė/ē/ō. The name is always "grafia concisă" / `grafia-concisa`.

@DECISIONS.md
@concise-writing-spec.md

## Layout
- `grafia_concisa/`: shared rules (`rules.py`) and the DB builder (`db.py`). Put rule logic here only.
- `pipeline/`: `load_dexonline.py` (dexonline dump → local MariaDB), `build_db.py` (MariaDB → `word_db.sqlite`), `seed_db.py` (dev DB, no MySQL)
- `server/`: Flask API (`server.py`), static UI (`static/index.html`), tests (`test_server.py`)

## Commands
- Use `uv` only, never `pip`.
- `uv run python -m unittest discover -s server`: run all tests (must pass before any commit).
- `uv run python pipeline/seed_db.py`: build a small dev DB.
- Full DB: see README.md. A MariaDB server is required: on this machine, a portable one on port 3307; see "This machine" there. Then run:
  - `uv run python pipeline/load_dexonline.py --download --client <mariadb> --port 3307`: load or update dexonline;
  - `uv run --extra pipeline python pipeline/build_db.py --mysql --host 127.0.0.1 --port 3307 --database dexonline`: build.
- `uv run flask --app server/server.py run`: dev server.

## Conventions
- Simple over clever, with best practices, and no speculative features.
- Tests are table-driven: a new case is one line in `WORDS`, `DERIVED`, `SYLLABIFY`, `EE_WORDS`, `TEXTS` or `REVERTS`.
- Docs have separate roles:
  - `DECISIONS.md` logs each settled decision: date, principle, why, an example and the spec section. It holds no mechanics, and it is the authority on intent.
  - `concise-writing-spec.md` holds the complete current rules, with no history. A rule change updates it in the same commit; a disagreement with `DECISIONS.md` is fixed there.
  - `README.ro.md` is the Romanian rendering of the spec, updated with it. It never contains anything that is not in the spec.
- Linguistic edge cases: ask Cezar and don't guess. They are discussed in Romanian.
- Keep the dexonline attribution (GPL) in the UI.

## Open tasks
1. Refine the fallback syllabifier (`rules._rule_cut`). dexonline prevails wherever it has data. `IA_DIPHTHONG`, `EA_HIATUS`, `IE_HIATUS` and `OA_HIATUS` were reviewed by Cezar on 2026-10-09 against dexonline's own syllabifications of similar words. Extend them as new cases appear.
