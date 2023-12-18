#!/bin/sh
HOUSE_NAME=$1
ROOM_NAME=$2
DEVICE_NAME=$3
POWER_VALUE=$4
ENABLE=$5

curl --request POST \
    --url http://localhost:8080/houses/$HOUSE_NAME/rooms/$ROOM_NAME/devices \
    --header "Content-Type: application/json" \
    --data '{"'"$DEVICE_NAME"'": {"power": '$POWER_VALUE', "enable": '$ENABLE'}}'
