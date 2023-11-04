create schema social_net;

CREATE USER social_net INHERIT LOGIN PASSWORD 'social_net';
GRANT ALL PRIVILEGES ON DATABASE social_net TO social_net;
GRANT USAGE ON SCHEMA social_net to social_net;