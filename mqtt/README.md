# MQTT Example
This is an example for a chat application using mqtt.
The code was adapted from this [example](https://github.com/eclipse/paho.mqtt.rust/blob/master/examples/mqttrs_chat.rs).

## Requirements
- openssl
- cmake
- docker

## How to run
First you need a local MQTT broker.
I recommend using the [eclipse-mosquitto docker image](https://hub.docker.com/_/eclipse-mosquitto/):
```
docker pull eclipse-mosquitto
docker run -it -p 1883:1883 -p 9001:9001 eclipse-mosquitto
```
Then you can run the client:
```
cargo run
```
Run multiple instances of the client in seperate terminal tabs to chat.