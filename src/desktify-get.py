#!/usr/bin/python3

import sys
from time import sleep
import requests
import spotipy
import spotipy.util as util

import os

PATH_TO_IMAGE = "./assets/album_img.png"
PATH_TO_INFO  = "./assets/info.txt"

def write_info(info): 
  with open(PATH_TO_INFO, "w+") as f: 
    f.write(info)

def write_img(image): 
  with open(PATH_TO_IMAGE, "wb+") as f: 
    f.write(image)

def get_and_write_to_file(song): 
  # print(song)
  artists = [artist['name'] for artist in song['item']['artists']]
  title = song['item']['name']
  print(f"new song: {title} by {', '.join(artists)}!")
  loc_url = song['item']['album']['images'][0]['url']
  url = loc_url

  image = requests.get(url)

  write_info(title) 
  write_img(image.content)

def refresh_token(spot, token):
  r_token = token['refresh_token']
  new_token = spot.refresh_access_token(r_token)
  return spotipy.Spotify(auth=new_token['access_token'])

def main(): 
  scope = 'user-read-currently-playing'

  user = os.environ.get('SPOTIFY_USERNAME')
  client_id = os.environ.get('SPOTIPY_CLIENT_ID')
  client_secret = os.environ.get('SPOTIPY_CLIENT_SECRET')
  token = util.prompt_for_user_token(username=user, scope=scope, client_id=client_id, client_secret=client_secret, redirect_uri="http://127.0.0.1:8888/callback")

  old = None
      
  if token: 
    spotify = spotipy.Spotify(auth=token)
    print("running properly! listening to your music")
    while True: 
      
      try:  
        current = spotify.currently_playing() 

        if old == None or (current != None and current["item"]["album"]["name"] != old["item"]["album"]["name"]):
          old = current
          get_and_write_to_file(current)
      
        sleep(2)

      except spotipy.client.SpotifyException:
        refresh_token(spotify, token)

    
  else: 
    print("invalid token")
    sys.exit()

if __name__ == "__main__": 
  main()
