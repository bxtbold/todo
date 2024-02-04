#!/bin/bash

todo() {
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

complete -F _todo_completion todo
