# BitTorrent-Rust

[BitTorrent-Rust](https://gitea.antoine-langlois.net/DataHearth/BitTorrent-Rust) is a fully fledged [BitTorrent](https://www.bittorrent.org) tool suite. A `library` supporting all [BEP](https://www.bittorrent.org/beps/bep_0000.html) (WIP), a `GUI` (TODO) for desktop usage, a `CLI`/`TUI` (WIP) and a `web ui` for containerised environment.

## Why another `GUI`/`CLI`/`TUI`/`WebUI`/`library` ?

Why not 😁. Joke aside, I feel like some users might want an `AIO` [BitTorrent](https://www.bittorrent.org) environment. My goal is to create a modern stack.

## Features

### Library

Everything under a [BEP](https://www.bittorrent.org/beps/bep_0000.html) `MUST` be supported. Non standard fields for `.torrent` are also mapped when encountered and they need a special treatement (like dates).  

To start developing with it:
```bash
cargo add bittorrent-rs
```

> [!WARNING]
> The library is a work in progress. Please refer to the implemented section below.
> The library's documentation is also a work in progress.

#### Implemented

- [ ] [BEP 0003](https://www.bittorrent.org/beps/bep_0003.html) - V1 specification
  - [x] `.torrent` parsing
  - [ ] Tracker request
  - [ ] Peer download
- [ ] [BEP 0004](https://www.bittorrent.org/beps/bep_0004.html) - Reserved bit allocation (tested & verified)
- [ ] [BEP 0005](https://www.bittorrent.org/beps/bep_0005.html) - DHT protocol
- [ ] [BEP 0020](https://www.bittorrent.org/beps/bep_0020.html) - Peer ID convention
