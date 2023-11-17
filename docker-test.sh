#!/usr/bin/bash

cleanup () {
  docker kill weather_app
  docker rm -f weather_app
}

trap 'cleanup ; printf "${RED}Tests Failed For Unexpected Reasons${NC}\n"' HUP INT QUIT PIPE TERM

docker buildx build -t weather_app:latest .
docker run --name weather_app weather_app

cleanup
