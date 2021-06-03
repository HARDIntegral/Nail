cmake_minimum_required(VERSION 3.0)
project(parser C)
add_library(parser STATIC parse_c.c)
install(TARGETS parser DESTINATION .)