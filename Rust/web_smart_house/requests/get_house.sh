#!/bin/sh
HOUSE_NAME=$1

curl --request GET \
    --url "http://localhost:8080/houses/$HOUSE_NAME" \
    --header "Content-Type: application/json"
