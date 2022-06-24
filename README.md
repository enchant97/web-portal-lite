# Web Portal Lite
Web-Portal-Lite is a web app written in Rust using Rocket, aiming to provide a dashboard to manage multiple web links. It is a lightweight alternative to [Web Portal](https://github.com/enchant97/web-portal) which has a larger feature set, this app instead offers minimal possible for users who just want to create fancy looking links and not worry about the advanced features.

It is highly recommended to put this app behind a proxy like Nginx for custom routing, domain names and https connections.

> This app is still in early development, DO NOT use for production. Maybe checkout the normal version at: <https://github.com/enchant97/web-portal>.

## Features
- Access a grid of beautiful links to your web services
- Minimal use of Javascript, to provide a lightning fast experience
- Wide range of link colors
- Icon based theme
- Dark/Light mode
- Links can be put into groups
- Secure the portal with user accounts
- Customise dashboard through a basic yaml file

## Configuration
### Server
Before starting the server certain configs need to be set which are given as environment variables.

> Prefix all environment variables with ROCKET_

> All environment variables must be UPPER CASE

| Name        | Description                          | Default           |
| :---------- | :----------------------------------- | :---------------- |
| CONFIG_PATH | Where the dashboard config lives     | ./data/config.yml |
| ICONS_PATH  | Where icons will be stored           | ./data/icons      |
| SECRET_KEY  | Secure 256-bit base64 encoded string |                   |
| ADDRESS     | What address to bind to              | 0.0.0.0           |
| PORT        | What port to use for bind            | 8000              |

> Openssl can be used to generate a secret key: `openssl rand -base64 32`

### Dashboard
A dashboard config is supplied as a yaml file.

```yml
config_version: 1
public_dash: false

accounts:
  leo:
    password: "< argon2 hashed >"
    dashboard:
      -
        title: "My Group"
        show_header: true
        links:
          -
            title: "My Link"
            color_name: "blue"
            icon_name: "some-app"
            href: "https://example.com"
```

#### Icon Name
To set the icon name config you will need icons in the icon folder, these will need to be arranged in the format shown below:

```
icons/
    svg/
        some-app.svg
    png/
        some-app.png
```

A useful repository to get icons is <https://github.com/walkxhub/dashboard-icons>. They also have the added benefit of already being in this format.

#### Password
To hash a password for the config there is a built in tool which can be accessed like shown below:

```
$ web_portal_lite_cli pw-hasher
$ enter password: <password>
$ hashed password: $argon2id$...
```

Once you have the hash just copy it into the password field in the config.

## License
Copyright (c) 2022 Leo Spratt licenced under AGPL-3, the licence can be found in: `LICENSE.txt`. This project also uses some third-party code, licenses for those can be found at: `LICENSE-THIRD-PARTY.txt`.
