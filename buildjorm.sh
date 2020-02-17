#!/bin/bash
git submodule update --init --recursive
cargo install --force --path jormungandr
cargo install --force --path jcli

