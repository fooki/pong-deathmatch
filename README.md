# pong-deathmatch
Pong.. to the death!


# Server

Running the server:
```
cargo run -- -s --addr 127.0.0.1:5555
```
The server needs to be run before the clients. It has three different states.

![server-states](https://github.com/fooki/pong-deathmatch/blob/master/images/server-states.jpg?raw=true)

For now, it won't bounce back from a client timeouts that reconnects.

# Clients
Clients are run using the command:
```
cargo run -- -a 127.0.0.1:5555
```

You can also run cpu clients:

```
cargo run -- --cpu -a 127.0.0.1:5555
```

# Server Client Communication
Communication is done via a semi-reliable UDP library called [Laminar](https://github.com/amethyst/laminar). It works as follows:
- The server pings clients periodically and clients pong back, to maintain a "connection" between them. If the clients are too slow to respond, they will be considered timed out and disconnected.
- When two clients are connected and a game is running, the server will broadcast out the current state of the game periodically.
- Clients will extrapolate between broadcasts and guess how the game updates. This is not tricky as they know their own paddle position at all times and its easy to extrapolate the ball movement between paddles. The only information a client really needs quickly is when the opponent moves their paddle.
- Whenever a client wants to move a paddle, they'll send a move msg to the server. When they stop sending it, their paddle won't move any further. This isn't very efficient as it requires a lot of messages to be sent. A better way of doing this would be to send deltas, e g. "Now I started moving" -> "Now I stopped moving".

At this moment Laminar (the network library this game uses) almost reimplements the tcp protocol without handshakes or congestion control. Its possible to make Laminar less reliable and thus get better performance, but that would require more handshakes during the phases where clients connect.

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
