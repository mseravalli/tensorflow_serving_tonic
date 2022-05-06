#!/bin/bash -xe

mkdir -p build_tmp

pushd build_tmp
git clone https://github.com/tensorflow/tensorflow.git
git clone https://github.com/tensorflow/serving.git   
popd

ln -s ${PWD}/build_tmp/serving/tensorflow_serving ./
ln -s ${PWD}/build_tmp/tensorflow/tensorflow ./

cargo build

rm -rf tensorflow_serving
rm -rf tensorflow
rm -rf build_tmp
