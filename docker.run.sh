#!/bin/sh

docker run \
    -p 3030:3030 \
    -v `pwd`/:/memodogs \
    -w /memodogs \
    -e "RUST_LOG=debug" \
    -e "MEMODOGS_PORT=3030" \
    -e "MEMODOGS_IMAGES_PATH=assets/images" \
    -e "MEMODOGS_STATIC_BASE_URL=http://localhost:3030/static" \
    -it memodogs:dev-server \
    /bin/sh
