#!/bin/bash
set -ex

curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly -y
export PATH="$HOME/.cargo/bin:$PATH"

cd /io

for PYBIN in /opt/python/{cp37-cp37m,cp36-cp36m,cp38-cp38}/bin; do
    export PYTHON_SYS_EXECUTABLE="$PYBIN/python"
#  pin wheel version
# https://github.com/pypa/auditwheel/issues/102
    "${PYBIN}/pip" install -U setuptools wheel==0.31.1 setuptools-rust
    "${PYBIN}/python" setup.py sdist bdist_wheel
done

for whl in dist/*.whl; do
    auditwheel repair "$whl" -w dist/
    rm "$whl"
done

rm -r build/
rm -r *.egg-info/*
