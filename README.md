# pg-auto-backup

> Disclaimer: This is a learning project, use at your own risk.

Backup PostgreSQL databases by running `pg_dump` and save the output to local filesystem or S3.

## Usage

### Binary

Copy and modify the [default configuration file](./example/default.toml)

You can pass the configuration file path as an argument or set the `PG_AUTO_BACKUP_CONFIG` environment variable.

```sh
PG_AUTO_BACKUP_CONFIG=/path/to/config.toml ./pg-auto-backup     # set in environment variable
./pg-auto-backup -c /path/to/config.toml                        # pass as argument
```

If none of the above is set, the program will create a default configuration file at the default location set by the [ProjectDirs.config_dir()](https://docs.rs/directories/latest/directories/struct.ProjectDirs.html#method.config_dir)

### Container

Mount the flowwing volumes:

- `/app/config/config.toml` - the configuration file
- `/var/lib/pg-auto-backup` - the backup files if using local storage

The prebuilt image contains PostgreSQL 14/15/16 client tools.

- `/usr/bin/pg_dump` symlink to `/usr/libexec/postgresql/pg_dump` for PostgreSQL 16
- `/usr/libexec/postgresql/pg_dump` symlink to `/usr/libexec/postgresql16/pg_dump` for PostgreSQL 16
- `/usr/libexec/postgresql14/pg_dump` for PostgreSQL 14
- `/usr/libexec/postgresql15/pg_dump` for PostgreSQL 15
- `/usr/libexec/postgresql16/pg_dump` for PostgreSQL 16

```sh
docker run -v /path/to/config.toml:/app/config/config.toml -v /path/to/backup:/var/lib/pg-auto-backup ghcr.io/liamsho/pg-auto-backup:latest
```

## License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details
