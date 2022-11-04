# Reservation Demo Microservice

This is intended to showcase a variety of backend development features that might be useful in real world scenarios.

I took a stylistic approach to solving this problem, in theory, this is an event driven microservice and at small to
medium loads a service such as AWS Lambda or GCP Cloud Run would potentially make more sense and integrate more clean
with cloud native services.

I choose to make a more platform and language agnostic approach, to have a service that is integrated with Kubernetes,
AWS, and other services to showcase what you would run into more commonly in the wild.

Redis - Cache
PostgreSQL - DB
Messaging - Not needed, iteration 2 setup dummy message handoff perhaps

### User initiates workflow

A user queries seat map with request body:

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