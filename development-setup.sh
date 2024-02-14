#!/usr/bin/env bash

# check if 'pwd' is the root of the project
if [ ! -f "Cargo.toml" ]; then
  echo "Only run this inside the root of the project."
  exit 1
fi

echo "This will install the necessary tools to run dioxus and grass."
read -p "Are you sure you want to continue? (y/n)" -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]
then
  exit 1
fi

# Setup a prompt for using tmux
read -p "Do you want to use tmux to run dioxus serve and grass in separate panes? (Y/N)" -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    USE_TMUX=true
else
    USE_TMUX=false
fi

# Install grass if not installed
if ! command -v grass &> /dev/null; then
  cargo install grass
  echo "Grass is installed."
fi

# Install dioxus-cli if not installed
if ! command -v dx &> /dev/null; then
  cargo install dioxus-cli
  echo "dioxus-cli is installed."
fi

# Function to run dioxus serve and grass in separate tmux panes
run_with_tmux() {
    # tmux split-window -h
    tmux send-keys "dx serve --hot-reload --port 2020 --open" Enter
    tmux split-window -v
    tmux send-keys "while true; do cp ./public/style.css ./public/style.css.bak; echo 'style.css was backup to style.css.bak'; grass ./sass/index.scss ./public/style.css; echo 'Sass was compiled.'; inotifywait -e modify ./sass/*; echo ''; echo 'File was modified. Sass will be recompiled.'; echo ''; done" Enter
}

# If tmux is to be used, run commands in separate panes
if [ "$USE_TMUX" = true ]; then
    # Check if already in a tmux session
    if [ -n "$TMUX" ]; then
        run_with_tmux
    else
        echo "You are not currently in a tmux session. Please start a tmux session and run this script again."
        exit 1
    fi
else
    # Run commands without tmux
    dx serve --hot-reload --port 2020 --open &
    while true; do
        cp ./public/style.css ./public/style.css.bak
        echo "style.css was backup to style.css.bak"
        grass ./sass/index.scss ./public/style.css
        echo "Sass was compiled."
        inotifywait -e modify ./sass/*
        echo ""
        echo "File was modified. Sass will be recompiled."
        echo ""
    done
fi
