FROM totallynotrust/rust-default:latest

COPY . /app
WORKDIR /app

# Install Diesel CLI
RUN ~/.cargo/bin/cargo install diesel_cli --no-default-features --features "mysql"

# Install dependencies
RUN apt-get update
RUN apt-get install -y mysql-client

# Copy the migration scripts to the container
COPY ./migrations ./migrations

# Set up the entrypoint script
COPY entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh

# Specify the command to run the entrypoint script
ENTRYPOINT ["./entrypoint.sh" ]
#ENTRYPOINT [ "sleep", "infinity" ]