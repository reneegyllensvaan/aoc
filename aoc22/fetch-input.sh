#!/usr/bin/env zsh
day=$1
session=$(cat ./aoc-session)
curl "https://adventofcode.com/2022/day/$day/input" \
  --compressed \
  -H 'User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:120.0) Gecko/20100101 Firefox/120.0' \
  -H 'Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8' \
  -H 'Accept-Language: en-US,en;q=0.5' \
  -H 'Accept-Encoding: gzip, deflate, br' \
  -H 'Referer: https://adventofcode.com/2023/day/8' \
  -H 'DNT: 1' \
  -H 'Connection: keep-alive' \
  -H "Cookie: session=$session" \
  -H 'Upgrade-Insecure-Requests: 1' \
  -H 'Sec-Fetch-Dest: document' \
  -H 'Sec-Fetch-Mode: navigate' \
  -H 'Sec-Fetch-Site: same-origin' \
  -H 'Sec-Fetch-User: ?1' \
  -H 'Pragma: no-cache' \
  -H 'Cache-Control: no-cache' \
  -H 'TE: trailers' \
  | tee "input/day$(printf '%02d' $day)"
