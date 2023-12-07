#!/bin/sh
HOUSE_ID=$1

curl --request DELETE \
    --url "http://localhost:8080/house/$HOUSE_ID" \
    --header "Content-Type: application/json"
