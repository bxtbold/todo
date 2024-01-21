#!/bin/bash

_todo_completion() {
    if [ "$2" == "add" ]; then
        COMPREPLY=( $(echo "Adding $3") )
    elif [ "$2" == "done" ]; then
        COMPREPLY=( $(echo "Completed $3") )
    elif [ "$2" == "rm" ]; then
        COMPREPLY=( $(echo "Removing $3") )
    elif [ "$2" == "list" ]; then
        COMPREPLY=( $(echo "Listing") )
    elif [ "$2" == "sort" ]; then
        COMPREPLY=( $(echo "Sorting") )
    else
        COMPREPLY=( $(compgen -W "add done rm list sort" -- "$2") )
    fi
}

complete -F _todo_completion todo
