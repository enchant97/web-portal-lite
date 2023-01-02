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

| Name                 | Description                                                               | Default           | Docker Default   |
| :------------------- | :------------------------------------------------------------------------ | :---------------- | :--------------- |
| CONFIG_PATH          | Where the dashboard config lives                                          | ./data/config.yml | ./app/config.yml |
| ICONS_PATH           | Where icons will be stored                                                | ./data/icons      | ./app/icons      |
| PUBLIC_DASH_USERNAME | The public dashboard username, only used when public dashboard is enabled | public            | public           |
| SECRET_KEY           | Secure 256-bit base64 encoded string                                      |                   |                  |
| ADDRESS              | What address to bind to                                                   | 0.0.0.0           | 0.0.0.0          |
| PORT                 | What port to use for bind                                                 | 8000              | 8000             |

> Openssl can be used to generate a secret key: `openssl rand -base64 32`

## Dashboard Configuration
Before starting the server a dashboard config will need to be created. This is supplied as a YAML file.

- After modification the server will need to be restarted
- Use the built-in CLI to produce a template config
- Use the built-in CLI to produce a hashed password
- The 'public' account password should be left blank, if `public_dash` is enabled

## Docker
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

## Without Docker

```
git clone https://github.com/enchant97/web-portal-lite.git

cd web-portal-lite

cargo build --release

cp target/release/web-portal-lite /usr/local/bin/web-portal-lite

web-portal-lite <command>
```
