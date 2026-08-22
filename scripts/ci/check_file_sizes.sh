#!/usr/bin/env bash
# SPDX-FileCopyrightText: 2026 Kevin Monaghan
# SPDX-License-Identifier: MIT-0

set -euo pipefail

readonly SOURCE_TARGET=400
readonly SOURCE_HARD_MAX=600
readonly ROOT_MODULE_HARD_MAX=200
readonly TEST_HARD_MAX=800

repository_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "${repository_root}"

roots=()
for candidate in src tests; do
  if [[ -d "${candidate}" ]]; then
    roots+=("${candidate}")
  fi
done

if [[ ${#roots[@]} -eq 0 ]]; then
  printf '%s\n' 'no Rust source or test directories found; file-size check has nothing to inspect'
  exit 0
fi

status=0
warnings=0
checked=0
generated=0

report_error() {
  local file=$1
  local message=$2
  if [[ "${GITHUB_ACTIONS:-false}" == "true" ]]; then
    printf '::error file=%s::%s\n' "${file}" "${message}"
  else
    printf 'error: %s: %s\n' "${file}" "${message}" >&2
  fi
}

report_warning() {
  local file=$1
  local message=$2
  if [[ "${GITHUB_ACTIONS:-false}" == "true" ]]; then
    printf '::warning file=%s::%s\n' "${file}" "${message}"
  else
    printf 'warning: %s: %s\n' "${file}" "${message}" >&2
  fi
}

while IFS= read -r -d '' file; do
  checked=$((checked + 1))

  if [[ "/${file}/" == */generated/* ]]; then
    header=$(head -n 20 -- "${file}")
    if grep --fixed-strings --quiet '@generated' <<<"${header}" \
      && grep --fixed-strings --quiet 'Generator:' <<<"${header}" \
      && grep --fixed-strings --quiet 'Source:' <<<"${header}"; then
      generated=$((generated + 1))
      continue
    fi
    report_error "${file}" \
      'generated-path exemption requires @generated, Generator:, and Source: markers in the first 20 lines'
    status=1
  fi

  line_count=$(awk 'END { print NR }' "${file}")
  basename=${file##*/}

  if [[ "${basename}" == "lib.rs" || "${basename}" == "mod.rs" ]]; then
    hard_max=${ROOT_MODULE_HARD_MAX}
    policy='crate/module root'
  elif [[ "${file}" == src/* ]]; then
    hard_max=${SOURCE_HARD_MAX}
    policy='handwritten source'
    if (( line_count > SOURCE_TARGET && line_count <= SOURCE_HARD_MAX )); then
      report_warning "${file}" \
        "${line_count} lines exceeds the ${SOURCE_TARGET}-line source target; split by capability before the ${SOURCE_HARD_MAX}-line hard maximum"
      warnings=$((warnings + 1))
    fi
  else
    hard_max=${TEST_HARD_MAX}
    policy='test'
  fi

  if (( line_count > hard_max )); then
    report_error "${file}" \
      "${line_count} lines exceeds the ${hard_max}-line ${policy} hard maximum (TV-SIZE-01)"
    status=1
  fi
done < <(find "${roots[@]}" -type f -name '*.rs' -print0)

printf 'checked %d Rust files (%d documented generated exemptions, %d target warnings)\n' \
  "${checked}" "${generated}" "${warnings}"

exit "${status}"
