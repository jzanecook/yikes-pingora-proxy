# Yikes' Pingora Proxy

This is a simple load balancer built on Pingora. You specify the sources, the SNI (Hostname of the site) and your host/port and let it run.

To use it, clone this repo and run `cargo run -- -h` to see a list of the commands.

I didn't have the internal infrastructure Yikes has to test with, so I made it possible to specify anything.

### For Yikes' Example
- Let's say you have a service running on XXX.XXX.XXX.187 on port 12300, and you want to point sub.domain.com to that IP.
- On the server that has the IP you pointed the sub.domain.com to, run this proxy with `cargo run -- -c conf.yaml -s XXX.XXX.XXX.187:12300 -S sub.domain.com`
- By default, the proxy will start on 

### Additional Note
- I recommend prefixing the commands with `RUST_LOG=INFO` so you can see the info output.
- Pingora has a daemon mode I want to try out.