#! /usr/bin/env bash
set -eu
set -o pipefail

echo "This script packages and publishes the crate to https://crates.io/"
echo "It is meant to run AFTER ci.sh"

# https://pubs.opengroup.org/onlinepubs/9699919799/utilities/getopts.html
yflag=
while getopts y name
do
    case $name in
    y)   yflag=1;;
    ?)   printf "Usage: %s -y\nThe -y option is REQUIRED to confirm publication.\n" $0
         exit 2;;
    esac
done
shift $(($OPTIND - 1))

## `-z` tests if the string has zero length
if [ -z "$yflag" ]; then
    echo "You MUST pass the -y option to confirm that you truly want to PUBLISH."
    exit 1
fi

## Exit if the CRATES_TOKEN env var is not defined
if [ ! -v CRATES_TOKEN ]; then
  echo "The env. variable CRATES_TOKEN is not set."
  exit 3
fi

echo "----------"
CARGO_TOML_VERSION=$(cargo metadata --no-deps --locked --format-version 1 | jq '.packages | .[0] | .version')
echo "Version in Cargo.toml: ${CARGO_TOML_VERSION}"

CRATES_IO_VERSION=$(cargo search mdbook-utils | sed -E 's/^.*"(.*)".*$/\1/')
echo "Version in crates.io: ${CRATES_IO_VERSION}"

# GIT_TAG=$(git describe --tags $(git rev-list --tags --max-count=1))
# echo "Git tag: ${GIT_TAG}"

## TODO check that the last commit has been tagged, that the tag corresponds to the version in Cargo.toml,
## and that the version number is larger than the one in crates.io.
# https://stackoverflow.com/questions/1474115/how-to-find-the-tag-associated-with-a-given-git-commit
# https://stackoverflow.com/questions/1404796/how-can-i-get-the-latest-tag-name-in-current-branch-in-git
# git log --pretty=oneline
# git tag
# git describe --exact-match <commit-id>

echo "----------"
echo "Files that will be packaged:"
cargo package --locked --list  # Print files included in a package.

echo "----------"
echo "Package & publish:"
cargo publish --dry-run --locked --token ${CRATES_TOKEN}

echo "----------"
