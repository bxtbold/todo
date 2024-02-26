#!/bin/bash

create_todo_history() {
    if [ ! -x "$TODO_HISTORY_PATH" ]; then
        mkdir $TODO_HISTORY_PATH
    fi
}


todo() {
    current_dir=$(pwd)

    script_path=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
    cd $script_path/..

    local binary_debug_path="$(pwd)/target/debug/todo"
    local binary_release_path="$(pwd)/target/release/todo"

    if [ -x "$binary_debug_path" ]; then
        "$binary_debug_path" "$@"
    elif [ -x "$binary_release_path" ]; then
        "$binary_release_path" "$@"
    else
        echo "Error: Binary not found. Building..."
        cargo build --release
        "$binary_release_path" "$@"
    fi

    cd $current_dir
}

_todo_completion() {
    if [ "$2" == "add" ]; then
        COMPREPLY=( )
    elif [ "$2" == "done" ]; then
        COMPREPLY=( )
    elif [ "$2" == "gui" ]; then
        COMPREPLY=( )
    elif [ "$2" == "rm" ]; then
        COMPREPLY=( )
    elif [ "$2" == "list" ]; then
        COMPREPLY=( )
    elif [ "$2" == "sort" ]; then
        COMPREPLY=( )
    else
        COMPREPLY=( $(compgen -W "add done gui rm list sort" -- "$2") )
    fi
}


export TODO_HISTORY_PATH="$HOME/todo_history"
create_todo_history
complete -F _todo_completion todo
