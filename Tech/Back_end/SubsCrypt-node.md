# [SubsCrypt-nodejs-backend](https://github.com/oxydev/SubsCrypt-nodejs-backend)
<img src="https://oxydev.github.io/SubsCrypt-docs/images/logo.jpg" width="500">

[![Test](https://github.com/oxydev/SubsCrypt-nodejs-backend/actions/workflows/node.js.yml/badge.svg)](https://github.com/oxydev/SubsCrypt-nodejs-backend/actions/workflows/node.js.yml)
[![Docker Image CI](https://github.com/oxydev/SubsCrypt-nodejs-backend/actions/workflows/docker-hub-test.yml/badge.svg)](https://github.com/oxydev/SubsCrypt-nodejs-backend/actions/workflows/docker-hub-test.yml)

# What is SubsCrypt-nodejs-backend?
We provide an express-js API wrapper over the [SubsCrypt-npm-library](https://github.com/oxydev/SubsCrypt-npm-library/) to
make it easier for developers to interact with smart contract getter functions. This Rest Api can be used by other libraries to support SubsCrypt in that language (for example in [this repository](#) we made python SubsCrypt library using this Rest Api).

# How to connect?
You have to option right now, either you can run clone this project and run it on your own and also connect to a running instance of SubsCrypt smart contract( You can check the [SubsCrypt NPM package README](https://github.com/oxydev/SubsCrypt-npm-library))

Or you can use our testing server in [here](https://api.subscrypt.io/).

And also stay tuned for the updates and the official product.


# How to Run(using npm)?

First clone the project and then install the required packages:
```
npm install
npm run serve
```

# How to run(using docker-compose)?

First you need Docker and Docker Compose installed on your system. for ubuntu : [Docker](https://www.digitalocean.com/community/tutorials/how-to-install-and-use-docker-on-ubuntu-18-04) (steps 1 and 2), [Docker Compose](https://www.digitalocean.com/community/tutorials/how-to-install-docker-compose-on-ubuntu-18-04) (step 1)

Now you need to run this command to run the server(it will fetch the latest image of project):
```bash
docker-compose up
```
# Building your own image
For building the image of your code, use this as your `docker-compose.yml` file:
```
version: '3.8'
services:
  node:
    network_mode: host
    build:
      context: ./
    volumes:
      - .:/usr/src/app
    command: npm run start
    environment:
      DEBUG: nodejs-docker-express:*
```

Then, run this command:
```bash
docker-compose build
```

The output will be something like this:

```
Building node
[+] Building 0.2s (11/11) FINISHED
 => [internal] load build definition from Dockerfile                                                               0.0s
 => => transferring dockerfile: 38B                                                                                0.0s
 => [internal] load .dockerignore                                                                                  0.0s
 => => transferring context: 34B                                                                                   0.0s
 => [internal] load metadata for docker.io/library/node:12-alpine                                                  0.0s
 => [base 1/4] FROM docker.io/library/node:12-alpine                                                               0.0s
 => [internal] load build context                                                                                  0.0s
 => => transferring context: 6.81kB                                                                                0.0s
 => CACHED [base 2/4] RUN mkdir -p /usr/src/app                                                                    0.0s
 => CACHED [base 3/4] WORKDIR /usr/src/app                                                                         0.0s
 => CACHED [base 4/4] COPY package.json .                                                                          0.0s
 => CACHED [dev 1/2] RUN npm install -g nodemon && npm install                                                     0.0s
 => [dev 2/2] COPY ./ .                                                                                            0.1s
 => exporting to image                                                                                             0.1s
 => => exporting layers                                                                                            0.0s
 => => writing image sha256:631ef82544f14889464c7c1c9bc2e4b5ea560b34db8ce20ae8aedd0a89169340                       0.0s
 => => naming to docker.io/library/subscrypt-nodejs-backend_node                                                   0.0s
```

And then you can run your image with ths command:
```bash
docker-compose up
```

# Documentation
This service is a RESTful API and you can see our api documentation [here](https://api.subscrypt.io/subscrypt-doc).

