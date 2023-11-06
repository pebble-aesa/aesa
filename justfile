build: build-frontend
  cargo build

build-frontend:
  cd public && yarn install && yarn build

run: build
  cargo shuttle run

deploy: build-frontend
  cargo shuttle deploy

migrate:
  cargo sqlx migrate run --database-url postgres://aesa:aesa@localhost:5432/aesa
