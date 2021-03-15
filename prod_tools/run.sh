#!/bin/sh

docker run \
    -p 8050:3030 \
    -v `pwd`/assets:/assets \
    --env-file ./.env \
    memodogs:prod-server
