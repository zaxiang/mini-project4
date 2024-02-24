FROM rust:1.75 AS builder

# Set the working directory
WORKDIR /usr/src/actix_web_app

# Change the user to root
USER root

# Copy everything
COPY . .

# Build the project
RUN cargo build --release

# Expose port 8080 (optional, doesn't actually publish the port)
EXPOSE 8080

# Change the user to a non-root user
USER 1001:1001

# Define the default command to run
CMD ["./target/release/actix_web_app"]