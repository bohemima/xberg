#!/usr/bin/env bash
set -euo pipefail

swap_size_gib="${SWAP_SIZE_GIB:-16}"
runner_temp="${RUNNER_TEMP:-}"

if [[ "$(uname -s)" != "Linux" ]]; then
  echo "benchmark swap setup requires Linux" >&2
  exit 1
fi
if [[ ! "$swap_size_gib" =~ ^[1-9][0-9]*$ ]]; then
  echo "SWAP_SIZE_GIB must be a positive integer, got: $swap_size_gib" >&2
  exit 1
fi
if [[ -z "$runner_temp" || ! -d "$runner_temp" ]]; then
  echo "RUNNER_TEMP must name an existing directory" >&2
  exit 1
fi

swap_file="${runner_temp}/xberg-benchmark-build.swap"
if [[ -e "$swap_file" ]]; then
  echo "refusing to overwrite existing swap path: $swap_file" >&2
  exit 1
fi

available_kib="$(df -Pk "$runner_temp" | awk 'NR == 2 { print $4 }')"
swap_kib=$((swap_size_gib * 1024 * 1024))
reserve_kib=$((8 * 1024 * 1024))
if [[ ! "$available_kib" =~ ^[0-9]+$ ]] || ((available_kib < swap_kib + reserve_kib)); then
  echo "insufficient disk for ${swap_size_gib} GiB swap plus 8 GiB build reserve" >&2
  exit 1
fi

# ~keep Fat LTO for the all-feature ARM CLI can exceed physical memory and starve the runner
# ~keep service. Disk-backed swap preserves the exact release profile and feature set being measured.
sudo fallocate --length "${swap_size_gib}G" "$swap_file"
sudo chmod 600 "$swap_file"
sudo mkswap "$swap_file"
sudo swapon "$swap_file"

if ! sudo swapon --show=NAME --noheadings | awk '{$1=$1};1' | grep -Fxq "$swap_file"; then
  echo "swap activation did not expose the expected path: $swap_file" >&2
  exit 1
fi

echo "activated ${swap_size_gib} GiB benchmark build swap at $swap_file"
free -h
