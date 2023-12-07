#!/bin/sh
HOUSE_NAME=$1

curl --request POST \
    --url http://localhost:8080/house \
    --header "Content-Type: application/json" \
    --data '{"name": "'"$HOUSE_NAME"'", "rooms": {}}'
