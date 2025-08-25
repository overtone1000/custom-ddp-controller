#! /bin/bash

curl --header "Content-Type: application/json" \
  --request POST \
  --data '"RunRandomPreview"' \
  127.0.0.1:30125