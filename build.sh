#!/usr/bin/env sh

(cd string_length && cargo component build --release)
(cd app && cargo component build)