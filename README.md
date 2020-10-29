# ssh password checker
## Overview
This is a CLI tool that checks if the target server has disabled SSH password authentication.

## dependencies
- OpenSSH
  - execute `ssh` command in this tool

## build (for linux)
### using container
```
mkdir -p target/release
docker build -t ssh-checker .
docker run -v $(pwd)/target/release:/workdir/target/release ssh-checker
```

## how to use
```
ssh-password-checker ${target_serve_ip} ${target_server_port}
```
- `target_server_ip` (required)
  - ssh connection address(IPv4)
- `target_server_port` (optional)
  - ssh connection port
  - default: 22

## examples
```
// when `PasswordAuthentication no` on server side
$ ./ssh-password-checker 192.0.2.2
{"message":"no problem!","result_code":100}

// when `PasswordAuthentication yes` on server side
$ ./ssh-password-checker 192.0.2.3
{"message":"password authentication is enable!","result_code":300}
{"status":false,"result":"[WARN] password authentication is enable"}

// when target port is specified
$ ./ssh-password-checker 192.0.2.3 22
{"message":"password authentication is enable!","result_code":300}

// when ssh connection fails
$ ./ssh-password-checker 192.0.2.4 10022
{"message":"ssh couldn't connect to this host","result_code":100}
```
