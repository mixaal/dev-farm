#!/bin/bash -xe

cd git-web
./build.sh
cd -

sudo docker build -t git-server .

