dev:
	docker-compose -f ../docker-compose.dev.yml up -d server
	
dev-down:
	docker-compose -f ../docker-compose.dev.yml down

migrate-up:
	sqlx migrate run

migrate-down:
	sqlx migrate revert

start-server:
	cargo watch -q -c -w src/ -x run

start-db:
	docker-compose -f ../docker-compose.dev.yml up -d postgres

install:
	cargo add actix-web
	cargo add actix-cors
	cargo add serde_json
	cargo add serde --features derive
	cargo add chrono --features serde
	cargo add env_logger
	cargo add dotenv
	cargo add uuid --features "serde v4"
	cargo add sqlx --features "runtime-async-std-native-tls postgres chrono uuid"
	# HotReload
	# cargo install cargo-watch
	# SQLX-CLI
	# cargo install sqlx-cli

build:
	cargo build

run:
	cargo run