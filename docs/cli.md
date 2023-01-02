# CLI
This page shows how the built in CLI can be used.

## Using The CLI From Docker
By default when running the container it will execute the "serve" command, you can override this to use the CLI. Here is a few ways this can be accomplished:

Spin up a temporary container overriding the command:

```
docker run --rm -it ghcr.io/enchant97/web-portal-lite:1 <command>
```

Exec into running container (will not work if no config has been set):

```
docker exec -it <container name> ./web-portal-lite <command>
```

## Commands

### Serve
To serve the actual web server this command can be used:

> This will need a valid config before launching

```
web-portal-lite serve
```

### Hash Passwords
To use the user accounts feature a hashed password is expected, use this command to hash a given password. Once the password has been hashed; simply copy it into the password field in your config.

```
web-portal-lite pw-hasher
enter password: <password>
hashed password: $argon2id$...
```

### Generate Config
To gain a template config the following command can be run. This will output to stdout, you can then copy and paste this into a config.yml file.

```
web-portal-lite gen-config
```

### Get Version

```
web-portal-lite version
```
