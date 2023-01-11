# `desktify`: a somewhat overcomplicated app for a somewhat simple job.

as the description implies.

opens a 400x400px window on your desktop, showing the album art for the currently playing song. if the song changes and i've done my job right, the album art will react accordingly. 

## setup and use

you need a Spotify developer account, and the following environment variables set: 
 - `SPOTIPY_CLIENT_ID`
 - `SPOTIPY_CLIENT_SECRET`
 - `SPOTIPY_REDIRECT_URI`
in accordance to what they are set to in your account.

make sure to compile before running: 
```bash
cargo build --release
``
then just run `./run`. 

dependencies: 
 - `spotipy`
 - `json`

to stop, exit the application, then close the existing python3 processes. i should really find a way to stop them myself, sorry. 

## TODOS

 - [ ] find a way to kill the python processes on exit.
 - [ ] make it less complicated.
 - [ ] make the window resizable.
