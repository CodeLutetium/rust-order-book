# For development purposes only: start docker container with postgres
#!/bin/bash

# Define the PostgreSQL image name and tag
IMAGE_NAME="postgres"
IMAGE_TAG="latest"

# Check if the image exists
if docker images -q ${IMAGE_NAME}:${IMAGE_TAG} > /dev/null 2>&1; then
    echo "The PostgreSQL image '${IMAGE_NAME}:${IMAGE_TAG}' already exists."
else
    echo "The PostgreSQL image '${IMAGE_NAME}:${IMAGE_TAG}' does not exist. Pulling the image..."
    docker pull ${IMAGE_NAME}:${IMAGE_TAG}
fi
docker run -p 5432:5432 -e POSTGRES_PASSWORD=admin -e POSTGRES_DB=order-book -d postgres
