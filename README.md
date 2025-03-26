# herpy-bench

Herpy API Gateway Benchmark

### Benchmark

- Mac mini 2024
- Chip Apple M4 Pro
- Memory 24 GB

### LoadTest without API Gateway

```shell
wrk -t12 -c100 -d60s http://127.0.0.1:8000/v1/hello

Running 1m test @ http://127.0.0.1:8000/v1/hello
  12 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   582.59us  128.13us  21.90ms   97.05%
    Req/Sec    13.59k   321.06    18.57k    91.16%
  9747178 requests in 1.00m, 1.19GB read
Requests/sec: 162184.34
Transfer/sec:     20.26MB
```

### LoadTest Herpy API Gateway

```shell
wrk -t12 -c100 -d60s http://127.0.0.1:8080/hello

Running 1m test @ http://127.0.0.1:8080/hello
  12 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     1.11ms   63.59us   5.81ms   82.77%
    Req/Sec     7.21k   113.46     7.47k    79.31%
  5173684 requests in 1.00m, 646.36MB read
  Non-2xx or 3xx responses: 23
Requests/sec:  86084.10
Transfer/sec:     10.75MB
```

### LoadTest Zolly API Gateway

```shell
wrk -t12 -c100 -d60s http://127.0.0.1:8070/v1/hello

Running 1m test @ http://127.0.0.1:8070/v1/hello
  12 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     1.54ms  262.81us  41.65ms   92.11%
    Req/Sec     5.21k   269.46     9.00k    90.83%
  3739133 requests in 1.00m, 467.13MB read
Requests/sec:  62213.64
Transfer/sec:      7.77MB
```

### LoadTest KrakenD API Gateway

```shell
wrk -t12 -c100 -d60s http://127.0.0.1:8090/hello

Running 1m test @ http://127.0.0.1:8090/hello
  12 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     1.61ms  629.55us  29.75ms   79.79%
    Req/Sec     5.03k   294.94    14.10k    88.27%
  3602395 requests in 1.00m, 807.35MB read
Requests/sec:  59940.37
Transfer/sec:     13.43MB
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
