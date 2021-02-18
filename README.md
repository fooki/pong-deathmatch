# pong-deathmatch
Pong.. to the death!


# Server
The server needs to be run before the clients. It has three different states.

![server-states](https://github.com/fooki/pong-deathmatch/blob/master/images/server-states.jpg?raw=true)

For now, it won't bounce back from a client timeouts that reconnects.

## TODO

### Pong game
- Add different bounces depending on where on the paddle the ball hit.
- Keep scores.

### Networking
- Allow adding delays/packet drops and duplicates for better testing.
- Start measuring network communication in order to make informed decisions.
- Run server networking in a separate thread from game update loop.
- Use laminar heartbeats instead of pinging manually.
- Serialization is bloated, Could send thinner custom messages.
- Send deltas instead of absolute state. The only game state that constantly
  needs to be sent to players is when one of them changes movement. Everything
  else can be extrapolated most of the time.
- Allow game to be paused when timed out.
- Allow clients to try to periodically reconnect when disconnected.

### GGEZ
- Look into the performance, no consideration has been made.

### General Rust improvements
- Find a way to map errors to own custom errors better.
- Don't leak laminar errors beyond networking layer.
- Avoid sprinkling expects/unwraps :).
- More testing.
