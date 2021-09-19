#!/usr/bin/env zsh

bin=~/Develop/Rust/jack/target/debug/jack

$bin error    'Error message'
$bin warn     'Warn message'
$bin -b2 -a2 warn     'Warn message with 2 empty lines above and below'
$bin info     'Info message'
$bin debug    'Debug message'
$bin verbose  'Verbose message'

$bin success  'Success message'

for i in {1..5}; do
  $bin progress "[$i] In progress ..."
  sleep 0.7
done
$bin success 'Done'
