# fluentd logging aggregator

This docker-compose.yml starts a fluentd instance and a postgres database.
It'll forward logs to the postgres database.

It expects logs to be in JSON string format.
And the keys expected are "timestamp, target, level, message"

You can tweak this to your needs in the fluentd/fluent.conf
and initdb/init.sql files.

## dashboard

WIP
