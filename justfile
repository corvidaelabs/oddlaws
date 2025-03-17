set dotenv-load := true

build-docker:
    docker build -t oddlaws:latest . --build-arg BUILD_DB=$DATABASE_URL

run-discord-bot:
    watchexec -r -w labs/discord-bot -- cargo run --package discord-bot
