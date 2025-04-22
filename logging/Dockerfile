FROM fluent/fluentd:v1.16-1
USER root
RUN apk add --no-cache ruby-dev build-base postgresql-dev
RUN gem install pg
RUN gem install fluent-plugin-sql
USER fluent
