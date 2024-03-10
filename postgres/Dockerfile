FROM postgres:16

# Install pgcrypto and enable the extension
RUN apt-get update && apt-get install -y postgresql-contrib
RUN echo "CREATE EXTENSION IF NOT EXISTS pgcrypto;" >> /docker-entrypoint-initdb.d/init.sql

# Log out queries for debugging
CMD ["postgres", "-c", "log_statement=all", "-c", "log_destination=stderr"]