# The RUSTful API

The rustful api is a REST api created in rust (doy) where you can retrieve or upload quotes to the webserver, create shortened URLs and have a nice little pastebin!

## Features

### Ship 1

- [x] Create index page
    - [x] Install tailwindcss
    - [x] Write HTML
    - [x] Create quotes card
- [x] Create rust server
    - [x] Return quotes as JSON
    - [x] Return random quote
    - [x] Add new quotes
    - [x] Start server on http and https

### Ship 2

- [ ] Update index page
    - [ ] Create URL shortener card
    - [ ] Update styling & js

- [ ] Update rust server
    - [ ] Handle new URLs
        - [ ] Create short URL
        - [ ] Store it in the database
    - [ ] Redirect short URLs
    - [ ] Delete request for URLs

### Ship 3

- [ ] Update index page
    - [ ] Create pastebin card
    - [ ] Update the styling & js
- [ ] Update rust server
    - [ ] Support for uploading text
    - [ ] Support retrieving text
    - [ ] Support deleting text