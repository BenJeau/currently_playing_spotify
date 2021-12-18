# spotify-currently-playing

Super simple Rust HTTP server using Actix to know what the specified user is currently listening to. Server caching is used keeping the latest song in memory to not overload the Spotify's REST API.


| Name                    | Required              | Description                                                                                                                        |
| ----------------------- | --------------------- | ---------------------------------------------------------------------------------------------------------------------------------- |
| `INTERVAL_QUERY_SECS`   | No (defaults to `10`) | Maximum interval in seconds which the Spotify API will be called. Requests made during the interval will return the cached result. |
| `SPOTIFY_AUTH_CODE`     | Yes                   | Authentication code from the Spotify user taken from the Authentication auth flow (learn more [below](#authentication-code))  |
| `SPOTIFY_CLIENT_ID`     | Yes                   | Spotify application client id (learn more [below](#client-id-and-secret))                                                          |
| `SPOTIFY_CLIENT_SECRET` | Yes                   | Spotify application client secret (learn more [below](#client-id-and-secret))                                                      |

## Developing

Simply have rust installed and run `cargo run`

## Spotify Credentials

### Client ID and Secret

1. Create a new application in the [Spotify's dashboard](https://developer.spotify.com/dashboard/)
2. The client id and client secret is available on the dashboard

## Authentication Code

The following steps are what is described in the [Spotify Authorization Flow](https://developer.spotify.com/documentation/general/guides/authorization/code-flow/) and assumes you already created a Spotify application.

1. Add `http://localhost:8888/callback` as a Redirect URI in the settings
2. Go to the following website (must replace `SPOTIFY_CLIENT_ID` with your own Spotify application client id)
  * https://accounts.spotify.com/authorize?scope=user-read-recently-played&response_type=code&redirect_uri=http://localhost:8888/callback&client_id=SPOTIFY_CLIENT_ID
3. Extract the authentication code from the URL (what follows `?code=` from the URL response, such as http://localhost:8888/callback?code=AQA_F-eO8V...)

INTERVAL_QUERY_SECS=1 SPOTIFY_AUTH_CODE=AQAZVnxG8fBfw-OM12n0xx8YkP-Qwl6-oyYItiSuNBkUjRpntQAu7ar6KjSindprlgz6pstYYRG9PMRr-nz226TSl-kyIasN-fppS84qc3dAB6pV-TT44ZrfRXauLdkB_fo2VVECJk_1isvL-F6u8PeMFkxzRjEtkAzuSQbxrbHbJp-SWB9B9wSrrAD9wsPAE2e9xkcdMwtt8FMJuA SPOTIFY_CLIENT_ID=991385edbd4c4fb4ad8915322d1b3b37 SPOTIFY_CLIENT_SECRET=5ec356eae7304c378d8f1149fcf84a60 cargo run