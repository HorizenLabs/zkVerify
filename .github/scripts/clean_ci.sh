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


# This script frees up 28 GB of disk space by deleting unneeded packages and 
# cached docker images.
#
echo "=============================================================================="
echo "Freeing up disk space on CI system"
echo "=============================================================================="

echo "********************************* START SIZE *********************************"
df -h

# echo "Removing Android library"
# sudo rm -rf /usr/local/lib/android
# echo "Removing .NET runtime"
# sudo rm -rf /usr/share/dotnet
# echo "Removing Haskell runtime"
# sudo rm -rf /opt/ghc
# sudo rm -rf /usr/local/.ghcup

PACKAGES_TO_REMOVE_COUNT=10
echo "Listing ${PACKAGES_TO_REMOVE_COUNT} largest packages"
dpkg-query -Wf '${Installed-Size}\t${Package}\n' | sort -n | tail -n ${PACKAGES_TO_REMOVE_COUNT}
PACKAGES_TO_REMOVE_LIST=$(dpkg-query -Wf '${Installed-Size}\t${Package}\n' | sort -n | tail -n ${PACKAGES_TO_REMOVE_COUNT} | awk '{printf "%s ", $2}')
echo "Removing large packages"
sudo apt-get remove -y ${PACKAGES_TO_REMOVE_LIST} --fix-missing
sudo apt-get autoremove -y
sudo apt-get clean

echo "************************* AFTER PACKAGES CLEAN SIZE *************************"
df -h

echo "Removing docker images"
docker image prune -a -f

echo "********************************* END  SIZE *********************************"
df -h
date
