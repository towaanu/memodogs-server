#!/bin/sh

docker run \
    -p 3030:3030 \
    -v `pwd`/:/memodogs \
    -w /memodogs \
    -it memodogs:dev-server \
    /bin/sh
