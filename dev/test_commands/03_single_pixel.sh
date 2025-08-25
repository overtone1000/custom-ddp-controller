#! /bin/bash

curl --header "Content-Type: application/json" \
  --request POST \
  --data '{"SinglePixel":['$1',{"red":255,"green":7,"blue":42}]}' \
  127.0.0.1:30125

  #{"SinglePixel":[14,{"red":255,"green":7,"blue":42}]}