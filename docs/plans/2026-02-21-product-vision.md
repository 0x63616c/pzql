# pzql - Product Vision

## What it is

pzql is a personal PostgreSQL desktop client for Mac. It replaces psql for day-to-day
development - giving you a clean, modern interface to connect to your databases, browse
your data, and run queries.

## Who it's for

Developers who work with Postgres and want a focused tool that doesn't get in the way.
Not an enterprise tool, not a power-user workbench - just the right amount of
functionality, done well.

## Core experience

Three things and nothing else:

1. **Connections** - add multiple connections, persist them, switch between them easily
2. **Table browser** - explore your databases and tables, see your data
3. **SQL runner** - write and run queries

## The feel

Intuitive over clever. Things do what you expect. Hit Enter, your query runs. No
shortcuts to memorize, no modes to switch into. The app should feel obvious on first use.

## What it's not

pzql is not trying to be TablePlus or Beekeeper Studio. It won't do everything. It's a
focused, opinionated tool built for one person's workflow - and that's a feature, not a
limitation.

## v1 scope

**In scope:**
- Connection management - multiple connections, persisted, easy to switch between
- Read-only database and table browser
- SQL runner
- Query history - automatically saved, no manual effort required
- Copy results as CSV or JSON
- Table search and filter

**Out of scope (for now):**
- Row editing outside the SQL runner
- Schema diagrams
- SSH tunnels
- Query explain and analyze
- Multiple result tabs
