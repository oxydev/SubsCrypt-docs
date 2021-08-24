# [SubsCrypt-front](https://github.com/oxydev/SubsCrypt-front)
<a href="https://github.com/oxydev/SubsCrypt-front"><img src="https://oxydev.github.io/SubsCrypt-docs/images/logo.jpg" width="500"></a>

[![Docker Image CI](https://github.com/oxydev/SubsCrypt-front/actions/workflows/docker-hub-test.yml/badge.svg)](https://github.com/oxydev/SubsCrypt-front/actions/workflows/docker-hub-test.yml)

This is an open source project for generating the front end part of a big project called SubsCrypt. The main project is for creating subscription plans and subscribe to them by the use of cryptocurrency. Front end codes generates completely client side, and the necessary information are received from the blockchain and the server.

The necessary information like the users and providers addresses, their plans info and their username and wallet addresses are stored on the blockchain. They are received and used in the project by the use of [SubsCrypt npm library](https://www.npmjs.com/package/@oxydev/subscrypt). The other information which is not necessary for transactions is stored on [SubsCrypt backend server](https://github.com/oxydev/SubsCrypt-nodejs-backend) and can be received by a simple api fetching.


# Technologies

The following technologies are used in the project:

* React.Js version: 17.0.2
* Next.Js version: 11.0.1
* axios version: 0.21.1
* sass version: 1.35.2

# Live Version
You can check out the live version in: https://subscrypt.io/

# Running and testing

For running and testing this code on your device the following command is needed:

For installing the dependencies:
```
npm install

```
or
```
yarn install

```

For running the project in a development mode:
```
npm run dev

```

For generating a production version to run on a server:
```
npm run build

```

# How to run(using docker-compose)?

First you need Docker and Docker Compose installed on your system. for ubuntu : [Docker](https://www.digitalocean.com/community/tutorials/how-to-install-and-use-docker-on-ubuntu-18-04) (steps 1 and 2), [Docker Compose](https://www.digitalocean.com/community/tutorials/how-to-install-docker-compose-on-ubuntu-18-04) (step 1)

Now you need to run this command to run the server(it will fetch the latest image of project):
```bash
docker-compose up
```
