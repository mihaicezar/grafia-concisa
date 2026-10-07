FROM python:3.12-slim
COPY --from=ghcr.io/astral-sh/uv:0.8 /uv /bin/uv

ENV UV_COMPILE_BYTECODE=1 UV_LINK_MODE=copy PYTHONUNBUFFERED=1
WORKDIR /app

# Dependencies first (cached layer)
COPY pyproject.toml uv.lock ./
RUN uv sync --locked --no-dev --no-install-project

COPY grafia_concisa ./grafia_concisa
COPY server ./server
RUN uv sync --locked --no-dev

# Read-only database, built beforehand with pipeline/build_db.py
COPY word_db.sqlite ./word_db.sqlite

RUN useradd --create-home app
USER app
ENV PATH="/app/.venv/bin:$PATH" WORD_DB=/app/word_db.sqlite PORT=10000
EXPOSE 10000
CMD ["sh", "-c", "exec gunicorn --chdir server --bind 0.0.0.0:${PORT} --workers 2 --threads 4 server:app"]
