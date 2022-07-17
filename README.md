
# Git Ticket Summary

A tool that runs through a Git repository gathering commits associated with a
given ticket identifier so that it can present a summary to you of teh commits
related to the ticket as well as all the files touched by those commits.

For each of the commits it provides links to the commits & files. Currently
this only supports GitHub Cloud.

## Installation

### macOS

On macOS `git-ticket-summary` is distributed via our Homebrew tap. Thus you can
install it simply by running the following.

```
brew tap uptech/homebrew-oss
brew install git-ticket-summary
```

### Other

On any other platform or if you just want to install from source you may do so
doing the following.

Simply copy `git-ticket-summary` to a location that exists within your `PATH`
environment variable, e.g. `/usr/local/bin`.

Some people like to create a `~/bin` folder and add it to their `PATH`
environment variable and then install it there.

## Usage

```
git-ticket-summary <ticket-id> <branch>
```

The `git-ticket-summary` command requires two command line arguments.

1. `<ticket-id>` - the ticket ID that you would like a summary for, e.g. `FI-21`
2. `<branch>` - the you want it to start at when traversing the tree to find associated commits, e.g. `main`

## Example

The following is an example run with it's output.

```
✔ git-ticket-summary FI-21 main

Ticket Summary

ticket: FI-21
repository: https://github.com/Viva-Equity/faninc-api.git

## Commits

7696f0b - N - Add allowed athletes table (3 days ago) <Alon Dahari> https://github.com/Viva-Equity/faninc-api/commit/7696f0b811cf1d01559a1fcf351cc8ce8b3cb907
1d841aa - N - Add get current athlete profile query (3 days ago) <Alon Dahari> https://github.com/Viva-Equity/faninc-api/commit/1d841aa6e73e07d6de6633453b0a7a2f3db457ca
ce34f9f - N - Add schoolEmail to athlete profile dto (3 days ago) <Alon Dahari> https://github.com/Viva-Equity/faninc-api/commit/ce34f9f1e36accdcfbc26e823aa7751ee380cd77
01011c8 - N - Add school email to athlete profile (5 days ago) <Alon Dahari> https://github.com/Viva-Equity/faninc-api/commit/01011c84698147a2dc762e19988a55f3cbc77c2e

## Files created / deleted / modified

https://github.com/Viva-Equity/faninc-api/blob/main/prisma/migrations/20220712151445_add_school_email_to_athlete_profile/migration.sql
https://github.com/Viva-Equity/faninc-api/blob/main/prisma/migrations/20220714095556_add_allowed_athletes/migration.sql
https://github.com/Viva-Equity/faninc-api/blob/main/prisma/schema.prisma
https://github.com/Viva-Equity/faninc-api/blob/main/src/athlete-profile/athlete-profile-entity.converter.ts
https://github.com/Viva-Equity/faninc-api/blob/main/src/athlete-profile/athlete-profile.interface.ts
https://github.com/Viva-Equity/faninc-api/blob/main/src/athlete-profile/athlete-profile.repository.spec.ts
https://github.com/Viva-Equity/faninc-api/blob/main/src/athlete-profile/athlete-profile.repository.ts
https://github.com/Viva-Equity/faninc-api/blob/main/src/athlete-profile/athlete.mocks.ts
https://github.com/Viva-Equity/faninc-api/blob/main/src/graphql/athlete-profile/athlete-profile.graphql
https://github.com/Viva-Equity/faninc-api/blob/main/src/graphql/athlete-profile/athlete-profile.resolver.ts
https://github.com/Viva-Equity/faninc-api/blob/main/src/graphql/graphql.ts
```
