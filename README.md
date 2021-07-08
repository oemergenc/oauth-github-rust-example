# OAuth exmaple workflow implemented in rust

## Prerequisites

You need to have rust installed. On mac os x run the following to install rust:

```
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
# you may need to restart your shell
rustup update
```

You also need to setup a github oauth application. This can be done on the following way:

* Go to https://github.com/settings/applications/new
* Enter an application name
* Enter this in `Homepage url`: http://localhost:3000
* Enter this in `Authorization callback URL`: http://localhost:3000/oauth2callback
* Register application
* On the following page you will see a client id and a client secret, those will be needed

## Getting started

```
git clone url
cd oauth-github-rust-example
export GITHUB_CLIENT_ID=$YOUR_CLIENT_ID;export GITHUB_CLIENT_SECRET=$YOUR_CLIENT_SECRET; cargo run 
```

After that follow the instructions in the console. If everything works as expected you should see an access token
printed in the console.
