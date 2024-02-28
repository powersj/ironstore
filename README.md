# IronStore

*Dirt simple Redis in Rust*

## Overview

The goal of this project was to give myself a larger Rust project to play with.

## Server

To build the server run:

```bash
make server
```

To run the server create a basic configuration and run:

```bash
$ cat config.conf
port = 1234

$ ./redis-server config.conf
```

## Client

To build the client:

```bash
make cli
```

Below outlines the various commands supported by the client:

| Command | Description  | Example         |
|---------|--------------|-----------------|
| set     | Create value | `set key value` |
| get     | Get value    | `get key` |
| del     | Delete value | `del key` |

### Future Commands

Below outlines the commands that would be nice to add to round out the basic
functionality.

These first few provide some additional helpers to make using the store easier:

| Command  | Description                     | Example            |
|----------|---------------------------------|--------------------|
| flushall | Delete all keys and databases   | `flushall [async]` |
| info     | Info and stats about the server | `info`             |
| keys     | List known keys                 | `keys [pattern]`   |
| ping     | Test connection                 | `ping`             |

These would be a second phase of additional helpers around modifying and
interacting with keys and values:

| Command  | Description                       | Example            |
|----------|-----------------------------------|--------------------|
| append   | Append to existing value          | `append key value` |
| dump     | Serialize value to stdout         | `dump key`         |
| exists   | Does a key exist                  | `exists key`       |
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
