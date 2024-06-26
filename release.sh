#!/bin/bash

OLD_VERSION=$(cat Cargo.toml | grep '^version = ".*"$' | cut -d'"' -f2)
MAJOR=$(echo $OLD_VERSION | cut -d'.' -f1)
MINOR=$(echo $OLD_VERSION | cut -d'.' -f2)
PATCH=$(echo $OLD_VERSION | cut -d'.' -f3)

NEW_PATCH=$((PATCH+1))
GEN_VERSION="$MAJOR.$MINOR.$NEW_PATCH"
echo "Old version was: $OLD_VERSION"
read -p "Choose next version? (Default is $GEN_VERSION): " NEW_VERSION

if [[ -z $NEW_VERSION ]]; then
  NEW_VERSION=$GEN_VERSION
fi

NEW_VERSION_VALID=$(echo $NEW_VERSION | grep -E '^[0-9]+\.[0-9]+\.[0-9]+$' | wc -l)
if [[ $NEW_VERSION_VALID -ne 1 ]]; then
  echo "Provided version '$NEW_VERSION' is not valid."
  exit 1
fi

echo "--- Releasing version '$NEW_VERSION' ---"

echo "Replacing version in Cargo.toml..."
sed -i "s/^version = \".*\"$/version = \"$NEW_VERSION\"/g" Cargo.toml

echo "Replacing version in README.md"
sed -i "s/$OLD_VERSION/$NEW_VERSION/g" README.md

echo "Building docker image..."
docker buildx \
  build \
  --push \
  --platform "linux/arm64,linux/amd64" \
  --tag jnatten/domeneshop-ddns:$NEW_VERSION \
  .

RETURN_CODE=$?

if [[ $RETURN_CODE -ne 0 ]]; then
  echo "Something went wrong when publishing docker image, exiting without committing..."
  exit 1
fi

echo "Comitting release number..."
git add Cargo.toml README.md
git commit -m "VERSION: $NEW_VERSION"

echo "Tagging git commit..."
git tag "v$NEW_VERSION"


read -p "Everything seems okay from here, push the changes? [Y/n]: " YESNO

if [[ $YESNO = "" ]]; then
  YESNO = "Y"
fi

YESNO=${YESNO^^} # TO UPPERCASE
if [[ $YESNO = "Y" ]]; then
  git push
fi

echo "Releasing done!"
