# herpy-bench

Herpy API Gateway Benchmark

### Benchmark

- Chip Apple M1 Max
- Memory 32 GB

### LoadTest without API Gateway

```shell
rewrk -h http://127.0.0.1:8000/v1/hello -t 12 -c 100 -d 60s

Benchmarking 100 connections @ http://127.0.0.1:8000/v1/hello for 1 minute(s)
  Latencies:
    Avg      Stdev    Min      Max
    0.63ms   0.20ms   0.02ms   37.64ms
  Requests:
    Total: 9451249 Req/Sec: 157519.70
  Transfer:
    Total: 1.15 GB Transfer Rate: 19.68 MB/Sec
```

### LoadTest Herpy API Gateway

```shell
rewrk -h http://127.0.0.1:8080/hello -t 12 -c 100 -d 60s

Benchmarking 100 connections @ http://127.0.0.1:8080/hello for 1 minute(s)
  Latencies:
    Avg      Stdev    Min      Max
    1.29ms   0.27ms   0.08ms   31.11ms
  Requests:
    Total: 4658318 Req/Sec: 77640.57
  Transfer:
    Total: 581.97 MB Transfer Rate: 9.70 MB/Sec
```

### LoadTest KrakenD API Gateway

```shell
rewrk -h http://127.0.0.1:8090/hello -t 12 -c 100 -d 60s

Benchmarking 100 connections @ http://127.0.0.1:8090/hello for 1 minute(s)
  Latencies:
    Avg      Stdev    Min      Max
    3.08ms   2.24ms   0.05ms   66.17ms
  Requests:
    Total: 1949441 Req/Sec: 32490.83
  Transfer:
    Total: 436.90 MB Transfer Rate: 7.28 MB/Sec
```