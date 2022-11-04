HTTP1 (QUICK eventually)

Redis - Cache
PostgreSQL - DB
Messaging - Not needed, iteration 2 setup dummy message handoff perhaps

### User initiates workflow

A user queries seatmap with request body:

- Flight Route - Example: NYC -> SFO
- Carrier - Example: Delta
- Timestamptz - Example: <current timestamp>

### Server response

- Seatmap

### User sends request for specific seat

Same as initiate workflow but this time

- Row Number
- Seat Letter
- User ID

### Server validates response

Query Cache,

- If cache found skip DB read
- If no cache, load seat map to cache

If seat is free

- assign to user
- update cache
- update db

If seat is taken

- return error