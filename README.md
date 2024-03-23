# herpy-bench

Herpy API Gateway Benchmark

### LoadTest without API Gateway

```shell
rewrk -h http://127.0.0.1:8000/v1/hello -t 12 -c 100 -d 60s

Benchmarking 100 connections @ http://127.0.0.1:8000/v1/hello for 1 minute(s)
  Latencies:
    Avg      Stdev    Min      Max
    1.96ms   1.51ms   0.03ms   39.83ms
  Requests:
    Total: 3053635 Req/Sec: 50894.22
  Transfer:
    Total: 381.49 MB Transfer Rate: 6.36 MB/Sec
```

### LoadTest Herpy API Gateway

```shell
rewrk -h http://127.0.0.1:8080/hello -t 12 -c 100 -d 60s

Benchmarking 100 connections @ http://127.0.0.1:8080/hello for 1 minute(s)
  Latencies:
    Avg      Stdev    Min      Max
    3.03ms   1.88ms   0.11ms   33.97ms
  Requests:
    Total: 1978253 Req/Sec: 32971.05
  Transfer:
    Total: 186.77 MB Transfer Rate: 3.11 MB/Sec
```

### LoadTest KrakenD API Gateway

```shell
rewrk -h http://127.0.0.1:8090/hello -t 12 -c 100 -d 60s

Benchmarking 100 connections @ http://127.0.0.1:8090/hello for 1 minute(s)
  Latencies:
    Avg      Stdev    Min      Max
    3.90ms   1.62ms   0.06ms   65.20ms
  Requests:
    Total: 1539334 Req/Sec: 25656.21
  Transfer:
    Total: 344.99 MB Transfer Rate: 5.75 MB/Sec
```