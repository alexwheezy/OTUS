#!/bin/sh
HOUSE_NAME=$1
ROOM_NAME=$2

curl --request POST \
    --url http://localhost:8080/houses/$HOUSE_NAME/rooms \
    --header "Content-Type: application/json" \
    --data '{"name": "'"$ROOM_NAME"'", "devices": []}'
