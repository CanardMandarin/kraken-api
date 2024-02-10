# kraken-api

Another unofficial Rust Library for the [Kraken API](https://docs.kraken.com/rest/#).


## Installation

Add the lib as a project dependency in you `Cargo.toml`.

```
[dependencies]
kraken-api = { git = "https://github.com/CanardMandarin/kraken-api" }
```


## Basic Usage

The library provides the necessary building blocks to interact with the Kraken API. It mostly exposes three main traits that you can implement on your types to build logic that fits your exact needs:

- The `Client` / `AsyncClient` traits can be implemented on tour types to turn them into working instance that can communcicate with the API. Default `Kraken`/`AsyncKraken` clients are already implemented.

- The `Endpoint` trait can be implemented on your types to turn them into actual endpoints that your application needs to interact with. Not all the endpoints are currently implemented but it's really easy to add new ones or adapt existing ones to fit your exact needs.

- The `Query` / `AsyncQuery` traits are implemented on all types that implement `Endpoint` and expose the `query` / `query_async` methods in which the `Client` / `AsyncClient` are injected to perform the requests.