#!/usr/bin/env zsh

bin=~/Develop/Rust/jack/target/debug/jack

$bin error    'Error message'
$bin warn     'Warn message'
$bin info     'Info message'
$bin debug    'Debug message'
$bin verbose  'Verbose message'

$bin success  'Success message'
