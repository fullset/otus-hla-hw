FROM postgres

COPY . /tmp/db
RUN chmod +x /tmp/db/migrator-init.sh
ENV SQL_DIR=/tmp/db/schema
ENV MIGRATIONS_DIR=/tmp/db/migrations

ENTRYPOINT ["/tmp/db/migrator-init.sh"]
