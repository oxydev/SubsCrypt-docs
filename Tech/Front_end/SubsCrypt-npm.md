# [SubsCrypt-npm-library](https://github.com/oxydev/SubsCrypt-npm-library)
<img src="https://oxydev.github.io/SubsCrypt-docs/images/logo.jpg" width="500">

[![npm](https://img.shields.io/npm/v/@oxydev/subscrypt.svg)](https://www.npmjs.com/package/@oxydev/subscrypt)
[![Test status](https://github.com/oxydev/SubsCrypt-npm-library/actions/workflows/node.js.yml/badge.svg)](https://github.com/oxydev/SubsCrypt-npm-library/actions/workflows/node.js.yml)
[![npm module downloads](https://badgen.net/npm/dt/@oxydev/subscrypt)](https://www.npmjs.com/package/@oxydev/subscrypt)
# Import Package
We have made this wrapper over SubsCrypt contract to make uses of our api easier.
You have to option right now, either you can use the latest version of code on repo or use the npm package on npm repository which is the latest version of this repo's release.
* Clone the repo and add this to `package.json` of your project.
    ```
    "@oxydev/subscrypt": "file:../SubsCrypt-npm-library",
    ```
* Or run this command, and then you can use latest SubsCrypt npm package
    ```
    yarn add @oxydev/subscrypt
    ``` 
# Use Package in your project
```
import subscrypt from '@oxydev/subscrypt';
```
or
```
const subscrypt = require('@oxydev/subscrypt');
```
Now you can easily use the SubsCrypt library to get data from blockchain. Below are some examples:
```
console.log(await subscrypt.getPlanData('5Dyu5YxLufavjPg8vP31BhKs5xz8ncdkQcNdGwf5XtW4C9Ym', 2));

console.log(await subscrypt.checkSubscription('5Dyu5YxLufavjPg8vP31BhKs5xz8ncdkQcNdGwf5XtW4C9Ym','5Dyu5YxLufavjPg8vP31BhKs5xz8ncdkQcNdGwf5XtW4C9Ym', 2));

console.log(await subscrypt.retrieveWholeDataWithPassword('5Dyu5YxLufavjPg8vP31BhKs5xz8ncdkQcNdGwf5XtW4C9Ym',"token", "pass_phrase"));
```
More Samples will be added to the documents of project [here](https://oxydev.github.io/SubsCrypt-docs/#/).

# Use SubsCrypt CLI

### Install package
```
npm install -g @oxydev/subscrypt
```
### Use CLI
```
subscrypt --version
```
# Docs
You can access the docs [here](/docs)
