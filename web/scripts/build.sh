#!/usr/bin/env bash
set -eux

cargo check
trunk build --release
