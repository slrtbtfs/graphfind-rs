[![build status](https://github.com/slrtbtfs/graphfind-rs/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/slrtbtfs/graphfind-rs/actions) [![crates.io](https://img.shields.io/crates/v/graphfind-rs)](https://crates.io/crates/graphfind-rs)
# graphfind-rs

Library for finding patterns in graphs.

### External Dependencies

The function `VizDotGraph::print_to_svg` depends on `graphviz`, available at https://graphviz.org/download/. Without this package,
calling this function always returns an `Error`.
