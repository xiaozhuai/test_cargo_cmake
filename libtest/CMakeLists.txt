cmake_minimum_required(VERSION 3.15)
project(libtest C)

add_library(test test.c)

install(TARGETS test EXPORT test)
