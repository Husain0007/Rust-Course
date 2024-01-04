Step 1: Create a binary project `cargo new server`  
We will create a barebone server working with the HTTP anc TCP protocols.
To create a new listener we use the `bind()` function with the ip address we want to bind on and the port number. bind() returns a result type, for a production level server we would have to handle the error gracefully, however in this setting we will directly proceed to `unwrap()`. Calling the `incoming()` function on `listener` will return an iterator, each value will be of type `Result<TcpStream, Error>` ie; type result that either contains a TcpStream element or an error.   
We will use `shadowing` to create another variable called stream which recieves the `stream.unwrap()` value.
Step 2: Run the program: `cargo run`  
Step 3: Read data from `TcpStream`  
Create a function called `handleConnection()` which will take a mutable `TCPStream` as input.
Step 4: Creating a response for the GET requests.
```
        HTTP-Version Status-Code Reason-Phrase CRLF  
        headers CRLF  
        message-body  
        example: HTTP/1.1 200 OK\r\n\r\n 
        Above example without headers and message body
```

Step 5: Now multithreading.
The `/sleep` page is used to demonstrate the need for multithreading, ie; a slow process backs up other requests to the server. This can be solved by using a fixed size thread pool. We don't want someone to be able to take down our server by sending many requests causing the system to spin up multiple threads, thats' why we use a fixed number of threads.


Steps of development:
Learn a bit about TCP and HTTP.
Listen for TCP connections on a socket.
Parse a small number of HTTP requests.
Create a proper HTTP response.
Improve the throughput of our server with a thread pool.
