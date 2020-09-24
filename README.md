# ssh password checker
## Overview
This is a CLI tool that checks if the target server has disabled SSH password authentication.

## dependencies
- OpenSSH
  - execute `ssh` command in this tool

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
{"status":true,"result":"[INFO] no problem!"}

// when `PasswordAuthentication yes` on server side
$ ./ssh-password-checker 192.0.2.3
{"status":false,"result":"[WARN] password authentication is enable"}

// when target port is specified
$ ./ssh-password-checker 192.0.2.3 22
{"status":false,"result":"[WARN] password authentication is enable"}

// when ssh connection fails
$ ./ssh-password-checker 192.0.2.4 10022
{"status":true,"result":"[INFO] ssh couldn't connect to this host"}
```
