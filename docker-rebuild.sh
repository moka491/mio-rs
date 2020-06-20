#!/bin/bash
docker-compose down
git pull
COMPOSE_DOCKER_CLI_BUILD=1 DOCKER_BUILDKIT=1 docker-compose build --force-rm
docker compose up -d