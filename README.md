Rust MT comms latency
=====================

```
Tokio Broadcast - Time elapsed: 1.072081607s, per item (avg): 1.072081ms
Tokio MPSC - Time elapsed: 1.074076264s, per item (avg): 1.074076ms
```

```
Crossbeam - Time elapsed: 647.727µs, per item (avg): 647ns
```

```
Flume - Time elapsed: 1.690443ms, per item (avg): 1.69µs
```

```
Kanal - Time elapsed: 328.035µs, per item (avg): 328ns
Kanal 2 RX - Time elapsed: 376.115µs, per item (avg): 376ns
Kanal Broadcast - Time elapsed: 331.523µs, per item (avg): 331ns
```

```
Glommio - Time elapsed: 4.901359ms, per item (avg): 4.901µs
Glommio broadcast - Time elapsed: 24.555779ms, per item (avg): 24.555µs
Glommio shared channels - Time elapsed: 11.921685ms, per item (avg): 11.921µs
```
