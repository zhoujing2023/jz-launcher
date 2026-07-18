#!/usr/bin/env bash

set -euo pipefail

readonly SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
readonly PROJECT_ROOT="$(cd -- "${SCRIPT_DIR}/.." && pwd)"

cd "${PROJECT_ROOT}"

output_dir="${1:-${PROJECT_ROOT}/dist}"
git_ref="${2:-HEAD}"

for command_name in git gzip awk mktemp; do
    if ! command -v "${command_name}" >/dev/null 2>&1; then
        echo "错误：缺少必要命令：${command_name}" >&2
        exit 1
    fi
done

if ! git rev-parse --verify "${git_ref}^{commit}" >/dev/null 2>&1; then
    echo "错误：Git 引用不存在：${git_ref}" >&2
    exit 1
fi

version="$({ git show "${git_ref}:launcher-gui/Cargo.toml" || true; } \
    | awk -F '"' '/^version[[:space:]]*=[[:space:]]*"/ { print $2; exit }')"

if [[ -z "${version}" ]]; then
    echo "错误：无法从 launcher-gui/Cargo.toml 读取版本号" >&2
    exit 1
fi

readonly package_name="jz-launcher-${version}"
readonly archive_name="${package_name}.tar.gz"

mkdir -p "${output_dir}"
output_dir="$(cd -- "${output_dir}" && pwd)"
readonly archive_path="${output_dir}/${archive_name}"

temporary_archive="$(mktemp "${output_dir}/.${archive_name}.XXXXXX")"
trap 'rm -f -- "${temporary_archive}"' EXIT

git archive \
    --format=tar \
    --prefix="${package_name}/" \
    "${git_ref}" \
    | gzip -n -9 >"${temporary_archive}"

mv -- "${temporary_archive}" "${archive_path}"
trap - EXIT

echo "源码包已生成：${archive_path}"
echo "Git 引用：${git_ref} ($(git rev-parse --short "${git_ref}^{commit}"))"

if command -v sha256sum >/dev/null 2>&1; then
    sha256sum "${archive_path}"
fi
