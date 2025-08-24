#! /bin/bash

curl --header "Content-Type: application/json" \
  --request POST \
  --data '{"Display":"RainbowOscillation"}' \
  127.0.0.1:30125