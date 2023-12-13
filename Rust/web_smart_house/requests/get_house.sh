#!/bin/sh
HOUSE_NAME=$1

curl --request GET \
    --url "http://localhost:8080/house/$HOUSE_NAME" \
    --header "Content-Type: application/json"
