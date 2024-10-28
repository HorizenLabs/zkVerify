#!/usr/bin/env bash
# Licensed to the Apache Software Foundation (ASF) under one or more
# contributor license agreements.  See the NOTICE file distributed with
# this work for additional information regarding copyright ownership.
# The ASF licenses this file to You under the Apache License, Version 2.0
# (the "License"); you may not use this file except in compliance with
# the License.  You may obtain a copy of the License at
#
#    http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.


# This script frees up some disk space by deleting unneeded packages and 
# cached docker images.
#
echo "=============================================================================="
echo "Mounting a loopback device on /mnt on CI system"
echo "=============================================================================="

echo "********************************* PRE MOUNTS *********************************"
df -h

DISK_SIZE=60000000000 # 60GB
LO_FILE="/mnt/lodisk"

sudo touch "${LO_FILE}"
sudo fallocate -z -l ${DISK_SIZE} "${LO_FILE}"
ROOT_LOOP_DEV=$(sudo losetup --find --show "${LO_FILE}")
sudo mkfs.ext4 "${ROOT_LOOP_DEV}"
sudo mount "${ROOT_LOOP_DEV}" "${GITHUB_WORKSPACE}"
sudo chown -R runner:runner "${GITHUB_WORKSPACE}"

echo "************************* POST MOUNTS *************************"
df -h
date
