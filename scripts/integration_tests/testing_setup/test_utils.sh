#!/usr/bin/env bash

function assert_eq() {
    if [ "$1" == "$2" ]
    then 
        echo -e "\033[0;32m OK \033[0m"
    else 
        echo -e "\033[0;31m FAIL"
        echo "assert_eq"
        echo "expects" $2
        echo "got " $1 " \033[0m"
        exit -1
    fi
}

function assert_ne() {
    if [ "$1" != "$2" ]
    then 
        echo -e "\033[0;32m OK \033[0m"
    else 
        echo -e "\033[0;31m FAIL"
        echo "assert_ne"
        echo "expects" $2
        echo "got " $1 " \033[0m"
        exit -1
    fi
}

function test_passed() {
    echo -e "\033[0;32m test:" $1 "has passed"
}