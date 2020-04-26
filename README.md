[![Build Status](http://15.188.81.183:8080/buildStatus/icon?job=avalon-rs%2Fmaster)](http://15.188.81.183:8080/job/avalon-rs/job/master/)

# avalon-rs
Avalon role game coded in Rust.
[Game Rules](https://hobbylark.com/card-games/How-to-Play-Avalon)

# Clone, run and test out code
```bash
git clone https://github.com/tul1/avalon-rs.git
cd ./avalon-rs
cargo build
cargo test
```

# Use our Avalon Docker
In case you don't want to install all the dependencies in your local machine you can run our project inside a Docker. With the commands beneath you'll be able to launch a Docker container and run try avalon-rs locally. 

```bash
docker build -t avalon-rs:latest .
docker run -it --rm avalon-rs bash
cd avalon-rs
```

A public image is also available in dockerhub, you just have to run this command to get it.

```bash
docker pull tul1/avalon-rs:latest
```
