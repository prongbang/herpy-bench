# herpy-bench

Herpy API Gateway Benchmark

### Benchmark

- Mac Studio 2022
- Chip Apple M1 Max
- Memory 32 GB

### LoadTest without API Gateway

```shell
wrk -t12 -c100 -d60s http://127.0.0.1:8000/v1/hello

Running 1m test @ http://127.0.0.1:8000/v1/hello
  12 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     5.51ms   24.68ms 395.91ms   95.49%
    Req/Sec    13.44k     8.87k  211.81k    89.36%
  9403288 requests in 1.00m, 1.15GB read
Requests/sec: 156474.47
Transfer/sec:     19.55MB
```

### LoadTest Herpy API Gateway

```shell
wrk -t12 -c100 -d60s http://127.0.0.1:8080/hello

Running 1m test @ http://127.0.0.1:8080/hello
  12 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     2.14ms    8.55ms 272.67ms   98.09%
    Req/Sec     6.85k     1.14k   12.84k    83.34%
  4916822 requests in 1.00m, 614.27MB read
Requests/sec:  81811.14
Transfer/sec:     10.22MB
```

### LoadTest Zolly API Gateway

```shell
wrk -t12 -c100 -d60s http://127.0.0.1:8070/v1/hello

Running 1m test @ http://127.0.0.1:8070/v1/hello
  12 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     2.44ms    8.68ms 253.62ms   97.28%
    Req/Sec     6.33k     1.44k   17.85k    79.91%
  4545014 requests in 1.00m, 567.81MB read
Requests/sec:  75621.61
Transfer/sec:      9.45MB
```

### LoadTest KrakenD API Gateway

```shell
wrk -t12 -c100 -d60s http://127.0.0.1:8090/hello

Running 1m test @ http://127.0.0.1:8090/hello
  12 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     3.03ms    5.69ms 111.44ms   96.09%
    Req/Sec     3.69k     1.00k    6.61k    75.60%
  2647991 requests in 1.00m, 593.45MB read
Requests/sec:  44109.90
Transfer/sec:      9.89MB
```

### LoadTest Nginx API Gateway

```shell
wrk -t12 -c100 -d60s http://127.0.0.1:8081/hello

Running 1m test @ http://127.0.0.1:8081/hello
  12 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     8.87ms   76.93ms   1.25s    98.63%
    Req/Sec     5.62k     1.93k    8.82k    78.35%
  1200598 requests in 1.00m, 355.27MB read
  Socket errors: connect 0, read 0, write 0, timeout 178
  Non-2xx or 3xx responses: 1168098
Requests/sec:  19978.23
Transfer/sec:      5.91MB
```