set dotenv-load := true

build-docker:
    docker build -t oddlaws:latest . --build-arg BUILD_DB=$DATABASE_URL

dev: dev-bot

dev-bot:
    watchexec -r -w labs/discord-bot -- cargo run --package discord-bot
