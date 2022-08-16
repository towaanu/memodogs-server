#!/bin/sh

docker run \
    -p 3030:3030 \
    -e "RUST_LOG=info" \
    -e "MEMODOGS_PORT=3030" \
    -e "MEMODOGS_STATIC_BASE_URL=https://api.memodogs.towaanu.com/static" \
    -e "MEMODOGS_CORS_ORIGIN=https://memodogs.towaanu.com" \
    memodogs:api0.5
