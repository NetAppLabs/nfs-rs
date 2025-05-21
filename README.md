# NFS-RS

A pure Rust implementation of the NFSv3 protocol for use in User Space applications.

## Justification

The [libnfs](https://github.com/cholcombe973/libnfs) Rust bindings project provides
support for the NFS protocol for most scenarios.  However, its reliance on the C
implementation of the NFS protocol by using
[libnfs](https://github.com/sahlberg/libnfs), leads to issues when the
target platform is not supported by the C implementation, as well as, there being
added headaches for anyone trying to cross-compile for a different target OS and/or
platform.

By having an implementation of the NFS protocol written purely in Rust, the issues
listed above no longer apply as well as it provides a more flexible foundation.

## Usage

Function `nfs_rs::parse_url_and_mount` takes in a URL and returns a `Mount` trait
which defines the procedures that can be performed on an NFS mount.

```rust
let mount = nfs_rs::parse_url_and_mount("nfs://127.0.0.1/some/export").unwrap();
if let Some(fh) = mount.create_path("nfs-rs.txt", 0o664).ok() {
    let contents = "hello rust".as_bytes().to_vec();
    let num_bytes_written = mount.write(&fh, 0, &contents).unwrap_or_default();
    assert_eq!(num_bytes_written as usize, contents.len());
    let bytes_read = mount.read(&fh, 0, 16).unwrap_or_default();
    assert_eq!(&bytes_read, &contents);
}
```

## URL format

URL format follows [libnfs](https://github.com/sahlberg/libnfs) where
the URL format supported is:

`nfs://<server|ipv4|ipv6>[:<port>]/path[?arg=val[&arg=val]*]`.

Arguments supported are:
* `uid=<int>`
  * UID value to use when talking to the server.  Defaults to 65534 on Windows and
  getuid() on unixen.
* `gid=<int>`
  * GID value to use when talking to the server.  Defaults to 65534 on Windows and
  getgid() on unixen.
* `version=<3|4|4.1>`
  * NFS Version.  Can be specified as comma separated list in descending order of
  preference, e.g. `4.1,4,3`, with first supported one being used.  Default is 3.
  * Note: versions `4` and `4.1` accepted in URL but not yet supported.
* `nfsport=<port>`
  * Use this port for NFS instead of using the portmapper or port specified in
  host part of URL.
* `mountport=<port>`
  * Use this port for the MOUNT protocol instead of using portmapper. Ignored for
  NFSv4 as it does not use the MOUNT protocol.
* `readdir-buffer=<count> | readdir-buffer=<dircount>,<maxcount>`
  * Set the buffer size for READDIRPLUS, where dircount is the maximum amount of
  bytes the server should use to retrieve the entry names and maxcount is the
  maximum size of the response buffer (including attributes).  If only one <count>
  is given it will be used for both.  Default is 8192 for both.

## Limitations

Initial implementation only provides NFSv3 client functionality - essentially it is
similar to [go-nfs-client](https://github.com/willscott/go-nfs-client) but with all
NFSv3 procedures implemented ([go-nfs-client](https://github.com/willscott/go-nfs-client) 
omits some procedures, probably due to them being rarely used features of NFS).

One thing that [go-nfs-client](https://github.com/willscott/go-nfs-client) does, 
but this rust implementation does not do, is to attempt connecting from a privileged 
port (i.e. binding source port to a port that is lower than 1024). 
This is due to rust's `std::net` crate not offering a way to do this.

## License

[Apache-2.0](LICENSE)

Disclaimer: _This is not an officially supported NetApp product._

## Contributing

See [Contributing.md](./CONTRIBUTING.md)