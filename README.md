# URL Shortener in Rust

A simple URL shortener built with Rust using the Warp web framework. This project provides both a web server to shorten URLs and redirect to the original URLs, as well as a command-line interface to shorten URLs directly.

## Features

- Shorten URLs via HTTP POST requests
- Redirect to original URLs via short codes
- Shorten URLs via command-line interface
- In-memory storage of shortened URLs

## Requirements

- Rust 1.39 or higher

## Usage

### Running as a Web Server

1. Clone the repository:

   ```bash
   git clone https://github.com/username/url_shortener.git
   cd url_shortener
2. Compile and run the server
   ```bash
   cargo run
3. Shorten a URL by sending a POST request to http://127.0.0.1:3030/shorten with the URL in the JSON body:
   ```bash
    curl -X POST -H "Content-Type: application/json" -d '{"url": "https://www.example.com"} http://127.0.0.1:3030/shorten

4. Access the original URL by navigating to http://127.0.0.1:3030/<short_code> in your web browser, replacing <short_code> with the code returned in the previous step.

### Shortening URLs from the Command Line

You can also shorten a URL directly from the command line:

  `cargo run https://www.example.com`


This will print the original URL and the shortened URL to the console.

### Screenshot
![image](https://github.com/shivam2003-dev/URL-Shortener/assets/78433942/53c59e56-1289-4d1d-bfa8-6630d0929bc1)



### Limitations
- The shortened URLs are stored in memory, so they will be lost when the server is stopped.
- The command-line interface only shortens URLs and does not provide a way to access the original URLs.

### Contributing
Feel free to fork the project, create a feature branch, and send us a pull request!



