# For development purposes only: start docker container with postgres
docker run -p 5432:5432 -e POSTGRES_PASSWORD=admin -e POSTGRES_DB=order-book -d postgres
