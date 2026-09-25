This project functions as a testing ground for me to begin work on bigger projects. The project allows you to create and connect to a local server, as well as alter a database using rusqlite.
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
