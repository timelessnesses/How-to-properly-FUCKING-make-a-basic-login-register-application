# How to properly FUCKING make a basic login/register application

This repository exists because how many people actually fucked up the login/registration part of their application, written in Safe FUCKING Rust.  

## How do I run this?

You need PostgreSQL.  

1. Create `.env` with contents of

```sh
DB_HOST=localhost # database hostname or ip address
DB_NAME=testery # database name
DB_USER=testery # database username for login
DB_PASS=testery # database user's password for loginn
DB_PORT=5432 # database host's port
```

2. Run the executable

## FAQs

Q: Why not accepting hashed password from client?  
A: Because it's same thing as sending password in plain text  
  
Q: Why Rust?  
A: Why not?  
  
Q: Why can't I just store the password in plain text?  
A: Because when your system got hacked and leaked password, hackers will easily be able to steal the password, so hashing it makes it harder to crack  
  
Q: Why salting?  
A: Salting password when hashing makes it even harder to crack and prevented hackers from using [Rainbow Table](https://en.wikipedia.org/wiki/Rainbow_table)  
  
Q: Can't I just embed database connection to my executable and give it to my users?
A: No matter how many encryption to put on the database credential, it will be cracked. So it's best to just make an API for it
  
Q: What makes you mad about this?
A: [https://www.youtube.com/watch?v=fA9_KzJSrNQ](https://www.youtube.com/watch?v=fA9_KzJSrNQ)  
  
Q: Why shouldn't I send separate message for `Invalid Password` or `User not found`
A: So you wouldn't give hints to hackers who is bruteforcing random usernames and passwords.
