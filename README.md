A Rust client-server application designed as a foundation for exploring networking, concurrency, and database systems. The application creates a local TCP server, supports multiple concurrent clients, and provides commands for reading and modifying data stored in a SQLite database.
STEPS TO LAUNCH:
1. Download the project
2. Open two instances of terminal
3. Navigate the terminal to the "src" folder in the directory
4. In one instance, run "cargo run --bin server"
5. On the other, run "cargo run--bin client" (This server can handle several clients at once; however, be careful with resource allocation, as there is NOT a thread limit/pool)

Current Commands:

"Hello" returns Hello

"Exit" terminates the client process

"shutdown" Shuts down server

"set" <str1> <str2> Adds a new string and corresponding value to the database under the next ID available

"get" <strname> Gets a specific name's value from the database
