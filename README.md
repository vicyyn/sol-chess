<p align="center">
  <img src=https://i.imgur.com/SYBlC49.jpg>
</p>

<h1 align="center">Solana Chess</h1>
<p align="center"><strong>Chess Engine written in Anchor Framework</strong></p>

<div align="center">

  <a href="https://opensource.org/licenses/MIT">![License](https://img.shields.io/badge/License-MIT-yellow.svg)</a>  

</div>

## Installation

```sh
anchor build
anchor deploy
```

## Testing

```sh
anchor test
// OR
cargo run ./client/
```

## Note 

Anchor will generate a broken idl, use the one provided in `/idl`

## Features
|         Feature         | Implemented |
|-------------------------|:-----------:|
| Check Legal Moves       |      ✅     |
| Checkmate               |      ✅     |
| Enpassant               |      ✅     |
| Castling                |      ✅     |
| Promotion               |      ✅     |
| Elo                     |      ✅     |
| Time Control            |      ❌     |


## Propositions
|         Feature         | Implemented |
|-------------------------|:-----------:|
| Tokens Wagers           |      ❌     |
| Elo NFTs                |      ❌     |
| Chess Bot with Clockwork|      ❌     |
| Tournaments             |      ❌     |
