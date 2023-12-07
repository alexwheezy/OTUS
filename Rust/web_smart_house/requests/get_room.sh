#!/bin/sh
HOUSE_ID=$1
ROOM_NAME=$2

curl --request GET \
    --url "http://localhost:8080/house/$HOUSE_ID/room/$ROOM_NAME" \
    --header "Content-Type: application/json"
