#!/bin/bash
set -e

# 기본 데이터 복사 (없을 때만)
if [ ! -f /data/users.json ]; then
    cp /lite-wiki/data/users.json /data/users.json
fi

if [ ! -f /data/settings.json ]; then
    cp /lite-wiki/data/settings.json /data/settings.json
fi

if [ ! "$(ls -A /data/docs)" ]; then
    cp -r /lite-wiki/data/docs/* /data/docs/
fi

if [ ! "$(ls -A /data/uploads)" ]; then
    cp -r /lite-wiki/data/uploads/* /data/uploads/
fi

exec lite-wiki