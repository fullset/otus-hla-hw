FROM postgres

COPY . /tmp/db
RUN chmod +x /tmp/db/sharding/migrator-sh-init.sh
ENV SQL_DIR=/tmp/db/schema

ENTRYPOINT ["/tmp/db/sharding/migrator-sh-init.sh"]
