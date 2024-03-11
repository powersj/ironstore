# IronStore

*Dirt simple Redis in Rust*

## Overview

The goal of this project was to give myself a larger Rust project to play with.

## Server

To run the server on `localhost:8080` run:

```bash
make server
ironstore-server
```

## Client

To build the client:

```bash
make cli
ironstore-cli
```

Below outlines the various commands supported by the client:

| Command  | Description                     | Example            |
|----------|---------------------------------|--------------------|
| append   | Append to existing value        | `append key value` |
| del      | Delete value                    | `del key`          |
| exists   | Does a key exist                | `exists key`       |
| flushall | Delete all keys and databases   | `flushall`         |
| get      | Get value                       | `get key`          |
| info     | Info and stats about the server | `info`             |
| keys     | List known keys                 | `keys [pattern]`   |
| ping     | Test connection                 | `ping`             |
| set      | Create value                    | `set key value`    |

Note that not all commands have a 1:1 with redis' CLI. There are some additional
options that are not implemented.

### Future Commands

Below outlines the commands that would be nice to add to round out the basic
functionality.

| Command  | Description                       | Example             |
|----------|-----------------------------------|---------------------|
| incr     | Increment key value               | `incr key`          |
| mget     | Get multiple keys                 | `mget key [key...]` |

The following would add for quick restore functionality:

| Command  | Description                       | Example            |
|----------|-----------------------------------|--------------------|
| dump     | Serialize value to stdout         | `dump key`         |
| restore  | Restore key from serialized value | `restore key`      |

The following are all related to TTL of a value:

| Command  | Description                   | Example              |
|----------|-------------------------------|----------------------|
| expire   | Set TTL on key                | `expire key seconds` |
| persist  | Remove TTL from key           | `persist key`        |
| ttl      | Remaining time to TTL for key | `ttl key`            |

After this, adding additional types, databases, and persistence would come next.

## Support

If you find a bug, have a question, or ideas for improvements please file an
issue on GitHub.
