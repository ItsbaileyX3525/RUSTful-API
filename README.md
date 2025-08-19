# The RUSTful API

The rustful api is a REST api created in rust (doy) where you can retrieve or upload quotes to the webserver, create shortened URLs and have a nice little pastebin!

## Utilising the power of the API

To use the API you can either use the webpage which has all the information you need to start. Like literally everything you dont even need to read anything on this README or you can read this README, I recommend reading the information on the SWAGGER site as it is most likely better explained than it is here!

### Using the post feature on Swagger

By navigating to the website you will find a few options that say
"POST" /quotes
"GET" /quotes
"GET" /quote
"GET" /quotes/{id}

if you click on the post card it will create a drop-down with information on how you can create a request to upload your quote.
The website gives you a basic outline of what you can pass to the curl request that the server makes to itself, the default should be

`{
  "text": "test",
  "speaker": "me"
}`

Where "text" is the quote you want to upload and "speaker" is the person that said the quote.

After filling out the information that you want to upload just hit the "Execute" button and the server will make the POST request to itself, you can tell if its work as right below on the "Responses" section it will show the Curl request it ran with your input and right underneath, the response from the server, 201 or 200 meaning it uploaded successfully!

If you don't make a post request the "Responses" section will tell you what data should be expected when you make the POST request!

### Using the GET /quotes feature

Navigate to the GET /quotes tab and give it a click, as this is a GET request you don't need to provide any arguments like the previous one, just hit "Execute" and the server will make a request to itself and display all quotes on the server!

You should, again, see the Curl request that it made and where it made it to. You should see "Server response" which means the server has retrieved the request and the response body is what it sent back!

Ignoring all that you can see what you could expect to see if hadn't pressed "Execute" to begin with

### Using the GET /quote feature

All the same steps as the previous GET except this time navigate to
"GET" /quote and click execute.

### using the GET /quote/{id} feature

Despite this being a GET request it does have an argument you must pass for the server to process it fully. By passing the id of the quote you want to see, ("66017dc4-ec9e-4db4-bba9-3bf9698da964" if you want to test it) it will return that exact quote.

After clicking "Execute" like the others you should see a server response with the Curl request it made and where it made it to, underneath you can see the JSON response from the server with your quote that you requested! (as well as the response headers)

Ignoring the server response, like all the others you can see what you would expect to see if you had run the execute button!

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

~~- [ ] Update index page
    - [ ] Create URL shortener card
    - [ ] Update styling & js~~

- [ ] Update rust server
    - [x] Handle new URLs
        - [x] Create short URL
        - [x] Store it in the database
    - [x] Redirect short URLs
    - [ ] Delete request for URLs

~~- [ ] Update index page
    - [ ] Create pastebin card
    - [ ] Update the styling & js~~

- [ ] Update rust server
    - [ ] Support for uploading text
    - [ ] Support retrieving text
    - [ ] Support deleting text