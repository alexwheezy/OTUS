#!/bin/sh
HOUSE_NAME=$1

curl --request DELETE \
    --url "http://localhost:8080/house/$HOUSE_NAME" \
    --header "Content-Type: application/json"
