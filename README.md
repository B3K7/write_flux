# write_flux client
The [Write Flux client](https://github.com/B3K7/infx_w) is a rust based alternative to the [Telegraf agent](https://github.com/influxdata/telegraf).  
   - The Write Flux client depends on a number of configuration items including the [Influxdb2 rust client](https://github.com/aprimadi/influxdb2)
      - The Influxdb2 rust client depends on the [Open-Telementry toolkit](https://github.com/open-telemetry/opentelemetry-rust); and, 
      - The Influxdb2 rust client makes use of the [Influxdb 2.0 API](https://docs.influxdata.com/influxdb/v2.0/reference/api/)
         - The Influx 2.0 API  relies on the [InfluxDB open source time series platform](https://github.com/influxdata/influxdb)

# About InfluxDb
 From an environment-technology "form/fit/function" perspective, Influx uses Log structured Merge(LSM) Trees.  
 Used properly, LSM(s) can delivery durability and high-write throughput.
 
 For more information, please consult:
  - [Ben Stopford's paper on LSM(s)](http://www.benstopford.com/2015/02/14/log-structured-merge-trees/); and,
  - [John Pradeep Vincent's article on LSM(s)](https://medium.com/swlh/log-structured-merge-trees-9c8e2bea89e8)
  - [Figure # 3 of Adam Jacobs ACM article](https://queue.acm.org/detail.cfm?id=1563874)


## build instructions
cargo build --release

## Help
$ ./write_flux --help
```
Influxdb2 point client

Usage: write_flux [OPTIONS] --target-json <TARGET_JSON> --measurement-json <MEASUREMENT_JSON>

Options:
  -t, --target-json <TARGET_JSON>            endpoint target
  -m, --measurement-json <MEASUREMENT_JSON>  influx measurements
  -v, --verbose...                           More output per occurrence
  -q, --quiet...                             Less output per occurrence
  -h, --help                                 Print help
  -V, --version                              Print version
```

## Optimization Notes

### 1st pass Core/Duty Cycle Optimization
time ./write_flux -t ./nfx.db.json -m ./measurements.1k.json
```
real    0m0.234s
user    0m0.020s
sys     0m0.007s
```

### 1st pass Network Interface Optimization
strace -r -e recvfrom,sendto ./write_flux -t ./nfx.db.json -m ./measurements.2.json
```
     0.000000 sendto(9, "\26\3\1\2\0\1\0\1\374\3\3\270\303I\266\204\225\0\234\342\265\275\307\251\30g\3\354\344J\322\344"..., 517, MSG_NOSIGNAL, NULL, 0) = 517
     0.035998 recvfrom(9, "\26\3\3\0z", 5, 0, NULL, NULL) = 5
     0.000417 recvfrom(9, "\2\0\0v\3\3\376\365\256R\353\352\365w\363\355\303Vj\37\2523\235P\247\251dd]C\342\30"..., 122, 0, NULL, NULL) = 122
     0.000765 recvfrom(9, "\24\3\3\0\1", 5, 0, NULL, NULL) = 5
     0.000309 recvfrom(9, "\1", 1, 0, NULL, NULL) = 1
     0.000271 recvfrom(9, "\27\3\3\0\33", 5, 0, NULL, NULL) = 5
     0.000099 recvfrom(9, "\370\207\333\302\335It\2573\352@`UM\177\327\210z\275\32Ad\367\362\271U\233", 27, 0, NULL, NULL) = 27
     0.000379 recvfrom(9, "\27\3\3\17\374", 5, 0, NULL, NULL) = 5
     0.000143 recvfrom(9, "Y\225G\221\234\374M^C\232\320\247\376y\332\245a\251\270\253\273\26\205c\211.\352!}\2\327y"..., 4092, 0, NULL, NULL) = 4092
     0.002014 recvfrom(9, "\27\3\3\1\31", 5, 0, NULL, NULL) = 5
     0.000163 recvfrom(9, "Y8\377B\320!\312\303\320U\221\301\255,\f\303\314\315K\3452\215\307\364\331{_\4!\20w\177"..., 281, 0, NULL, NULL) = 281
     0.000281 recvfrom(9, "\27\3\3\0E", 5, 0, NULL, NULL) = 5
     0.000106 recvfrom(9, ">\351I7\372L51\27\315\10\363F\365\22\244=\330=\342\236\27\302g\226\371\350\277o\226\246\306"..., 69, 0, NULL, NULL) = 69
     0.000299 sendto(9, "\24\3\3\0\1\1\27\3\3\0E\2668\366\320kq'#\260L\346dW\24\177\241^O\342\236\t"..., 80, MSG_NOSIGNAL, NULL, 0) = 80
     0.087497 +++ exited with 0 +++
```

### 1st pass Memory Subsystem Optimization
strace -r ./write_flux   -t ./nfx.db.json -m ./measurements.2.json 2>&1 | grep -e mmap -e brk
```
     0.000160 brk(NULL)                 = 0x556cefa79000
     0.000161 mmap(NULL, 34603, PROT_READ, MAP_PRIVATE, 3, 0) = 0x7f081119d000
     0.000016 mmap(NULL, 8192, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0x7f081119b000
     0.000024 mmap(NULL, 600368, PROT_READ, MAP_PRIVATE|MAP_DENYWRITE, 3, 0) = 0x7f0811108000
     0.000018 mmap(0x7f0811125000, 319488, PROT_READ|PROT_EXEC, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x1d000) = 0x7f0811125000
     0.000017 mmap(0x7f0811173000, 106496, PROT_READ, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x6b000) = 0x7f0811173000
     0.000029 mmap(0x7f081118e000, 53248, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x85000) = 0x7f081118e000
     0.000018 mmap(NULL, 3100624, PROT_READ, MAP_PRIVATE|MAP_DENYWRITE, 3, 0) = 0x7f0810e13000
     0.000021 mmap(0x7f0810e99000, 1736704, PROT_READ|PROT_EXEC, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x86000) = 0x7f0810e99000
     0.000022 mmap(0x7f0811041000, 593920, PROT_READ, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x22e000) = 0x7f0811041000
     0.000019 mmap(0x7f08110d2000, 204800, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x2be000) = 0x7f08110d2000
     0.000020 mmap(0x7f0811104000, 16336, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_FIXED|MAP_ANONYMOUS, -1, 0) = 0x7f0811104000
     0.000016 mmap(NULL, 103496, PROT_READ, MAP_PRIVATE|MAP_DENYWRITE, 3, 0) = 0x7f0810df9000
     0.000018 mmap(0x7f0810dfc000, 69632, PROT_READ|PROT_EXEC, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x3000) = 0x7f0810dfc000
     0.000019 mmap(0x7f0810e0d000, 16384, PROT_READ, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x14000) = 0x7f0810e0d000
     0.000018 mmap(0x7f0810e11000, 8192, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x17000) = 0x7f0810e11000
     0.000018 mmap(NULL, 136304, PROT_READ, MAP_PRIVATE|MAP_DENYWRITE, 3, 0) = 0x7f0810dd7000
     0.000017 mmap(0x7f0810ddd000, 65536, PROT_READ|PROT_EXEC, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x6000) = 0x7f0810ddd000
     0.000019 mmap(0x7f0810ded000, 24576, PROT_READ, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x16000) = 0x7f0810ded000
     0.000018 mmap(0x7f0810df3000, 8192, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x1b000) = 0x7f0810df3000
     0.000019 mmap(0x7f0810df5000, 13424, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_FIXED|MAP_ANONYMOUS, -1, 0) = 0x7f0810df5000
     0.000016 mmap(NULL, 1323280, PROT_READ, MAP_PRIVATE|MAP_DENYWRITE, 3, 0) = 0x7f0810c93000
     0.000017 mmap(0x7f0810ca0000, 630784, PROT_READ|PROT_EXEC, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0xd000) = 0x7f0810ca0000
     0.000019 mmap(0x7f0810d3a000, 634880, PROT_READ, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0xa7000) = 0x7f0810d3a000
     0.000017 mmap(0x7f0810dd5000, 8192, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x141000) = 0x7f0810dd5000
     0.000016 mmap(NULL, 20752, PROT_READ, MAP_PRIVATE|MAP_DENYWRITE, 3, 0) = 0x7f0810c8d000
     0.000022 mmap(0x7f0810c8e000, 8192, PROT_READ|PROT_EXEC, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x1000) = 0x7f0810c8e000
     0.000019 mmap(0x7f0810c90000, 4096, PROT_READ, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x3000) = 0x7f0810c90000
     0.000016 mmap(0x7f0810c91000, 8192, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x3000) = 0x7f0810c91000
     0.000016 mmap(NULL, 8192, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0x7f0810c8b000
     0.000019 mmap(NULL, 1918592, PROT_READ, MAP_PRIVATE|MAP_DENYWRITE, 3, 0) = 0x7f0810ab6000
     0.000017 mmap(0x7f0810ad8000, 1417216, PROT_READ|PROT_EXEC, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x22000) = 0x7f0810ad8000
     0.000019 mmap(0x7f0810c32000, 323584, PROT_READ, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x17c000) = 0x7f0810c32000
     0.000018 mmap(0x7f0810c81000, 24576, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x1ca000) = 0x7f0810c81000
     0.000019 mmap(0x7f0810c87000, 13952, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_FIXED|MAP_ANONYMOUS, -1, 0) = 0x7f0810c87000
     0.000034 mmap(NULL, 12288, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0x7f0810ab3000
     0.000019 mmap(NULL, 12288, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS|MAP_STACK, -1, 0) = 0x7f08111a3000
     0.000045 brk(NULL)                 = 0x556cefa79000
     0.000017 brk(0x556cefa9a000)       = 0x556cefa9a000
     0.000020 mmap(NULL, 2101248, PROT_NONE, MAP_PRIVATE|MAP_ANONYMOUS|MAP_STACK, -1, 0) = 0x7f08108b2000
     0.000080 mmap(NULL, 2101248, PROT_NONE, MAP_PRIVATE|MAP_ANONYMOUS|MAP_STACK, -1, 0) = 0x7f08106b1000
     0.000019 mmap(NULL, 2101248, PROT_NONE, MAP_PRIVATE|MAP_ANONYMOUS|MAP_STACK, -1, 0) = 0x7f08104b0000
     0.000026 mmap(NULL, 2101248, PROT_NONE, MAP_PRIVATE|MAP_ANONYMOUS|MAP_STACK, -1, 0) = 0x7f08102ac000
     0.000385 brk(0x556cefabb000)       = 0x556cefabb000
     0.000192 brk(0x556cefadc000)       = 0x556cefadc000
     0.000075 brk(0x556cefafd000)       = 0x556cefafd000
     0.000026 brk(0x556cefb1e000)       = 0x556cefb1e000
     0.000030 brk(0x556cefb1c000)       = 0x556cefb1c000
     0.000120 brk(0x556cefb3d000)       = 0x556cefb3d000
     0.000023 brk(0x556cefb5e000)       = 0x556cefb5e000
     0.000031 brk(0x556cefb5d000)       = 0x556cefb5d000
     0.000159 brk(0x556cefb7e000)       = 0x556cefb7e000
     0.000239 brk(0x556cefb9f000)       = 0x556cefb9f000
     0.000137 brk(0x556cefbc0000)       = 0x556cefbc0000
     0.000132 brk(0x556cefbe1000)       = 0x556cefbe1000
     0.000117 brk(0x556cefc02000)       = 0x556cefc02000
     0.000137 brk(0x556cefc23000)       = 0x556cefc23000
     0.000128 brk(0x556cefc44000)       = 0x556cefc44000
     0.000106 brk(0x556cefc65000)       = 0x556cefc65000
     0.000131 brk(0x556cefc86000)       = 0x556cefc86000
     0.000134 brk(0x556cefca7000)       = 0x556cefca7000
     0.000162 brk(0x556cefcc8000)       = 0x556cefcc8000
     0.000142 brk(0x556cefce9000)       = 0x556cefce9000
     0.000153 mmap(NULL, 2101248, PROT_NONE, MAP_PRIVATE|MAP_ANONYMOUS|MAP_STACK, -1, 0) = 0x7f08100a8000
     0.000458 brk(0x556cefd0b000)       = 0x556cefd0b000
```     

### 1st pass SBOM Optimization

```     
cargo tree
write_flux v0.0.5 (./write_flux)
├── chrono v0.4.23
│   ├── iana-time-zone v0.1.53
│   ├── num-integer v0.1.45
│   │   └── num-traits v0.2.15
│   │       [build-dependencies]
│   │       └── autocfg v1.1.0
│   │   [build-dependencies]
│   │   └── autocfg v1.1.0
│   ├── num-traits v0.2.15 (*)
│   ├── serde v1.0.152
│   │   └── serde_derive v1.0.152 (proc-macro)
│   │       ├── proc-macro2 v1.0.51
│   │       │   └── unicode-ident v1.0.6
│   │       ├── quote v1.0.23
│   │       │   └── proc-macro2 v1.0.51 (*)
│   │       └── syn v1.0.107
│   │           ├── proc-macro2 v1.0.51 (*)
│   │           ├── quote v1.0.23 (*)
│   │           └── unicode-ident v1.0.6
│   └── time v0.1.45
│       └── libc v0.2.139
├── clap v4.1.6
│   ├── bitflags v1.3.2
│   ├── clap_derive v4.1.0 (proc-macro)
│   │   ├── heck v0.4.1
│   │   ├── proc-macro-error v1.0.4
│   │   │   ├── proc-macro-error-attr v1.0.4 (proc-macro)
│   │   │   │   ├── proc-macro2 v1.0.51 (*)
│   │   │   │   └── quote v1.0.23 (*)
│   │   │   │   [build-dependencies]
│   │   │   │   └── version_check v0.9.4
│   │   │   ├── proc-macro2 v1.0.51 (*)
│   │   │   ├── quote v1.0.23 (*)
│   │   │   └── syn v1.0.107 (*)
│   │   │   [build-dependencies]
│   │   │   └── version_check v0.9.4
│   │   ├── proc-macro2 v1.0.51 (*)
│   │   ├── quote v1.0.23 (*)
│   │   └── syn v1.0.107 (*)
│   ├── clap_lex v0.3.1
│   │   └── os_str_bytes v6.4.1
│   ├── is-terminal v0.4.3
│   │   ├── io-lifetimes v1.0.5
│   │   │   └── libc v0.2.139
│   │   └── rustix v0.36.8
│   │       ├── bitflags v1.3.2
│   │       ├── io-lifetimes v1.0.5 (*)
│   │       ├── libc v0.2.139
│   │       └── linux-raw-sys v0.1.4
│   ├── once_cell v1.17.1
│   ├── strsim v0.10.0
│   └── termcolor v1.2.0
├── clap-verbosity-flag v2.0.0
│   ├── clap v4.1.6 (*)
│   └── log v0.4.17
│       └── cfg-if v1.0.0
├── env_logger v0.10.0
│   ├── humantime v2.1.0
│   ├── is-terminal v0.4.3 (*)
│   ├── log v0.4.17 (*)
│   ├── regex v1.7.1
│   │   ├── aho-corasick v0.7.20
│   │   │   └── memchr v2.5.0
│   │   ├── memchr v2.5.0
│   │   └── regex-syntax v0.6.28
│   └── termcolor v1.2.0
├── futures v0.3.26
│   ├── futures-channel v0.3.26
│   │   ├── futures-core v0.3.26
│   │   └── futures-sink v0.3.26
│   ├── futures-core v0.3.26
│   ├── futures-executor v0.3.26
│   │   ├── futures-core v0.3.26
│   │   ├── futures-task v0.3.26
│   │   └── futures-util v0.3.26
│   │       ├── futures-channel v0.3.26 (*)
│   │       ├── futures-core v0.3.26
│   │       ├── futures-io v0.3.26
│   │       ├── futures-macro v0.3.26 (proc-macro)
│   │       │   ├── proc-macro2 v1.0.51 (*)
│   │       │   ├── quote v1.0.23 (*)
│   │       │   └── syn v1.0.107 (*)
│   │       ├── futures-sink v0.3.26
│   │       ├── futures-task v0.3.26
│   │       ├── memchr v2.5.0
│   │       ├── pin-project-lite v0.2.9
│   │       ├── pin-utils v0.1.0
│   │       └── slab v0.4.7
│   │           [build-dependencies]
│   │           └── autocfg v1.1.0
│   ├── futures-io v0.3.26
│   ├── futures-sink v0.3.26
│   ├── futures-task v0.3.26
│   └── futures-util v0.3.26 (*)
├── influxdb2 v0.3.5
│   ├── base64 v0.13.1
│   ├── bytes v1.4.0
│   ├── chrono v0.4.23 (*)
│   ├── csv v1.2.0
│   │   ├── csv-core v0.1.10
│   │   │   └── memchr v2.5.0
│   │   ├── itoa v1.0.5
│   │   ├── ryu v1.0.12
│   │   └── serde v1.0.152 (*)
│   ├── dotenv v0.15.0
│   ├── fallible-iterator v0.2.0
│   ├── futures v0.3.26 (*)
│   ├── go-parse-duration v0.1.1
│   ├── influxdb2-derive v0.1.0 (proc-macro)
│   │   ├── itertools v0.10.5
│   │   │   └── either v1.8.1
│   │   ├── proc-macro2 v1.0.51 (*)
│   │   ├── quote v1.0.23 (*)
│   │   ├── regex v1.7.1
│   │   │   ├── aho-corasick v0.7.20 (*)
│   │   │   ├── memchr v2.5.0
│   │   │   └── regex-syntax v0.6.28
│   │   └── syn v1.0.107 (*)
│   ├── influxdb2-structmap v0.2.0
│   │   ├── chrono v0.4.23 (*)
│   │   ├── num-traits v0.2.15 (*)
│   │   └── ordered-float v3.4.0
│   │       └── num-traits v0.2.15 (*)
│   ├── nom v6.1.2
│   │   ├── bitvec v0.19.6
│   │   │   ├── funty v1.1.0
│   │   │   ├── radium v0.5.3
│   │   │   ├── tap v1.0.1
│   │   │   └── wyz v0.2.0
│   │   ├── funty v1.1.0
│   │   ├── lexical-core v0.7.6
│   │   │   ├── arrayvec v0.5.2
│   │   │   ├── bitflags v1.3.2
│   │   │   ├── cfg-if v1.0.0
│   │   │   ├── ryu v1.0.12
│   │   │   └── static_assertions v1.1.0
│   │   └── memchr v2.5.0
│   │   [build-dependencies]
│   │   └── version_check v0.9.4
│   ├── opentelemetry v0.13.0
│   │   ├── async-trait v0.1.64 (proc-macro)
│   │   │   ├── proc-macro2 v1.0.51 (*)
│   │   │   ├── quote v1.0.23 (*)
│   │   │   └── syn v1.0.107 (*)
│   │   ├── dashmap v4.0.2
│   │   │   ├── cfg-if v1.0.0
│   │   │   └── num_cpus v1.15.0
│   │   │       └── libc v0.2.139
│   │   ├── fnv v1.0.7
│   │   ├── futures v0.3.26 (*)
│   │   ├── lazy_static v1.4.0
│   │   ├── percent-encoding v2.2.0
│   │   ├── pin-project v1.0.12
│   │   │   └── pin-project-internal v1.0.12 (proc-macro)
│   │   │       ├── proc-macro2 v1.0.51 (*)
│   │   │       ├── quote v1.0.23 (*)
│   │   │       └── syn v1.0.107 (*)
│   │   ├── rand v0.8.5
│   │   │   ├── libc v0.2.139
│   │   │   ├── rand_chacha v0.3.1
│   │   │   │   ├── ppv-lite86 v0.2.17
│   │   │   │   └── rand_core v0.6.4
│   │   │   │       └── getrandom v0.2.8
│   │   │   │           ├── cfg-if v1.0.0
│   │   │   │           └── libc v0.2.139
│   │   │   └── rand_core v0.6.4 (*)
│   │   ├── thiserror v1.0.38
│   │   │   └── thiserror-impl v1.0.38 (proc-macro)
│   │   │       ├── proc-macro2 v1.0.51 (*)
│   │   │       ├── quote v1.0.23 (*)
│   │   │       └── syn v1.0.107 (*)
│   │   ├── tokio v1.25.0
│   │   │   ├── bytes v1.4.0
│   │   │   ├── libc v0.2.139
│   │   │   ├── memchr v2.5.0
│   │   │   ├── mio v0.8.6
│   │   │   │   ├── libc v0.2.139
│   │   │   │   └── log v0.4.17 (*)
│   │   │   ├── num_cpus v1.15.0 (*)
│   │   │   ├── parking_lot v0.12.1
│   │   │   │   ├── lock_api v0.4.9
│   │   │   │   │   └── scopeguard v1.1.0
│   │   │   │   │   [build-dependencies]
│   │   │   │   │   └── autocfg v1.1.0
│   │   │   │   └── parking_lot_core v0.9.7
│   │   │   │       ├── cfg-if v1.0.0
│   │   │   │       ├── libc v0.2.139
│   │   │   │       └── smallvec v1.10.0
│   │   │   ├── pin-project-lite v0.2.9
│   │   │   ├── signal-hook-registry v1.4.1
│   │   │   │   └── libc v0.2.139
│   │   │   ├── socket2 v0.4.7
│   │   │   │   └── libc v0.2.139
│   │   │   └── tokio-macros v1.8.2 (proc-macro)
│   │   │       ├── proc-macro2 v1.0.51 (*)
│   │   │       ├── quote v1.0.23 (*)
│   │   │       └── syn v1.0.107 (*)
│   │   │   [build-dependencies]
│   │   │   └── autocfg v1.1.0
│   │   └── tokio-stream v0.1.11
│   │       ├── futures-core v0.3.26
│   │       ├── pin-project-lite v0.2.9
│   │       └── tokio v1.25.0 (*)
│   ├── ordered-float v3.4.0 (*)
│   ├── parking_lot v0.11.2
│   │   ├── instant v0.1.12
│   │   │   └── cfg-if v1.0.0
│   │   ├── lock_api v0.4.9 (*)
│   │   └── parking_lot_core v0.8.6
│   │       ├── cfg-if v1.0.0
│   │       ├── instant v0.1.12 (*)
│   │       ├── libc v0.2.139
│   │       └── smallvec v1.10.0
│   ├── reqwest v0.11.14
│   │   ├── base64 v0.21.0
│   │   ├── bytes v1.4.0
│   │   ├── encoding_rs v0.8.32
│   │   │   └── cfg-if v1.0.0
│   │   ├── futures-core v0.3.26
│   │   ├── futures-util v0.3.26 (*)
│   │   ├── h2 v0.3.15
│   │   │   ├── bytes v1.4.0
│   │   │   ├── fnv v1.0.7
│   │   │   ├── futures-core v0.3.26
│   │   │   ├── futures-sink v0.3.26
│   │   │   ├── futures-util v0.3.26 (*)
│   │   │   ├── http v0.2.8
│   │   │   │   ├── bytes v1.4.0
│   │   │   │   ├── fnv v1.0.7
│   │   │   │   └── itoa v1.0.5
│   │   │   ├── indexmap v1.9.2
│   │   │   │   └── hashbrown v0.12.3
│   │   │   │   [build-dependencies]
│   │   │   │   └── autocfg v1.1.0
│   │   │   ├── slab v0.4.7 (*)
│   │   │   ├── tokio v1.25.0 (*)
│   │   │   ├── tokio-util v0.7.7
│   │   │   │   ├── bytes v1.4.0
│   │   │   │   ├── futures-core v0.3.26
│   │   │   │   ├── futures-sink v0.3.26
│   │   │   │   ├── pin-project-lite v0.2.9
│   │   │   │   ├── tokio v1.25.0 (*)
│   │   │   │   └── tracing v0.1.37
│   │   │   │       ├── cfg-if v1.0.0
│   │   │   │       ├── pin-project-lite v0.2.9
│   │   │   │       ├── tracing-attributes v0.1.23 (proc-macro)
│   │   │   │       │   ├── proc-macro2 v1.0.51 (*)
│   │   │   │       │   ├── quote v1.0.23 (*)
│   │   │   │       │   └── syn v1.0.107 (*)
│   │   │   │       └── tracing-core v0.1.30
│   │   │   │           └── once_cell v1.17.1
│   │   │   └── tracing v0.1.37 (*)
│   │   ├── http v0.2.8 (*)
│   │   ├── http-body v0.4.5
│   │   │   ├── bytes v1.4.0
│   │   │   ├── http v0.2.8 (*)
│   │   │   └── pin-project-lite v0.2.9
│   │   ├── hyper v0.14.24
│   │   │   ├── bytes v1.4.0
│   │   │   ├── futures-channel v0.3.26 (*)
│   │   │   ├── futures-core v0.3.26
│   │   │   ├── futures-util v0.3.26 (*)
│   │   │   ├── h2 v0.3.15 (*)
│   │   │   ├── http v0.2.8 (*)
│   │   │   ├── http-body v0.4.5 (*)
│   │   │   ├── httparse v1.8.0
│   │   │   ├── httpdate v1.0.2
│   │   │   ├── itoa v1.0.5
│   │   │   ├── pin-project-lite v0.2.9
│   │   │   ├── socket2 v0.4.7 (*)
│   │   │   ├── tokio v1.25.0 (*)
│   │   │   ├── tower-service v0.3.2
│   │   │   ├── tracing v0.1.37 (*)
│   │   │   └── want v0.3.0
│   │   │       ├── log v0.4.17 (*)
│   │   │       └── try-lock v0.2.4
│   │   ├── hyper-rustls v0.23.2
│   │   │   ├── http v0.2.8 (*)
│   │   │   ├── hyper v0.14.24 (*)
│   │   │   ├── rustls v0.20.8
│   │   │   │   ├── log v0.4.17 (*)
│   │   │   │   ├── ring v0.16.20
│   │   │   │   │   ├── libc v0.2.139
│   │   │   │   │   ├── once_cell v1.17.1
│   │   │   │   │   ├── spin v0.5.2
│   │   │   │   │   └── untrusted v0.7.1
│   │   │   │   │   [build-dependencies]
│   │   │   │   │   └── cc v1.0.79
│   │   │   │   ├── sct v0.7.0
│   │   │   │   │   ├── ring v0.16.20 (*)
│   │   │   │   │   └── untrusted v0.7.1
│   │   │   │   └── webpki v0.22.0
│   │   │   │       ├── ring v0.16.20 (*)
│   │   │   │       └── untrusted v0.7.1
│   │   │   ├── tokio v1.25.0 (*)
│   │   │   └── tokio-rustls v0.23.4
│   │   │       ├── rustls v0.20.8 (*)
│   │   │       ├── tokio v1.25.0 (*)
│   │   │       └── webpki v0.22.0 (*)
│   │   ├── hyper-tls v0.5.0
│   │   │   ├── bytes v1.4.0
│   │   │   ├── hyper v0.14.24 (*)
│   │   │   ├── native-tls v0.2.11
│   │   │   │   ├── log v0.4.17 (*)
│   │   │   │   ├── openssl v0.10.45
│   │   │   │   │   ├── bitflags v1.3.2
│   │   │   │   │   ├── cfg-if v1.0.0
│   │   │   │   │   ├── foreign-types v0.3.2
│   │   │   │   │   │   └── foreign-types-shared v0.1.1
│   │   │   │   │   ├── libc v0.2.139
│   │   │   │   │   ├── once_cell v1.17.1
│   │   │   │   │   ├── openssl-macros v0.1.0 (proc-macro)
│   │   │   │   │   │   ├── proc-macro2 v1.0.51 (*)
│   │   │   │   │   │   ├── quote v1.0.23 (*)
│   │   │   │   │   │   └── syn v1.0.107 (*)
│   │   │   │   │   └── openssl-sys v0.9.80
│   │   │   │   │       └── libc v0.2.139
│   │   │   │   │       [build-dependencies]
│   │   │   │   │       ├── autocfg v1.1.0
│   │   │   │   │       ├── cc v1.0.79
│   │   │   │   │       └── pkg-config v0.3.26
│   │   │   │   ├── openssl-probe v0.1.5
│   │   │   │   └── openssl-sys v0.9.80 (*)
│   │   │   ├── tokio v1.25.0 (*)
│   │   │   └── tokio-native-tls v0.3.1
│   │   │       ├── native-tls v0.2.11 (*)
│   │   │       └── tokio v1.25.0 (*)
│   │   ├── ipnet v2.7.1
│   │   ├── log v0.4.17 (*)
│   │   ├── mime v0.3.16
│   │   ├── native-tls v0.2.11 (*)
│   │   ├── once_cell v1.17.1
│   │   ├── percent-encoding v2.2.0
│   │   ├── pin-project-lite v0.2.9
│   │   ├── rustls v0.20.8 (*)
│   │   ├── rustls-pemfile v1.0.2
│   │   │   └── base64 v0.21.0
│   │   ├── serde v1.0.152 (*)
│   │   ├── serde_json v1.0.93
│   │   │   ├── itoa v1.0.5
│   │   │   ├── ryu v1.0.12
│   │   │   └── serde v1.0.152 (*)
│   │   ├── serde_urlencoded v0.7.1
│   │   │   ├── form_urlencoded v1.1.0
│   │   │   │   └── percent-encoding v2.2.0
│   │   │   ├── itoa v1.0.5
│   │   │   ├── ryu v1.0.12
│   │   │   └── serde v1.0.152 (*)
│   │   ├── tokio v1.25.0 (*)
│   │   ├── tokio-native-tls v0.3.1 (*)
│   │   ├── tokio-rustls v0.23.4 (*)
│   │   ├── tokio-util v0.7.7 (*)
│   │   ├── tower-service v0.3.2
│   │   ├── url v2.3.1
│   │   │   ├── form_urlencoded v1.1.0 (*)
│   │   │   ├── idna v0.3.0
│   │   │   │   ├── unicode-bidi v0.3.10
│   │   │   │   └── unicode-normalization v0.1.22
│   │   │   │       └── tinyvec v1.6.0
│   │   │   │           └── tinyvec_macros v0.1.1
│   │   │   └── percent-encoding v2.2.0
│   │   └── webpki-roots v0.22.6
│   │       └── webpki v0.22.0 (*)
│   ├── serde v1.0.152 (*)
│   ├── serde_json v1.0.93 (*)
│   ├── serde_qs v0.10.1
│   │   ├── percent-encoding v2.2.0
│   │   ├── serde v1.0.152 (*)
│   │   └── thiserror v1.0.38 (*)
│   ├── smallvec v1.10.0
│   ├── snafu v0.6.10
│   │   ├── doc-comment v0.3.3
│   │   └── snafu-derive v0.6.10 (proc-macro)
│   │       ├── proc-macro2 v1.0.51 (*)
│   │       ├── quote v1.0.23 (*)
│   │       └── syn v1.0.107 (*)
│   ├── tempfile v3.3.0
│   │   ├── cfg-if v1.0.0
│   │   ├── fastrand v1.9.0
│   │   ├── libc v0.2.139
│   │   └── remove_dir_all v0.5.3
│   ├── tracing v0.1.37 (*)
│   ├── tracing-subscriber v0.2.25
│   │   ├── ansi_term v0.12.1
│   │   ├── chrono v0.4.23 (*)
│   │   ├── lazy_static v1.4.0
│   │   ├── matchers v0.0.1
│   │   │   └── regex-automata v0.1.10
│   │   │       └── regex-syntax v0.6.28
│   │   ├── parking_lot v0.11.2 (*)
│   │   ├── regex v1.7.1 (*)
│   │   ├── serde v1.0.152 (*)
│   │   ├── serde_json v1.0.93 (*)
│   │   ├── sharded-slab v0.1.4
│   │   │   └── lazy_static v1.4.0
│   │   ├── smallvec v1.10.0
│   │   ├── thread_local v1.1.7
│   │   │   ├── cfg-if v1.0.0
│   │   │   └── once_cell v1.17.1
│   │   ├── tracing v0.1.37 (*)
│   │   ├── tracing-core v0.1.30 (*)
│   │   └── tracing-serde v0.1.3
│   │       ├── serde v1.0.152 (*)
│   │       └── tracing-core v0.1.30 (*)
│   └── url v2.3.1 (*)
├── log v0.4.17 (*)
├── nom v7.1.3
│   ├── memchr v2.5.0
│   └── minimal-lexical v0.2.1
├── opentelemetry v0.18.0
│   ├── opentelemetry_api v0.18.0
│   │   ├── futures-channel v0.3.26 (*)
│   │   ├── futures-util v0.3.26 (*)
│   │   ├── indexmap v1.9.2 (*)
│   │   ├── once_cell v1.17.1
│   │   ├── pin-project-lite v0.2.9
│   │   └── thiserror v1.0.38 (*)
│   └── opentelemetry_sdk v0.18.0
│       ├── async-trait v0.1.64 (proc-macro) (*)
│       ├── crossbeam-channel v0.5.6
│       │   ├── cfg-if v1.0.0
│       │   └── crossbeam-utils v0.8.14
│       │       └── cfg-if v1.0.0
│       ├── futures-channel v0.3.26 (*)
│       ├── futures-executor v0.3.26 (*)
│       ├── futures-util v0.3.26 (*)
│       ├── once_cell v1.17.1
│       ├── opentelemetry_api v0.18.0 (*)
│       ├── percent-encoding v2.2.0
│       ├── rand v0.8.5 (*)
│       └── thiserror v1.0.38 (*)
├── prelude v0.2.1
├── reqwest v0.11.14 (*)
├── serde v1.0.152 (*)
├── serde_derive v1.0.152 (proc-macro) (*)
├── serde_json v1.0.93 (*)
└── tokio v1.25.0 (*)
```     


### 1st pass SBOM feature Optimization
```
cargo tree --format "{p} {f}"
write_flux v0.0.5 (./write_flux) default,native-tls
├── chrono v0.4.23 clock,default,iana-time-zone,js-sys,oldtime,serde,std,time,wasm-bindgen,wasmbind,winapi
│   ├── iana-time-zone v0.1.53 fallback
│   ├── num-integer v0.1.45
│   │   └── num-traits v0.2.15 default,std
│   │       [build-dependencies]
│   │       └── autocfg v1.1.0
│   │   [build-dependencies]
│   │   └── autocfg v1.1.0
│   ├── num-traits v0.2.15 default,std (*)
│   ├── serde v1.0.152 default,derive,serde_derive,std
│   │   └── serde_derive v1.0.152 (proc-macro) default
│   │       ├── proc-macro2 v1.0.51 default,proc-macro
│   │       │   └── unicode-ident v1.0.6
│   │       ├── quote v1.0.23 default,proc-macro
│   │       │   └── proc-macro2 v1.0.51 default,proc-macro (*)
│   │       └── syn v1.0.107 clone-impls,default,derive,extra-traits,full,parsing,printing,proc-macro,quote,visit,visit-mut
│   │           ├── proc-macro2 v1.0.51 default,proc-macro (*)
│   │           ├── quote v1.0.23 default,proc-macro (*)
│   │           └── unicode-ident v1.0.6
│   └── time v0.1.45
│       └── libc v0.2.139 default,extra_traits,std
├── clap v4.1.6 cargo,color,default,derive,error-context,help,std,suggestions,usage
│   ├── bitflags v1.3.2 default
│   ├── clap_derive v4.1.0 (proc-macro) default
│   │   ├── heck v0.4.1 default
│   │   ├── proc-macro-error v1.0.4 default,syn,syn-error
│   │   │   ├── proc-macro-error-attr v1.0.4 (proc-macro)
│   │   │   │   ├── proc-macro2 v1.0.51 default,proc-macro (*)
│   │   │   │   └── quote v1.0.23 default,proc-macro (*)
│   │   │   │   [build-dependencies]
│   │   │   │   └── version_check v0.9.4
│   │   │   ├── proc-macro2 v1.0.51 default,proc-macro (*)
│   │   │   ├── quote v1.0.23 default,proc-macro (*)
│   │   │   └── syn v1.0.107 clone-impls,default,derive,extra-traits,full,parsing,printing,proc-macro,quote,visit,visit-mut (*)
│   │   │   [build-dependencies]
│   │   │   └── version_check v0.9.4
│   │   ├── proc-macro2 v1.0.51 default,proc-macro (*)
│   │   ├── quote v1.0.23 default,proc-macro (*)
│   │   └── syn v1.0.107 clone-impls,default,derive,extra-traits,full,parsing,printing,proc-macro,quote,visit,visit-mut (*)
│   ├── clap_lex v0.3.1
│   │   └── os_str_bytes v6.4.1 raw_os_str
│   ├── is-terminal v0.4.3
│   │   ├── io-lifetimes v1.0.5 close,default,libc,windows-sys
│   │   │   └── libc v0.2.139 default,extra_traits,std
│   │   └── rustix v0.36.8 default,io-lifetimes,libc,std,termios,use-libc-auxv
│   │       ├── bitflags v1.3.2 default
│   │       ├── io-lifetimes v1.0.5 close,default,libc,windows-sys (*)
│   │       ├── libc v0.2.139 default,extra_traits,std
│   │       └── linux-raw-sys v0.1.4 errno,general,ioctl,no_std
│   ├── once_cell v1.17.1 alloc,default,race,std
│   ├── strsim v0.10.0
│   └── termcolor v1.2.0
├── clap-verbosity-flag v2.0.0
│   ├── clap v4.1.6 cargo,color,default,derive,error-context,help,std,suggestions,usage (*)
│   └── log v0.4.17 std
│       └── cfg-if v1.0.0
├── env_logger v0.10.0 auto-color,color,default,humantime,regex
│   ├── humantime v2.1.0
│   ├── is-terminal v0.4.3  (*)
│   ├── log v0.4.17 std (*)
│   ├── regex v1.7.1 aho-corasick,memchr,perf,perf-cache,perf-dfa,perf-inline,perf-literal,std
│   │   ├── aho-corasick v0.7.20 default,std
│   │   │   └── memchr v2.5.0 default,std,use_std
│   │   ├── memchr v2.5.0 default,std,use_std
│   │   └── regex-syntax v0.6.28 default,unicode,unicode-age,unicode-bool,unicode-case,unicode-gencat,unicode-perl,unicode-script,unicode-segment
│   └── termcolor v1.2.0
├── futures v0.3.26 alloc,async-await,default,executor,futures-executor,std
│   ├── futures-channel v0.3.26 alloc,default,futures-sink,sink,std
│   │   ├── futures-core v0.3.26 alloc,default,std
│   │   └── futures-sink v0.3.26 alloc,default,std
│   ├── futures-core v0.3.26 alloc,default,std
│   ├── futures-executor v0.3.26 default,std
│   │   ├── futures-core v0.3.26 alloc,default,std
│   │   ├── futures-task v0.3.26 alloc,std
│   │   └── futures-util v0.3.26 alloc,async-await,async-await-macro,channel,futures-channel,futures-io,futures-macro,futures-sink,io,memchr,sink,slab,std
│   │       ├── futures-channel v0.3.26 alloc,default,futures-sink,sink,std (*)
│   │       ├── futures-core v0.3.26 alloc,default,std
│   │       ├── futures-io v0.3.26 std
│   │       ├── futures-macro v0.3.26 (proc-macro)
│   │       │   ├── proc-macro2 v1.0.51 default,proc-macro (*)
│   │       │   ├── quote v1.0.23 default,proc-macro (*)
│   │       │   └── syn v1.0.107 clone-impls,default,derive,extra-traits,full,parsing,printing,proc-macro,quote,visit,visit-mut (*)
│   │       ├── futures-sink v0.3.26 alloc,default,std
│   │       ├── futures-task v0.3.26 alloc,std
│   │       ├── memchr v2.5.0 default,std,use_std
│   │       ├── pin-project-lite v0.2.9
│   │       ├── pin-utils v0.1.0
│   │       └── slab v0.4.7 default,std
│   │           [build-dependencies]
│   │           └── autocfg v1.1.0
│   ├── futures-io v0.3.26 std
│   ├── futures-sink v0.3.26 alloc,default,std
│   ├── futures-task v0.3.26 alloc,std
│   └── futures-util v0.3.26 alloc,async-await,async-await-macro,channel,futures-channel,futures-io,futures-macro,futures-sink,io,memchr,sink,slab,std (*)
├── influxdb2 v0.3.5 rustls
│   ├── base64 v0.13.1 default,std
│   ├── bytes v1.4.0 default,std
│   ├── chrono v0.4.23 clock,default,iana-time-zone,js-sys,oldtime,serde,std,time,wasm-bindgen,wasmbind,winapi (*)
│   ├── csv v1.2.0
│   │   ├── csv-core v0.1.10 default
│   │   │   └── memchr v2.5.0 default,std,use_std
│   │   ├── itoa v1.0.5
│   │   ├── ryu v1.0.12
│   │   └── serde v1.0.152 default,derive,serde_derive,std (*)
│   ├── dotenv v0.15.0
│   ├── fallible-iterator v0.2.0 default,std
│   ├── futures v0.3.26 alloc,async-await,default,executor,futures-executor,std (*)
│   ├── go-parse-duration v0.1.1
│   ├── influxdb2-derive v0.1.0 (proc-macro)
│   │   ├── itertools v0.10.5 default,use_alloc,use_std
│   │   │   └── either v1.8.1 use_std
│   │   ├── proc-macro2 v1.0.51 default,proc-macro (*)
│   │   ├── quote v1.0.23 default,proc-macro (*)
│   │   ├── regex v1.7.1 aho-corasick,default,memchr,perf,perf-cache,perf-dfa,perf-inline,perf-literal,std,unicode,unicode-age,unicode-bool,unicode-case,unicode-gencat,unicode-perl,unicode-script,unicode-segment
│   │   │   ├── aho-corasick v0.7.20 default,std (*)
│   │   │   ├── memchr v2.5.0 default,std
│   │   │   └── regex-syntax v0.6.28 default,unicode,unicode-age,unicode-bool,unicode-case,unicode-gencat,unicode-perl,unicode-script,unicode-segment
│   │   └── syn v1.0.107 clone-impls,default,derive,extra-traits,full,parsing,printing,proc-macro,quote,visit,visit-mut (*)
│   ├── influxdb2-structmap v0.2.0
│   │   ├── chrono v0.4.23 clock,default,iana-time-zone,js-sys,oldtime,serde,std,time,wasm-bindgen,wasmbind,winapi (*)
│   │   ├── num-traits v0.2.15 default,std (*)
│   │   └── ordered-float v3.4.0 default,std
│   │       └── num-traits v0.2.15 default,std (*)
│   ├── nom v6.1.2 alloc,bitvec,default,funty,lexical,lexical-core,std
│   │   ├── bitvec v0.19.6 alloc,std
│   │   │   ├── funty v1.1.0
│   │   │   ├── radium v0.5.3
│   │   │   ├── tap v1.0.1
│   │   │   └── wyz v0.2.0 alloc
│   │   ├── funty v1.1.0
│   │   ├── lexical-core v0.7.6 arrayvec,correct,default,ryu,static_assertions,std,table
│   │   │   ├── arrayvec v0.5.2 array-sizes-33-128
│   │   │   ├── bitflags v1.3.2 default
│   │   │   ├── cfg-if v1.0.0
│   │   │   ├── ryu v1.0.12
│   │   │   └── static_assertions v1.1.0
│   │   └── memchr v2.5.0 default,std,use_std
│   │   [build-dependencies]
│   │   └── version_check v0.9.4
│   ├── opentelemetry v0.13.0 async-trait,dashmap,fnv,metrics,percent-encoding,pin-project,rand,rt-tokio,tokio,tokio-stream,trace
│   │   ├── async-trait v0.1.64 (proc-macro)
│   │   │   ├── proc-macro2 v1.0.51 default,proc-macro (*)
│   │   │   ├── quote v1.0.23 default,proc-macro (*)
│   │   │   └── syn v1.0.107 clone-impls,default,derive,extra-traits,full,parsing,printing,proc-macro,quote,visit,visit-mut (*)
│   │   ├── dashmap v4.0.2 default
│   │   │   ├── cfg-if v1.0.0
│   │   │   └── num_cpus v1.15.0
│   │   │       └── libc v0.2.139 default,extra_traits,std
│   │   ├── fnv v1.0.7 default,std
│   │   ├── futures v0.3.26 alloc,async-await,default,executor,futures-executor,std (*)
│   │   ├── lazy_static v1.4.0
│   │   ├── percent-encoding v2.2.0 alloc,default
│   │   ├── pin-project v1.0.12
│   │   │   └── pin-project-internal v1.0.12 (proc-macro)
│   │   │       ├── proc-macro2 v1.0.51 default,proc-macro (*)
│   │   │       ├── quote v1.0.23 default,proc-macro (*)
│   │   │       └── syn v1.0.107 clone-impls,default,derive,extra-traits,full,parsing,printing,proc-macro,quote,visit,visit-mut (*)
│   │   ├── rand v0.8.5 alloc,getrandom,libc,rand_chacha,std,std_rng
│   │   │   ├── libc v0.2.139 default,extra_traits,std
│   │   │   ├── rand_chacha v0.3.1 std
│   │   │   │   ├── ppv-lite86 v0.2.17 simd,std
│   │   │   │   └── rand_core v0.6.4 alloc,getrandom,std
│   │   │   │       └── getrandom v0.2.8 std
│   │   │   │           ├── cfg-if v1.0.0
│   │   │   │           └── libc v0.2.139 default,extra_traits,std
│   │   │   └── rand_core v0.6.4 alloc,getrandom,std (*)
│   │   ├── thiserror v1.0.38
│   │   │   └── thiserror-impl v1.0.38 (proc-macro)
│   │   │       ├── proc-macro2 v1.0.51 default,proc-macro (*)
│   │   │       ├── quote v1.0.23 default,proc-macro (*)
│   │   │       └── syn v1.0.107 clone-impls,default,derive,extra-traits,full,parsing,printing,proc-macro,quote,visit,visit-mut (*)
│   │   ├── tokio v1.25.0 bytes,default,fs,full,io-std,io-util,libc,macros,memchr,mio,net,num_cpus,parking_lot,process,rt,rt-multi-thread,signal,signal-hook-registry,socket2,sync,time,tokio-macros
│   │   │   ├── bytes v1.4.0 default,std
│   │   │   ├── libc v0.2.139 default,extra_traits,std
│   │   │   ├── memchr v2.5.0 default,std,use_std
│   │   │   ├── mio v0.8.6 default,net,os-ext,os-poll
│   │   │   │   ├── libc v0.2.139 default,extra_traits,std
│   │   │   │   └── log v0.4.17 std (*)
│   │   │   ├── num_cpus v1.15.0  (*)
│   │   │   ├── parking_lot v0.12.1 default
│   │   │   │   ├── lock_api v0.4.9
│   │   │   │   │   └── scopeguard v1.1.0
│   │   │   │   │   [build-dependencies]
│   │   │   │   │   └── autocfg v1.1.0
│   │   │   │   └── parking_lot_core v0.9.7
│   │   │   │       ├── cfg-if v1.0.0
│   │   │   │       ├── libc v0.2.139 default,extra_traits,std
│   │   │   │       └── smallvec v1.10.0
│   │   │   ├── pin-project-lite v0.2.9
│   │   │   ├── signal-hook-registry v1.4.1
│   │   │   │   └── libc v0.2.139 default,extra_traits,std
│   │   │   ├── socket2 v0.4.7 all
│   │   │   │   └── libc v0.2.139 default,extra_traits,std
│   │   │   └── tokio-macros v1.8.2 (proc-macro)
│   │   │       ├── proc-macro2 v1.0.51 default,proc-macro (*)
│   │   │       ├── quote v1.0.23 default,proc-macro (*)
│   │   │       └── syn v1.0.107 clone-impls,default,derive,extra-traits,full,parsing,printing,proc-macro,quote,visit,visit-mut (*)
│   │   │   [build-dependencies]
│   │   │   └── autocfg v1.1.0
│   │   └── tokio-stream v0.1.11 default,time
│   │       ├── futures-core v0.3.26 alloc,default,std
│   │       ├── pin-project-lite v0.2.9
│   │       └── tokio v1.25.0 bytes,default,fs,full,io-std,io-util,libc,macros,memchr,mio,net,num_cpus,parking_lot,process,rt,rt-multi-thread,signal,signal-hook-registry,socket2,sync,time,tokio-macros (*)
│   ├── ordered-float v3.4.0 default,std (*)
│   ├── parking_lot v0.11.2 default
│   │   ├── instant v0.1.12
│   │   │   └── cfg-if v1.0.0
│   │   ├── lock_api v0.4.9  (*)
│   │   └── parking_lot_core v0.8.6
│   │       ├── cfg-if v1.0.0
│   │       ├── instant v0.1.12  (*)
│   │       ├── libc v0.2.139 default,extra_traits,std
│   │       └── smallvec v1.10.0
│   ├── reqwest v0.11.14 __rustls,__tls,blocking,default,default-tls,hyper-rustls,hyper-tls,json,native-tls,native-tls-crate,rustls,rustls-pemfile,rustls-tls,rustls-tls-webpki-roots,serde_json,stream,tokio-native-tls,tokio-rustls,tokio-util,wasm-streams,webpki-roots
│   │   ├── base64 v0.21.0 default,std
│   │   ├── bytes v1.4.0 default,std
│   │   ├── encoding_rs v0.8.32 alloc,default
│   │   │   └── cfg-if v1.0.0
│   │   ├── futures-core v0.3.26 alloc,default,std
│   │   ├── futures-util v0.3.26 alloc,async-await,async-await-macro,channel,futures-channel,futures-io,futures-macro,futures-sink,io,memchr,sink,slab,std (*)
│   │   ├── h2 v0.3.15
│   │   │   ├── bytes v1.4.0 default,std
│   │   │   ├── fnv v1.0.7 default,std
│   │   │   ├── futures-core v0.3.26 alloc,default,std
│   │   │   ├── futures-sink v0.3.26 alloc,default,std
│   │   │   ├── futures-util v0.3.26 alloc,async-await,async-await-macro,channel,futures-channel,futures-io,futures-macro,futures-sink,io,memchr,sink,slab,std (*)
│   │   │   ├── http v0.2.8
│   │   │   │   ├── bytes v1.4.0 default,std
│   │   │   │   ├── fnv v1.0.7 default,std
│   │   │   │   └── itoa v1.0.5
│   │   │   ├── indexmap v1.9.2 std
│   │   │   │   └── hashbrown v0.12.3 raw
│   │   │   │   [build-dependencies]
│   │   │   │   └── autocfg v1.1.0
│   │   │   ├── slab v0.4.7 default,std (*)
│   │   │   ├── tokio v1.25.0 bytes,default,fs,full,io-std,io-util,libc,macros,memchr,mio,net,num_cpus,parking_lot,process,rt,rt-multi-thread,signal,signal-hook-registry,socket2,sync,time,tokio-macros (*)
│   │   │   ├── tokio-util v0.7.7 codec,default,io,tracing
│   │   │   │   ├── bytes v1.4.0 default,std
│   │   │   │   ├── futures-core v0.3.26 alloc,default,std
│   │   │   │   ├── futures-sink v0.3.26 alloc,default,std
│   │   │   │   ├── pin-project-lite v0.2.9
│   │   │   │   ├── tokio v1.25.0 bytes,default,fs,full,io-std,io-util,libc,macros,memchr,mio,net,num_cpus,parking_lot,process,rt,rt-multi-thread,signal,signal-hook-registry,socket2,sync,time,tokio-macros (*)
│   │   │   │   └── tracing v0.1.37 attributes,default,max_level_trace,release_max_level_debug,std,tracing-attributes
│   │   │   │       ├── cfg-if v1.0.0
│   │   │   │       ├── pin-project-lite v0.2.9
│   │   │   │       ├── tracing-attributes v0.1.23 (proc-macro)
│   │   │   │       │   ├── proc-macro2 v1.0.51 default,proc-macro (*)
│   │   │   │       │   ├── quote v1.0.23 default,proc-macro (*)
│   │   │   │       │   └── syn v1.0.107 clone-impls,default,derive,extra-traits,full,parsing,printing,proc-macro,quote,visit,visit-mut (*)
│   │   │   │       └── tracing-core v0.1.30 default,once_cell,std
│   │   │   │           └── once_cell v1.17.1 alloc,default,race,std
│   │   │   └── tracing v0.1.37 attributes,default,max_level_trace,release_max_level_debug,std,tracing-attributes (*)
│   │   ├── http v0.2.8  (*)
│   │   ├── http-body v0.4.5
│   │   │   ├── bytes v1.4.0 default,std
│   │   │   ├── http v0.2.8  (*)
│   │   │   └── pin-project-lite v0.2.9
│   │   ├── hyper v0.14.24 client,h2,http1,http2,runtime,socket2,tcp
│   │   │   ├── bytes v1.4.0 default,std
│   │   │   ├── futures-channel v0.3.26 alloc,default,futures-sink,sink,std (*)
│   │   │   ├── futures-core v0.3.26 alloc,default,std
│   │   │   ├── futures-util v0.3.26 alloc,async-await,async-await-macro,channel,futures-channel,futures-io,futures-macro,futures-sink,io,memchr,sink,slab,std (*)
│   │   │   ├── h2 v0.3.15  (*)
│   │   │   ├── http v0.2.8  (*)
│   │   │   ├── http-body v0.4.5  (*)
│   │   │   ├── httparse v1.8.0 default,std
│   │   │   ├── httpdate v1.0.2
│   │   │   ├── itoa v1.0.5
│   │   │   ├── pin-project-lite v0.2.9
│   │   │   ├── socket2 v0.4.7 all (*)
│   │   │   ├── tokio v1.25.0 bytes,default,fs,full,io-std,io-util,libc,macros,memchr,mio,net,num_cpus,parking_lot,process,rt,rt-multi-thread,signal,signal-hook-registry,socket2,sync,time,tokio-macros (*)
│   │   │   ├── tower-service v0.3.2
│   │   │   ├── tracing v0.1.37 attributes,default,max_level_trace,release_max_level_debug,std,tracing-attributes (*)
│   │   │   └── want v0.3.0
│   │   │       ├── log v0.4.17 std (*)
│   │   │       └── try-lock v0.2.4
│   │   ├── hyper-rustls v0.23.2
│   │   │   ├── http v0.2.8  (*)
│   │   │   ├── hyper v0.14.24 client,h2,http1,http2,runtime,socket2,tcp (*)
│   │   │   ├── rustls v0.20.8 dangerous_configuration,default,log,logging,tls12
│   │   │   │   ├── log v0.4.17 std (*)
│   │   │   │   ├── ring v0.16.20 alloc,default,dev_urandom_fallback,once_cell
│   │   │   │   │   ├── libc v0.2.139 default,extra_traits,std
│   │   │   │   │   ├── once_cell v1.17.1 alloc,default,race,std
│   │   │   │   │   ├── spin v0.5.2
│   │   │   │   │   └── untrusted v0.7.1
│   │   │   │   │   [build-dependencies]
│   │   │   │   │   └── cc v1.0.79
│   │   │   │   ├── sct v0.7.0
│   │   │   │   │   ├── ring v0.16.20 alloc,default,dev_urandom_fallback,once_cell (*)
│   │   │   │   │   └── untrusted v0.7.1
│   │   │   │   └── webpki v0.22.0 alloc,std
│   │   │   │       ├── ring v0.16.20 alloc,default,dev_urandom_fallback,once_cell (*)
│   │   │   │       └── untrusted v0.7.1
│   │   │   ├── tokio v1.25.0 bytes,default,fs,full,io-std,io-util,libc,macros,memchr,mio,net,num_cpus,parking_lot,process,rt,rt-multi-thread,signal,signal-hook-registry,socket2,sync,time,tokio-macros (*)
│   │   │   └── tokio-rustls v0.23.4 default,logging,tls12
│   │   │       ├── rustls v0.20.8 dangerous_configuration,default,log,logging,tls12 (*)
│   │   │       ├── tokio v1.25.0 bytes,default,fs,full,io-std,io-util,libc,macros,memchr,mio,net,num_cpus,parking_lot,process,rt,rt-multi-thread,signal,signal-hook-registry,socket2,sync,time,tokio-macros (*)
│   │   │       └── webpki v0.22.0 alloc,std (*)
│   │   ├── hyper-tls v0.5.0
│   │   │   ├── bytes v1.4.0 default,std
│   │   │   ├── hyper v0.14.24 client,h2,http1,http2,runtime,socket2,tcp (*)
│   │   │   ├── native-tls v0.2.11
│   │   │   │   ├── log v0.4.17 std (*)
│   │   │   │   ├── openssl v0.10.45 default
│   │   │   │   │   ├── bitflags v1.3.2 default
│   │   │   │   │   ├── cfg-if v1.0.0
│   │   │   │   │   ├── foreign-types v0.3.2
│   │   │   │   │   │   └── foreign-types-shared v0.1.1
│   │   │   │   │   ├── libc v0.2.139 default,extra_traits,std
│   │   │   │   │   ├── once_cell v1.17.1 alloc,default,race,std
│   │   │   │   │   ├── openssl-macros v0.1.0 (proc-macro)
│   │   │   │   │   │   ├── proc-macro2 v1.0.51 default,proc-macro (*)
│   │   │   │   │   │   ├── quote v1.0.23 default,proc-macro (*)
│   │   │   │   │   │   └── syn v1.0.107 clone-impls,default,derive,extra-traits,full,parsing,printing,proc-macro,quote,visit,visit-mut (*)
│   │   │   │   │   └── openssl-sys v0.9.80
│   │   │   │   │       └── libc v0.2.139 default,extra_traits,std
│   │   │   │   │       [build-dependencies]
│   │   │   │   │       ├── autocfg v1.1.0
│   │   │   │   │       ├── cc v1.0.79
│   │   │   │   │       └── pkg-config v0.3.26
│   │   │   │   ├── openssl-probe v0.1.5
│   │   │   │   └── openssl-sys v0.9.80  (*)
│   │   │   ├── tokio v1.25.0 bytes,default,fs,full,io-std,io-util,libc,macros,memchr,mio,net,num_cpus,parking_lot,process,rt,rt-multi-thread,signal,signal-hook-registry,socket2,sync,time,tokio-macros (*)
│   │   │   └── tokio-native-tls v0.3.1
│   │   │       ├── native-tls v0.2.11  (*)
│   │   │       └── tokio v1.25.0 bytes,default,fs,full,io-std,io-util,libc,macros,memchr,mio,net,num_cpus,parking_lot,process,rt,rt-multi-thread,signal,signal-hook-registry,socket2,sync,time,tokio-macros (*)
│   │   ├── ipnet v2.7.1 default
│   │   ├── log v0.4.17 std (*)
│   │   ├── mime v0.3.16
│   │   ├── native-tls v0.2.11  (*)
│   │   ├── once_cell v1.17.1 alloc,default,race,std
│   │   ├── percent-encoding v2.2.0 alloc,default
│   │   ├── pin-project-lite v0.2.9
│   │   ├── rustls v0.20.8 dangerous_configuration,default,log,logging,tls12 (*)
│   │   ├── rustls-pemfile v1.0.2
│   │   │   └── base64 v0.21.0 default,std
│   │   ├── serde v1.0.152 default,derive,serde_derive,std (*)
│   │   ├── serde_json v1.0.93 default,std
│   │   │   ├── itoa v1.0.5
│   │   │   ├── ryu v1.0.12
│   │   │   └── serde v1.0.152 default,derive,serde_derive,std (*)
│   │   ├── serde_urlencoded v0.7.1
│   │   │   ├── form_urlencoded v1.1.0
│   │   │   │   └── percent-encoding v2.2.0 alloc,default
│   │   │   ├── itoa v1.0.5
│   │   │   ├── ryu v1.0.12
│   │   │   └── serde v1.0.152 default,derive,serde_derive,std (*)
│   │   ├── tokio v1.25.0 bytes,default,fs,full,io-std,io-util,libc,macros,memchr,mio,net,num_cpus,parking_lot,process,rt,rt-multi-thread,signal,signal-hook-registry,socket2,sync,time,tokio-macros (*)
│   │   ├── tokio-native-tls v0.3.1  (*)
│   │   ├── tokio-rustls v0.23.4 default,logging,tls12 (*)
│   │   ├── tokio-util v0.7.7 codec,default,io,tracing (*)
│   │   ├── tower-service v0.3.2
│   │   ├── url v2.3.1 default
│   │   │   ├── form_urlencoded v1.1.0  (*)
│   │   │   ├── idna v0.3.0
│   │   │   │   ├── unicode-bidi v0.3.10 default,hardcoded-data,std
│   │   │   │   └── unicode-normalization v0.1.22 default,std
│   │   │   │       └── tinyvec v1.6.0 alloc,default,tinyvec_macros
│   │   │   │           └── tinyvec_macros v0.1.1
│   │   │   └── percent-encoding v2.2.0 alloc,default
│   │   └── webpki-roots v0.22.6
│   │       └── webpki v0.22.0 alloc,std (*)
│   ├── serde v1.0.152 default,derive,serde_derive,std (*)
│   ├── serde_json v1.0.93 default,std (*)
│   ├── serde_qs v0.10.1 default
│   │   ├── percent-encoding v2.2.0 alloc,default
│   │   ├── serde v1.0.152 default,derive,serde_derive,std (*)
│   │   └── thiserror v1.0.38  (*)
│   ├── smallvec v1.10.0
│   ├── snafu v0.6.10 default,guide,std
│   │   ├── doc-comment v0.3.3
│   │   └── snafu-derive v0.6.10 (proc-macro)
│   │       ├── proc-macro2 v1.0.51 default,proc-macro (*)
│   │       ├── quote v1.0.23 default,proc-macro (*)
│   │       └── syn v1.0.107 clone-impls,default,derive,extra-traits,full,parsing,printing,proc-macro,quote,visit,visit-mut (*)
│   ├── tempfile v3.3.0
│   │   ├── cfg-if v1.0.0
│   │   ├── fastrand v1.9.0
│   │   ├── libc v0.2.139 default,extra_traits,std
│   │   └── remove_dir_all v0.5.3
│   ├── tracing v0.1.37 attributes,default,max_level_trace,release_max_level_debug,std,tracing-attributes (*)
│   ├── tracing-subscriber v0.2.25 ansi,ansi_term,chrono,env-filter,fmt,json,lazy_static,matchers,parking_lot,regex,registry,serde,serde_json,sharded-slab,smallvec,thread_local,tracing,tracing-serde
│   │   ├── ansi_term v0.12.1
│   │   ├── chrono v0.4.23 clock,default,iana-time-zone,js-sys,oldtime,serde,std,time,wasm-bindgen,wasmbind,winapi (*)
│   │   ├── lazy_static v1.4.0
│   │   ├── matchers v0.0.1
│   │   │   └── regex-automata v0.1.10 default,regex-syntax,std
│   │   │       └── regex-syntax v0.6.28 default,unicode,unicode-age,unicode-bool,unicode-case,unicode-gencat,unicode-perl,unicode-script,unicode-segment
│   │   ├── parking_lot v0.11.2 default (*)
│   │   ├── regex v1.7.1 aho-corasick,memchr,perf,perf-cache,perf-dfa,perf-inline,perf-literal,std (*)
│   │   ├── serde v1.0.152 default,derive,serde_derive,std (*)
│   │   ├── serde_json v1.0.93 default,std (*)
│   │   ├── sharded-slab v0.1.4
│   │   │   └── lazy_static v1.4.0
│   │   ├── smallvec v1.10.0
│   │   ├── thread_local v1.1.7
│   │   │   ├── cfg-if v1.0.0
│   │   │   └── once_cell v1.17.1 alloc,default,race,std
│   │   ├── tracing v0.1.37 attributes,default,max_level_trace,release_max_level_debug,std,tracing-attributes (*)
│   │   ├── tracing-core v0.1.30 default,once_cell,std (*)
│   │   └── tracing-serde v0.1.3
│   │       ├── serde v1.0.152 default,derive,serde_derive,std (*)
│   │       └── tracing-core v0.1.30 default,once_cell,std (*)
│   └── url v2.3.1 default (*)
├── log v0.4.17 std (*)
├── nom v7.1.3 alloc,default,std
│   ├── memchr v2.5.0 default,std,use_std
│   └── minimal-lexical v0.2.1 std
├── opentelemetry v0.18.0 default,trace
│   ├── opentelemetry_api v0.18.0 default,pin-project-lite,trace
│   │   ├── futures-channel v0.3.26 alloc,default,futures-sink,sink,std (*)
│   │   ├── futures-util v0.3.26 alloc,async-await,async-await-macro,channel,futures-channel,futures-io,futures-macro,futures-sink,io,memchr,sink,slab,std (*)
│   │   ├── indexmap v1.9.2 std (*)
│   │   ├── once_cell v1.17.1 alloc,default,race,std
│   │   ├── pin-project-lite v0.2.9
│   │   └── thiserror v1.0.38  (*)
│   └── opentelemetry_sdk v0.18.0 async-trait,crossbeam-channel,default,percent-encoding,rand,trace
│       ├── async-trait v0.1.64 (proc-macro)  (*)
│       ├── crossbeam-channel v0.5.6 crossbeam-utils,default,std
│       │   ├── cfg-if v1.0.0
│       │   └── crossbeam-utils v0.8.14 std
│       │       └── cfg-if v1.0.0
│       ├── futures-channel v0.3.26 alloc,default,futures-sink,sink,std (*)
│       ├── futures-executor v0.3.26 default,std (*)
│       ├── futures-util v0.3.26 alloc,async-await,async-await-macro,channel,futures-channel,futures-io,futures-macro,futures-sink,io,memchr,sink,slab,std (*)
│       ├── once_cell v1.17.1 alloc,default,race,std
│       ├── opentelemetry_api v0.18.0 default,pin-project-lite,trace (*)
│       ├── percent-encoding v2.2.0 alloc,default
│       ├── rand v0.8.5 alloc,getrandom,libc,rand_chacha,std,std_rng (*)
│       └── thiserror v1.0.38  (*)
├── prelude v0.2.1
├── reqwest v0.11.14 __rustls,__tls,blocking,default,default-tls,hyper-rustls,hyper-tls,json,native-tls,native-tls-crate,rustls,rustls-pemfile,rustls-tls,rustls-tls-webpki-roots,serde_json,stream,tokio-native-tls,tokio-rustls,tokio-util,wasm-streams,webpki-roots (*)
├── serde v1.0.152 default,derive,serde_derive,std (*)
├── serde_derive v1.0.152 (proc-macro) default (*)
├── serde_json v1.0.93 default,std (*)
└── tokio v1.25.0 bytes,default,fs,full,io-std,io-util,libc,macros,memchr,mio,net,num_cpus,parking_lot,process,rt,rt-multi-thread,signal,signal-hook-registry,socket2,sync,time,tokio-macros (*)

```
