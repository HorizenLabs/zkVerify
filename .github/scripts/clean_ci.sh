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

echo "Listing 100 largest packages"
dpkg-query -Wf '${Installed-Size}\t${Package}\n' | sort -n | tail -n 100
echo "********************************* START SIZE *********************************"
df -h
echo "Removing large packages"
sudo apt-get remove -y '^dotnet-.*'
sudo apt-get remove -y '^temurin-.*'
sudo apt-get remove -y azure-cli google-cloud-cli microsoft-edge-stable google-chrome-stable firefox powershell
sudo apt-get autoremove -y
sudo apt-get clean
echo "************************* AFTER PACKAGES CLEAN SIZE *************************"
df -h
echo "Removing docker images"
docker image prune -a -f
echo "********************************* END  SIZE *********************************"
df -h
date
