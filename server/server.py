"""grafia concisă — HTTP API and static frontend.

Endpoints:
  POST /api/convert  {"text": str, "ee"?: bool, "ai"?: bool}     -> {"text", "segments", "unknown"}
  POST /api/revert   {"text": str, "ee"?: bool}                   -> {"text", "segments"}
  POST /api/lookup   {"words": [str], "ee"?: bool, "ai"?: bool}  -> {"results": {word: concise},
                                                                     "unknown",
                                                                     "alternatives": {word: [spellings]}}

Words missing from the database are converted by rules alone and listed in "unknown".
A homograph's segment (convert) or entry (lookup) lists all its spellings, default first;
so does an unknown word with the -ee switch in -eei (unchanged or -eii) or -eele/-eelor
(plural of an -ee or an -eu lemma), unless its lemma is in the database (Andreei <- Andreea,
bombeele <- bombeu: one spelling).

"ee" switches on the optional word-final -ee rule (default off); for revert, the input
  was written with it (aleẹ -> alee, not aleie: see the ee_revert table), and a tie's
  segment lists all its forms (bordeẹ: bordeie or bordee).
"ai" switches on â -> î, except in words with the root român/rumân (default on).
  GET  /healthz
"""
import os
import sqlite3
import threading
from collections.abc import Iterable, Iterator
from pathlib import Path

from flask import Flask, jsonify, request

from grafia_concisa.rules import (
    Row,
    a_to_i,
    convert_text,
    has_candidate,
    normalize,
    resolve,
    revert_segments,
)

ROOT = Path(__file__).resolve().parent.parent
MAX_TEXT_CHARS = 100_000
MAX_LOOKUP_WORDS = 1_000
_SQL_CHUNK = 500  # stay well under SQLite's bound-parameter limit


def create_app(db_path: str | Path | None = None) -> Flask:
    db_path = Path(db_path or os.environ.get("WORD_DB", ROOT / "word_db.sqlite"))
    app = Flask(__name__, static_folder="static", static_url_path="")
    app.config["MAX_CONTENT_LENGTH"] = 1 << 20
    local = threading.local()  # one read-only connection per thread

    def conn() -> sqlite3.Connection:
        c = getattr(local, "conn", None)
        if c is None:
            c = sqlite3.connect(f"file:{db_path}?mode=ro", uri=True)
            local.conn = c
        return c

    def select(sql: str, keys: Iterable[str]) -> Iterator[tuple]:
        keys = list(keys)
        for i in range(0, len(keys), _SQL_CHUNK):
            chunk = keys[i : i + _SQL_CHUNK]
            yield from conn().execute(sql.format(",".join("?" * len(chunk))), chunk)

    def lookup(keys: Iterable[str]) -> dict[str, Row]:
        rows = select("SELECT form, concise_form, concise_ee, alternatives FROM forms "
                      "WHERE form IN ({})", keys)
        return {f: (c, e, tuple((a or "").split())) for f, c, e, a in rows}

    def lookup_ee(keys: Iterable[str]) -> dict[str, tuple[str, tuple[str, ...]]]:
        rows = select("SELECT concise, form, alternatives FROM ee_revert WHERE concise IN ({})", keys)
        return {c: (f, tuple((a or "").split())) for c, f, a in rows}

    def lookup_known(words: Iterable[str]) -> dict[str, str]:
        """Lemmas in -eu, -ea or -iesc from their tables; any other word (-ee) among the forms.
        Each maps to its syllabification: an -iesc lemma's, '' for the others."""
        words = set(words)
        eu = {w for w in words if w.endswith("eu")}
        ea = {w for w in words if w.endswith("ea")}
        iesc = {w for w in words if w.endswith("iesc")}
        return dict(row for sql, keys in (
            ("SELECT lemma, '' FROM eu_lemmas WHERE lemma IN ({})", eu),
            ("SELECT lemma, '' FROM ea_lemmas WHERE lemma IN ({})", ea),
            ("SELECT lemma, reference FROM iesc_lemmas WHERE lemma IN ({})", iesc),
            ("SELECT form, '' FROM forms WHERE form IN ({})", words - eu - ea - iesc),
        ) for row in select(sql, keys))

    def body() -> dict:
        data = request.get_json(silent=True)
        if not isinstance(data, dict):
            raise BadRequest("Expected a JSON object")
        for switch in ("ee", "ai"):
            if not isinstance(data.get(switch, False), bool):
                raise BadRequest(f'"{switch}" must be a boolean')
        return data

    def text_field(data: dict) -> str:
        text = data.get("text")
        if not isinstance(text, str):
            raise BadRequest('Expected JSON {"text": "..."}')
        if len(text) > MAX_TEXT_CHARS:
            raise BadRequest(f"Text longer than {MAX_TEXT_CHARS} characters")
        return text

    @app.errorhandler(BadRequest)
    def bad_request(e: "BadRequest"):
        return jsonify(error=str(e)), 400

    @app.get("/")
    def index():
        return app.send_static_file("index.html")

    @app.get("/healthz")
    def healthz():
        conn().execute("SELECT 1 FROM forms LIMIT 1")
        return {"ok": True}

    @app.post("/api/convert")
    def api_convert():
        data = body()
        segments = convert_text(text_field(data), lookup, data.get("ee", False), lookup_known,
                                data.get("ai", True))
        return {
            "text": "".join(s["t"] for s in segments),
            "segments": segments,
            "unknown": sorted({s["unknown"] for s in segments if "unknown" in s}),
        }

    @app.post("/api/revert")
    def api_revert():
        data = body()
        segments = revert_segments(text_field(data), lookup_ee if data.get("ee", False) else None)
        return {"text": "".join(s["t"] for s in segments), "segments": segments}

    @app.post("/api/lookup")
    def api_lookup():
        data = body()
        ee, ai = data.get("ee", False), data.get("ai", True)
        words = data.get("words")
        if not isinstance(words, list) or not all(isinstance(w, str) for w in words):
            raise BadRequest('Expected JSON {"words": ["..."]}')
        if len(words) > MAX_LOOKUP_WORDS:
            raise BadRequest(f"More than {MAX_LOOKUP_WORDS} words")
        resolved = resolve({normalize(w) for w in words if has_candidate(w, ee)}, lookup, ee, lookup_known)
        results, unknown, alternatives = {}, [], {}
        for w in words:
            key = normalize(w)
            if key not in resolved:
                results[w] = key
                continue
            spellings, is_unknown = resolved[key]
            results[w] = spellings[0]
            if len(spellings) > 1:
                alternatives[w] = list(spellings)
            if is_unknown:
                unknown.append(w)
        if ai:
            results = {w: a_to_i(x) for w, x in results.items()}
            alternatives = {w: [a_to_i(x) for x in xs] for w, xs in alternatives.items()}
        return {"results": results, "unknown": unknown, "alternatives": alternatives}

    return app


class BadRequest(Exception):
    pass


app = create_app()
