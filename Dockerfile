# Use the official Rust image
FROM rust:latest

# Create and set the working directory
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Create a new directory for the source code
RUN mkdir src

# Copy the source code into the container
COPY src/ ./src/

# Build the project
RUN cargo build --release

# Expose the port that the app will run on
EXPOSE 8080 

# Set the command to run the application
CMD ["cargo", "run", "--release"]
