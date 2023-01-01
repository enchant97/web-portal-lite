# Install
This section will demonstrate how Web Portal Lite can be installed and configured.

## Important Notes
Please read these notes before continuing.

- Docker is not required, although not currently documented
- HTTPS is not currently supported, you will need to use a reverse proxy
- App icons are not included, you will need to provide your own

## Server Configuration
Before starting the server certain configs need to be set which are given as environment variables.

> Prefix all environment variables with ROCKET_

> All environment variables must be UPPER CASE

| Name                 | Description                                                               | Default          |
| :------------------- | :------------------------------------------------------------------------ | :--------------- |
| CONFIG_PATH          | Where the dashboard config lives                                          | ./app/config.yml |
| ICONS_PATH           | Where icons will be stored                                                | ./app/icons      |
| PUBLIC_DASH_USERNAME | The public dashboard username, only used when public dashboard is enabled | public           |
| SECRET_KEY           | Secure 256-bit base64 encoded string                                      |                  |
| ADDRESS              | What address to bind to                                                   | 0.0.0.0          |
| PORT                 | What port to use for bind                                                 | 8000             |

> Openssl can be used to generate a secret key: `openssl rand -base64 32`

## Dashboard Configuration
Before starting the server a dashboard config will need to be created. This is supplied as a YAML file.

> After modification the server will need to be restarted (will change in the future)

> To produce a hashed password a utility has been provided.

> The public account password should be left blank, if `public_dash` is enabled.

```yml
config_version: 1
public_dash: true

accounts:
  public:
    password: ""
    dashboard:
      -
        title: "My Group"
        compact: true
        show_header: false
        links:
          -
            title: "My Link"
            color_name: "blue"
            icon_name: "some-app"
            href: "https://example.com"
  leo:
    password: "< argon2 hashed >"
    dashboard:
      -
        title: "My Group"
        compact: false
        show_header: true
        links:
          -
            title: "My Link"
            color_name: "blue"
            icon_name: "some-app"
            href: "https://example.com"
```

### Add Icons
To set the icon name config you will need icons in the icon folder, these will need to be arranged in the format shown below:

```
icons/
    svg/
        some-app.svg
    png/
        some-app.png
```

A popular repository for getting these icons is at: <https://github.com/walkxcode/Dashboard-Icons>. You can also get my app icons at: <https://github.com/enchant97/app-icons>. These projects are both in the correct format.

### Hash Password
To hash a password for the config there is a built in tool which can be accessed like shown below:

```
docker exec -it <container name> ./web-portal-lite pw-hasher
enter password: <password>
hashed password: $argon2id$...
```

Or spin up a temporary container overriding the command:

```
docker run --rm ghcr.io/enchant97/web-portal-lite:1 pw-hasher
enter password: <password>
hashed password: $argon2id$...
```

Once you have the hash just copy it into the password field in the config.

## Run
After configs have been created, you can create a Docker Compose file:

```yml
version: "3"

services:
  web-portal-lite:
    container_name: web-portal-lite
    image: ghcr.io/enchant97/web-portal-lite:1
    restart: unless-stopped
    volumes:
      - ./config.yml:/app/config.yml:ro
      - ./icons:/app/icons:ro
    ports:
      - 8000:8000
    environment:
      # THIS MUST BE CHANGED
      - "ROCKET_SECRET_KEY=pMZceGSN6w85+KEmpywAivvp9OYmul6XXnaQXVveVmE="

```

Then run:

```
docker compose up -d
```
