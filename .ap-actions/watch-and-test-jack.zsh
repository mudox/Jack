#!/usr/bin/env zsh

dir="${HOME}/Develop/Rust/jack"

fswatch -or0 "${dir}" | xargs -0 -n1 -I{} watch-run "${dir}/.ap-actions/test-jack.zsh"
