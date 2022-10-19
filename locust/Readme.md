# Locust - load testing tool
[more info](https://locust.io/)

### Settings to run locust
You can take a lookfor all locust settings [more info](https://docs.locust.io/en/stable/configuration.html), but this are the one that I'm using:    
- `-f` => file to run
- `--headless` => run without ui
- `--users` => count of users
- `--spawn-rate` => Rate to spawn users at (users per second)
- `-t` => seconds to run
- `-H` => Host to load test

```bash
locust -f api-benchmark.py --headless --users 100 --spawn-rate 10 -t 60s -H http://api:3000
```