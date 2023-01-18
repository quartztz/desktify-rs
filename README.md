# `desktify`: a somewhat overcomplicated app for a somewhat simple job.

as the description implies.

opens a window on your desktop, showing the album art for the currently playing song. if the song changes and i've done my job right, the album art will react accordingly. 

## setup and use

### build and setup

you need a Spotify developer account, a valid project (should be straightforward enough to create), and the following environment variables set: 
 - `SPOTIPY_CLIENT_ID`
 - `SPOTIPY_CLIENT_SECRET`
 - `SPOTIPY_REDIRECT_URI`
in accordance to what they are set to in your account.

make sure to compile before running: 
```bash
cargo build --release
```

dependencies: 
 - `spotipy`
 - `json`

### usage
simply run `./run` with an optional argument for size: 

```bash
./run <size: {int or empty}>
```

default size is 400x400px, non resizable while running unfortunately. to stop, simply press `q` or `Esc` while the window is in focus, or hit `ctrl-c` while the terminal you ran from is in focus. 

## TODOS

 - [x] find a way to kill the python processes on exit.
 - [ ] make it less complicated.
 - [x] make the window resizable (implemented-ish).
 - [ ] cache images to reduce bandwidth usage (use file/album name as key?). 
