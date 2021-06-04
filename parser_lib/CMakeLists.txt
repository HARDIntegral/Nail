cmake_minimum_required(VERSION 3.0)
project(parser_lib C)
add_library(parser_lib STATIC parse_wrapper.c)
install(TARGETS parser_lib DESTINATION .)