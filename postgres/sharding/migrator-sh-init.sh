#!/bin/bash

# SQL_DIR="${SQL_DIR:-./ddl}"

set -e

echo 'CREATE DATABASE social_net' | psql $PSQL_CONN_STR
echo 'database social_net created'

echo "GRANT ALL PRIVILEGES ON DATABASE social_net TO $SN_USER" | psql $PSQL_CONN_STR
echo '$SN_USER user granted'

echo $(find $SQL_DIR/ddl -name apply.sql)

find $SQL_DIR/ddl -name apply.sql | sort -V | xargs -I{} psql $PSQL_CONN_STR/social_net -f {} -v ON_ERROR_STOP=1
echo 'release migrations upped'

echo "CREATE EXTENSION citus" | psql $PSQL_CONN_STR/social_net

echo "SELECT create_distributed_table('social_net.messages', 'user_from')" | psql $PSQL_CONN_STR/social_net 