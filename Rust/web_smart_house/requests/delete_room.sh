#!/bin/sh
HOUSE_NAME=$1
ROOM_NAME=$2

curl --request DELETE \
    --url "http://localhost:8080/houses/$HOUSE_NAME/rooms/$ROOM_NAME" \
    --header "Content-Type: application/json"
