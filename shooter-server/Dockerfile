# Use an existing docker image as a base
FROM ubuntu:latest

ARG USERNAME
ARG EMAIL
ARG PASSWORD

# Install dependencies
RUN apt-get update && apt-get install -y curl

# Set the working directory
WORKDIR /shooter-server/LinuxServer


# Copy the server files into the container
COPY . .

# Expose the necessary port
EXPOSE 7777/udp

# Command to run the server
CMD ["./Shooter_8INF916.sh", "-log"]

# Register the server
CMD ["./register.sh", "$USERNAME", "$EMAIL", "$PASSWORD"]