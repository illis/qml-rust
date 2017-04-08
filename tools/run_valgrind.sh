#!/bin/sh
valgrind --log-file=valgrind.log --leak-check=full --suppressions=qquickview.supp $1

