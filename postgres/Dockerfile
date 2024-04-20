# Use the postgres image as the base
FROM postgres:latest

# Set environment variables (replace with your own values)
ENV POSTGRES_USER postgres
ENV POSTGRES_PASSWORD ff
ENV POSTGRES_DB ShooterDB

# Copy the SQL scripts to the docker-entrypoint-initdb.d directory
COPY ./schema /docker-entrypoint-initdb.d/

# The postgres image's entrypoint will execute any *.sql, *.sql.gz, or *.sh scripts
# in /docker-entrypoint-initdb.d in alphabetical order. You don't need to specify an ENTRYPOINT or CMD.