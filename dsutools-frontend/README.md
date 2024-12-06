# dsutools-frontend

This template should help get you started developing with Vue 3 in Vite.

## Recommended IDE Setup

[VSCode](https://code.visualstudio.com/) + [Volar](https://marketplace.visualstudio.com/items?itemName=Vue.volar) (and disable Vetur).

## Customize configuration

See [Vite Configuration Reference](https://vitejs.dev/config/).

## Project Setup

This project requires Node.js and npm. However, `apt`'s version of Node.js is seriously outdated, so instead we'll install it with the Node Version Manager (nvm).

```sh
# installs nvm (Node Version Manager)
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.0/install.sh | bash

# download and install Node.js (you may need to restart the terminal)
nvm install 22

# verifies the right Node.js version is in the environment
node -v # should print `v22.8.0`

# verifies the right npm version is in the environment
npm -v # should print `10.8.2`
```

Then, you need to configure your instance of the repository to be an npm project. Also, whenver you need to update the NPM packages, you'll need to run `npm install` to download the new ones.
```sh
npm install
```

### Compile and Hot-Reload for Development

```sh
npm run dev
```

### Compile and Minify for Production
You'll need to run this if you want to run the dsutools-backend server with this frontend.\
It will populate the `dist` directory with the compiled `html`, `css`, and `javascript` files.\
Since we're using `vite` as our build system for the frontent, the `vite.config.js` file describes how to build the frontent.
```sh
npm run build
```

### Packages we're using:
- Vue
- Material 3 Colors
- Tabler Icons : tabler.io/icons
