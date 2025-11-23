# list all available subcommands
_default:
  @just --list

server:
  cargo run -p tserver

client:
  cargo run -p tclient
