# Description
A small learning exercies with async rust - Very simple implementation of quote server.

# Data format
Add a jsonl file called `quotes.json` to the top level project dir with the following format:
```json
    {"quote":"Be yourself; everyone else is already taken.","author":"Oscar Wilde"}
```

# Example
```
> nc -t 127.0.0.1 9393
“I'm selfish, impatient and a little insecure. I make mistakes, I am out of control and at times hard to handle. But if you can't handle me at my worst, then you sure as hell don't deserve me at my best.” - Marilyn Monroe
```

