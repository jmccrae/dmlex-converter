DMLEX Converter and Validator
=============================

This is a converter and validator for the [DMLEX](https://github.com/oasis-tcs/lexidma)
format

Packages
--------

* `dmlex`: A library in Rust for converting and validating DMLEX
* `dmlex-cli`: A command-line conversion tool
* `dmlex-web`: A web-based converter and validator. This is deployed at 
   https://jmccrae.github.io/dmlex-converter/

Installing
----------

This project can be built with Cargo

    cargo build

The web components are built with Trunk

    trunk serve
