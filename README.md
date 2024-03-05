# pg-auto-backup

> Disclaimer: This is a learning project, use at your own risk.

Backup PostgreSQL databases by running `pg_dump` and save the output to local filesystem or S3.

## Usage

Copy and modify the [default configuration file](./example/default.toml)

You can pass the configuration file path as an argument or set the `PG_AUTO_BACKUP_CONFIG` environment variable.

```sh
PG_AUTO_BACKUP_CONFIG=/path/to/config.toml ./pg-auto-backup     # set in environment variable
./pg-auto-backup -c /path/to/config.toml                        # pass as argument
```

If none of the above is set, the program will create a default configuration file at the default location set by the [ProjectDirs.config_dir()](https://docs.rs/directories/latest/directories/struct.ProjectDirs.html#method.config_dir)

## License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details
