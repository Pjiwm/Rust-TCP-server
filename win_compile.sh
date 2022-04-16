#!/bin/bash
docker build . -t rust_cross_compile/windows -f Dockerfile.windows
docker run --rm -ti -v `pwd`:/app rust_cross_compile/windows
