#!/bin/sh
HOUSE_NAME=$1
ROOM_NAME=$2

curl --request GET \
    --url "http://localhost:8080/house/$HOUSE_NAME/room/$ROOM_NAME" \
    --header "Content-Type: application/json"
