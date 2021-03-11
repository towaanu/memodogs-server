#!/bin/sh

docker run \
    -p 3030:3030 \
    -v `pwd`/:/memodogs \
    -w /memodogs \
    -e "RUST_LOG=debug" \
    -e "MEMODOGS_PORT=3030" \
    -it memodogs:dev-server \
    /bin/sh
