# Memodogs server
This is the api used by the memodogs game. You can play the game [here](memodogs.towaanu.com). \
This projects uses [Rust](https://www.rust-lang.org/) and [warp](https://github.com/seanmonstar/warp) \
You can find the client implementation [here](https://github.com/towaanu/memodogs)

## Run locally

### Using Docker
In order to run the project using docker, you need to build an image first. \
At the root of the project you can run :
```sh
    ./docker.build.sh
```
This command will create a Docker image named memodogs:dev-server.\
Once we have the image, we can run a new container containing the app using the following command:
```sh
    docker run \
    -p 3030:3030 \
    -v `pwd`/:/memodogs \
    -w /memodogs \
    -e "RUST_LOG=debug" \
    -e "MEMODOGS_PORT=3030" \
    -e "MEMODOGS_IMAGES_PATH=assets/images" \
    -e "MEMODOGS_STATIC_BASE_URL=http://localhost:3030/static" \
    -e "MEMODOGS_CORS_ORIGIN=http://localhost:3000" \
    -it memodogs:dev-server \
    cargo run
```
The server should be available here: [http://localhost:3000](http://localhost:3000). \
The server api should display "hello world" when requesting [http://localhost:3000](http://localhost:3000). \

*Note: You can use ./docker.run.sh to open a new shell inside the container. Then if you run cargo run, the app should be running.*


## Images assets
The images assets folder can be configured using the "MEMODOGS_IMAGES_PARTH" environment variable. \
By default the direcroty used is assets/images. \
Images should be put under `images/dogs` and `images/cats`. You need 24 images for dogs and 24 images for cats. \
Images must be suffix by `.jpg`.

## Client
The client can be found on [this repo](https://github.com/towaanu/memodogs)
