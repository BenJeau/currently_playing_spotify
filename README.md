# currently_playing_spotify

Rust WebSocket proxy server using [axum](https://crates.io/crates/axum) relaying Spotify tracks that are currently being played by the logged in user. WebSocket messages are only sent if the track changes or the state has changed (playing or paused). A background job periodically queries the API and relays the information to all users connected.

Once the server is running, you can connect to it using a WebSocket via the `/ws` endpoint on the `8080` port. The server will send a message to the client when the track changes or the state changes (playing or paused). The message is a JSON object with the following fields (where data is `null` if the user is not playing anything or is an object from the [Currently Playing Spotify API](https://developer.spotify.com/documentation/web-api/reference/#/operations/get-the-users-currently-playing-track) if compact is `false`, or if compact is `true` it is a subset of the fields from the Spotify API):

```json
{
  "data": null,
  "fetched": "2021-12-26T17:23:38.412067Z"
}
```

## Usage

Call the binary with the required parameters to the binary, to learn more about the parameters use `--help` or `-h` or look at the table below. 

| Parameter name       | Environment name      | Required | Default   | Description                                                                |
| -------------------- | --------------------- | -------- | --------- | -------------------------------------------------------------------------- |
| `--username` or `-u` | `SPOTIFY_USERNAME`    | Yes      | -         | Spotify account username                                                   |
| `--password` or `-p` | `SPOTIFY_PASSWORD`    | Yes      | -         | Spotify account password                                                   |
| `--interval` or `-i` | `INTERVAL_QUERY_SECS` | No       | `1`       | Maximum interval in seconds which the Spotify API will be called           |
| `--port`             | `WEBSOCKET_PORT`      | No       | `8080`    | WebSocket server port                                                      |
| `--address` or `-a`  | `WEBSOCKET_ADDRESS`   | No       | `0.0.0.0` | WebSocket server address                                                   |
| `--cors-origin`      | `CORS_ORIGIN`         | No       | `*`       | Set a single allow origin target, permissive if nothing is passed          |
| `--compact` or `-c`  | `COMPACT`             | No       | `false`   | Compacts the JSON response (removes many fields from the Spotify response) |

## Developing

Have rust installed and run `cargo run` with the appropriate parameters or environment variables (e.g. `cargo run -- --username BenJeau...`).

## Download

Get it from GitHub releases or use curl from the terminal (and replace `VERSION` with the appropriate version):

```sh
curl -L https://github.com/BenJeau/currently_playing_spotify/releases/download/VERSION/currently_playing_spotify --output ./currently_playing_spotify
chmod +x ./currently_playing_spotify
```
