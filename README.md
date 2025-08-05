Rust MT comms latency
=====================

```
Tokio Broadcast - Time elapsed: 1.076434353s, per item (avg): 1.076434ms
Tokio MPSC - Time elapsed: 1.08199787s, per item (avg): 1.081997ms
```

```
Crossbeam - Time elapsed: 52.820034ms, per item (avg): 52.82µs
```

```
Glommio - Time elapsed: 4.673674ms, per item (avg): 4.673µs
Glommio broadcast - Time elapsed: 14.03736ms, per item (avg): 14.037µs
Glommio shared channels - Time elapsed: 12.646136ms, per item (avg): 12.646µs
```
