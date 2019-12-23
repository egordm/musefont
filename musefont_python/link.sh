#!/bin/sh

ROOT_PATH="$(realpath "$(dirname "$0")/..")"
BUILD_DEBUG="target/debug/libmusefont_python.so"
BUILD_RELEASE="target/release/libmusefont_python.so"

TARGET_DEBUG="musefont_python/libs/debug/musefont_python.so"
TARGET_RELEASE="musefont_python/libs/release/musefont_python.so"

if [ -f "${ROOT_PATH}/${BUILD_DEBUG}" ]; then
  echo "Linking debug"
  rm "${ROOT_PATH}/${TARGET_DEBUG}"
  ln -s "${ROOT_PATH}/${BUILD_DEBUG}" "${ROOT_PATH}/${TARGET_DEBUG}"
fi

if [ -f "${ROOT_PATH}/${BUILD_RELEASE}" ]; then
  echo "Linking release"
  rm "${ROOT_PATH}/${TARGET_RELEASE}"
  ln -s "${ROOT_PATH}/${BUILD_RELEASE}" "${ROOT_PATH}/${TARGET_RELEASE}"
fi