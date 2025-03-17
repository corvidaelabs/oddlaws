# syntax = docker/dockerfile:1
FROM node:23 AS base
WORKDIR /app

# Create a stage specifically for dependency installation
FROM base AS deps

# Install packages needed to build node modules
RUN apt-get update -qq && \
    apt-get install -y python-is-python3 pkg-config build-essential && \
    rm -rf /var/lib/apt/lists/*

# Copy only package.json and yarn.lock first
COPY --link package.json yarn.lock ./

# Install dependencies
ENV DEBUG='vite:*'
RUN yarn install --frozen-lockfile

# Build stage
FROM deps AS build

# Copy application code
COPY --link src/ /app/src/
COPY --link static/ /app/static/
COPY --link eslint.config.js /app/
COPY --link .nvmrc /app/
COPY --link .prettierignore /app/
COPY --link .prettierrc /app/
COPY --link package.json /app/
COPY --link svelte.config.js /app/
COPY --link vite.config.ts /app/
COPY --link tsconfig.json /app/
COPY --link yarn.lock /app/

# Build time database URL for a migration check
ARG BUILD_DB
ENV DATABASE_URL=$BUILD_DB

# Build application
RUN yarn build

# Production stage
FROM base

# Install production-only packages
RUN apt-get update -qq && \
    apt-get install -y ca-certificates openssl && \
    rm -rf /var/lib/apt/lists/*

# Copy only production dependencies from deps stage
COPY --from=deps /app/node_modules /app/node_modules

# Copy built application
COPY --from=build /app /app

EXPOSE 3000
CMD [ "yarn", "start" ]
