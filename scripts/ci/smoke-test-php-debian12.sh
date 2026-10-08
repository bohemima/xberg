#!/usr/bin/env bash
set -euo pipefail

artifact="${1:?usage: smoke-test-php-debian12.sh <php-archive.tgz> <php-version>}"
php_version="${2:?usage: smoke-test-php-debian12.sh <php-archive.tgz> <php-version>}"

[ -f "$artifact" ] || {
  echo "smoke-test-php-debian12: artifact does not exist: $artifact" >&2
  exit 1
}
artifact="$(cd "$(dirname "$artifact")" && pwd)/$(basename "$artifact")"

docker run --rm \
  --volume "$artifact:/tmp/xberg.tgz:ro" \
  "php:${php_version}-cli-bookworm" \
  sh -euc '
    apt-get update -qq
    DEBIAN_FRONTEND=noninteractive apt-get install -y -qq --no-install-recommends libheif1 >/dev/null
    mkdir /tmp/xberg-extension
    tar -xzf /tmp/xberg.tgz -C /tmp/xberg-extension
    extension_count="$(find /tmp/xberg-extension -type f -name xberg.so -print | wc -l | tr -d " ")"
    [ "$extension_count" = 1 ] || {
      echo "expected exactly one xberg.so, found $extension_count" >&2
      exit 1
    }
    extension="$(find /tmp/xberg-extension -type f -name xberg.so -print -quit)"
    php -n -d "extension=$extension" --ri xberg
  '
