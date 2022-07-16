
# Git Ticket Summary

This repository houses the source code for the Git Ticket Summary command line
tool.

It is a tool that runs through a Git repository and gathers & presents all the
commits associated with a given ticket identifier and presents them as well as
all the files touched by those commits.

It currently also generates GitHub links to the commits so that people can go
to them and comment inline on the code to provide feedback.

Beyond that it provides links to all of the touch files using the currently
checked out branch. This is useful to gain further context around if & how the
code has progressed since the commits happened.

```
✔ git-ticket-summary FI-21

Ticket Summary - FI-21

## Commits

7696f0b - N - Add allowed athletes table (2 days ago) <Alon Dahari> https://github.com/Viva-Equity/faninc-api/commit/7696f0b811cf1d01559a1fcf351cc8ce8b3cb907
1d841aa - N - Add get current athlete profile query (2 days ago) <Alon Dahari> https://github.com/Viva-Equity/faninc-api/commit/1d841aa6e73e07d6de6633453b0a7a2f3db457ca
ce34f9f - N - Add schoolEmail to athlete profile dto (2 days ago) <Alon Dahari> https://github.com/Viva-Equity/faninc-api/commit/ce34f9f1e36accdcfbc26e823aa7751ee380cd77
01011c8 - N - Add school email to athlete profile (4 days ago) <Alon Dahari> https://github.com/Viva-Equity/faninc-api/commit/01011c84698147a2dc762e19988a55f3cbc77c2e

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
