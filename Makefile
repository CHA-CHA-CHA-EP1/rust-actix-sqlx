DB_DOCKER_CONTAINER=rust_mydb

create_docker_container:
	docker run --name ${DB_DOCKER_CONTAINER} -p 5420:5432 -e POSTGRES_USER=postgres -e POSTGRES_PASSWORD=123456 -d postgres:12-alpine

create_postgres_db:
	docker exec -it ${DB_DOCKER_CONTAINER} createdb --username=postgres  rust_mydb

install:
	cargo install sqlx-cli

migrate:
	sqlx migrate run
