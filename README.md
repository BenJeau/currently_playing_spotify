# spotify-currently-playing

Simple Rust HTTP proxy server using Actix to know what track the specified user is currently listening. Server caching is used keeping the latest song in memory to not overload the Spotify's REST API.

## Usage

Call the binary with the required parameters to the binary, to learn more about the parameters use `--help` or `-h` or look at the table below. 

| Parameter name    | Environment name        | Required              | Description                                                                                                                            |
| ----------------- | ----------------------- | --------------------- | -------------------------------------------------------------------------------------------------------------------------------------- |
| `--interval`      | `INTERVAL_QUERY_SECS`   | No (defaults to `1`) | Maximum interval in seconds which the Spotify API will be called. Requests made during the interval will return the cached result.     |
| `--auth-code`     | `SPOTIFY_AUTH_CODE`     | Yes                   | Authentication code from the Spotify user taken from the Authentication authentication flow (learn more [below](#authentication-code)) |
| `--client-id`     | `SPOTIFY_CLIENT_ID`     | Yes                   | Spotify application client id (learn more [below](#client-id-and-secret))                                                              |
| `--client-secret` | `SPOTIFY_CLIENT_SECRET` | Yes                   | Spotify application client secret (learn more [below](#client-id-and-secret))                                                          |

## Developing

Have rust installed and run `cargo build` and run the corresponding binary in the `./target` folder with the appropriate parameters.

## Spotify Credentials

### Client ID and Secret

1. Create a new application in the [Spotify's dashboard](https://developer.spotify.com/dashboard/)
2. The client id and client secret is available on the dashboard

### Authentication Code

The following steps are what is described in the [Spotify Authorization Flow](https://developer.spotify.com/documentation/general/guides/authorization/code-flow/) and assumes you already created a Spotify application.

1. Add `http://localhost:8888/callback` as a Redirect URI in the settings
2. Go to the following website (must replace `SPOTIFY_CLIENT_ID` with your own Spotify application client id)
  * https://accounts.spotify.com/authorize?scope=user-read-recently-played&response_type=code&redirect_uri=http://localhost:8888/callback&client_id=SPOTIFY_CLIENT_ID
3. Extract the authentication code from the URL (what follows `?code=` from the URL response, such as http://localhost:8888/callback?code=AQA_F-eO8V...)
