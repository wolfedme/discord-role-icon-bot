# Makefile for Rust project compilation and Docker image creation

# Variables
CARGO := cargo
DOCKER := docker
PROJECT_NAME := discord-role-icon-bot
DOCKER_IMAGE_NAME := discord-role-icon-bot
DOCKER_TAG := latest

# Default target
all: build docker

# Compile the Rust project
build:
	@echo "Building Rust project..."
	$(CARGO) build --release

# Create Docker image
docker: build
	@echo "Creating Docker image..."
	$(DOCKER) build -t $(DOCKER_IMAGE_NAME):$(DOCKER_TAG) .

# Clean build artifacts and remove Docker image
clean:
	@echo "Cleaning up..."
	$(CARGO) clean
	$(DOCKER) rmi $(DOCKER_IMAGE_NAME):$(DOCKER_TAG) || true

# Run the Docker container
run:
	@echo "Running Docker container..."
	$(DOCKER) run --rm $(DOCKER_IMAGE_NAME):$(DOCKER_TAG)

# Help target
help:
	@echo "Available targets:"
	@echo "  all     - Build Rust project and create Docker image (default)"
	@echo "  build   - Compile the Rust project"
	@echo "  docker  - Create Docker image"
	@echo "  clean   - Remove build artifacts and Docker image"
	@echo "  run     - Run the Docker container"
	@echo "  help    - Show this help message"

.PHONY: all build docker clean run help
