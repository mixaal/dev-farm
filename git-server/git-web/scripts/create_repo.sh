#!/bin/bash

REPO="$1"

[ -z "$REPO" ] && {
  echo "Usage: $0 <repository>"
  exit 1
}

curl -X POST -H "Content-type: application/json" -d  '{"name":"'$REPO'", "kind": "PYTHON"}' http://0.0.0.0:7998 | jq .
