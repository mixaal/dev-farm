#!/bin/bash -x

BASE_URI="http://localhost:7998/"

USER=${1:-mixaal}
PUB_KEY=${2:ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIKrV9+uHFpaOWqowGOwqzhqHnfjUkm8hQQ0NrYdEjMmB mikc@smurf}

curl -X POST -d '{"nick":"'$USER'", "ssh_pub_key": "'"$PUB_KEY"'"}' -H "Content-type: application/json" "$BASE_URI/users"
