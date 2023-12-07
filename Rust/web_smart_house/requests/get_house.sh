#!/bin/sh
HOUSE_ID=$1

curl --request GET \
    --url "http://localhost:8080/house/$HOUSE_ID" \
    --header "Content-Type: application/json"
