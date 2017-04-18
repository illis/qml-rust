#!/bin/bash
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
valgrind --log-file=${SCRIPT_DIR}/valgrind.log --leak-check=full --suppressions=${SCRIPT_DIR}/qquickview.supp $1

