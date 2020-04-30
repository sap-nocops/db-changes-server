## DB-CHANGES

[![Build Status](https://img.shields.io/endpoint.svg?url=https%3A%2F%2Factions-badge.atrox.dev%2Fsap-nocops%2Fdb-changes-server%2Fbadge%3Fref%3Dmaster&style=flat)](https://actions-badge.atrox.dev/sap-nocops/db-changes-server/goto?ref=master)

Rest api to get versions and changes of your app's database.
This is projects aim to be a server that provides db modification for your apps.
The purpose is to let you update db of your app without publishing a new version of the latter

### Usage

```markdown
./db-changes-server

Options:
  --port            server port. default: 8000
  --refresh-time    cache refresh time in seconds. default: 3600
  --db-path         path to db. default: ~/.db-changes/changes.db
  --changes-path    path to db version changes. default: ~/.db-changes/apps
  --help            display usage information

for example: ./db-changes-server --port 9000 --refresh-time 50
will run the server on port 9000 refreshing the local cache every 50 seconds

the server needs a sqlite db where to store app's and db's versions, the db's path is specified through the `--db-path` option.
In the `db_ddl.sql` file you'll find the dd to create such db.
the db versions are stored in a folder specified through the `changes-path` option.
The needs to be text file named as the corresponding db version and containing the sql that makes the change.

for example:
```
```sql
CREATE TABLE frattaglie (id int, name varchar);
INSERT INTO frattaglie (id, name) VALUES (1, 'lampredotto');
```

### Endpoints

The following endpoints are exposed to retrieve the db versions and changes

- /versions/<app_name>/<app_version>
- /changes/<app_name>/<db_version>
