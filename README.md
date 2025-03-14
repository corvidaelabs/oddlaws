# oddlaws

[![build](https://github.com/corvidaelabs/oddlaws/actions/workflows/build.yml/badge.svg)](https://github.com/corvidaelabs/oddlaws/actions/workflows/build.yml)

This repository serves as a sandbox for experimenting with oddlaws data applications

## Contributing

We try not to be too strict about contributing guidelines. We encourage you to contribute in any way you can,
whether it's by submitting bug reports, feature requests, or pull requests.

There's a basic pull request template that you can use when submitting a pull request. We have some issues outlined but
they're not exhaustive, contribute to the project as you see fit.

The project is MIT licensed, so feel free to use it in your own projects.

## Installation

### Requirements

- Node >= 23
- A PostgreSQL database

We recommend using [Docker](https://www.docker.com/) to run the PostgreSQL database. You can start a PostgreSQL container with
the following command:

```bash
docker run --name oddlaws-postgres -e POSTGRES_PASSWORD=postgres -e POSTGRES_DB=oddlaws -p 5432:5432 -d postgres
```

### Setup

1. Copy the `.env.example` file to `.env` and fill in the database credentials (if you used the above command, the defaults should work)
2. Run `yarn install` to install the dependencies
3. Run `yarn db:migrate` to create the database tables
4. Run `yarn dev` to start the development server

## Deployment

Merges into `main` currently deploy to the [staging site](https://oddlaws.sneakycrow.dev), but I (sneakycrow) am happy to setup a few other
environments for you if you need one.
