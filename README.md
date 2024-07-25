# herpy-bench

Herpy API Gateway Benchmark

### Benchmark

- Mac Studio 2022
- Chip Apple M1 Max
- Memory 32 GB

### LoadTest without API Gateway

```shell
rewrk -h http://127.0.0.1:8000/v1/hello -t 12 -c 100 -d 60s

Benchmarking 100 connections @ http://127.0.0.1:8000/v1/hello for 1 minute(s)
  Latencies:
    Avg      Stdev    Min      Max      
    0.67ms   1.52ms   0.05ms   124.63ms  
  Requests:
    Total: 8890025 Req/Sec: 148165.69
  Transfer:
    Total: 1.08 GB Transfer Rate: 18.51 MB/Sec
```

### LoadTest Herpy API Gateway

```shell
rewrk -h http://127.0.0.1:8080/hello -t 12 -c 100 -d 60s

Benchmarking 100 connections @ http://127.0.0.1:8080/hello for 1 minute(s)
  Latencies:
    Avg      Stdev    Min      Max      
    0.63ms   0.19ms   0.02ms   11.46ms  
  Requests:
    Total: 9528463 Req/Sec: 158806.60
  Transfer:
    Total: 826.92 MB Transfer Rate: 13.78 MB/Sec
```

### LoadTest KrakenD API Gateway

```shell
rewrk -h http://127.0.0.1:8090/hello -t 12 -c 100 -d 60s

  Latencies:
    Avg      Stdev    Min      Max      
    1.11ms   0.92ms   0.02ms   48.48ms  
  Requests:
    Total: 5427454 Req/Sec: 90456.93
  Transfer:
    Total: 936.86 MB Transfer Rate: 15.61 MB/Sec
```