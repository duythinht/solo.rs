### solo.rs
Prevents an application from running twice. This is useful when you launch your task thanks to `cron`
Inspired by Perl [solo](http://github.com/timkay/solo) and Ruby version [rolo](http://github.com/icy/rolo), but using pid file instead of open a socket

### Build by cargo

```
$ cargo build --release
$ cp ./target/release/solo /usr/local/bin/
```

### Example usage

```
$ solo --pid=path_to_file.pid -- /usr/bin/long_running_backup_db.sh -some-param
```

