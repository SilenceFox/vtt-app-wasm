#!/usr/bin/env bash

# check if 'pwd' is the root of the project
if [ "$(ls | grep "Cargo.toml")" != "Cargo.toml" ]; then
  echo "Only run this inside the root of the project."
  exit 1
fi

#setup a continue prompt
echo "This will install the necessary tools to run dioxus and grass."
read -p "Are you sure you want to continue? (y/n)" -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]
then
  exit 1
fi
#install grass
cargo install grass
echo "Grass is installed."

cargo install dioxus-cli
echo "Install dioxus-cli."

echo "Starting dioxus serve..."
dx serve --hot-reload --port 2020 --open &

while true; do
    cp ./public/style.css ./public/style.css.bak
    echo "style.css was backup to style.css.bak"
    grass ./sass/* ./public/style.css
    echo "Sass was compiled."
    inotifywait -e modify ./sass/*
    echo ""
    echo "File was modified. Sass will be recompiled."
    echo ""
done

