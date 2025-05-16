#!/bin/bash
set -e

# 기본 데이터 복사 (없을 때만)
if [ ! -f /data/users.json ]; then
    cp /lite-wiki/data/users.json /data/users.json
fi

if [ ! -f /data/settings.json ]; then
    cp /lite-wiki/data/settings.json /data/settings.json
fi

if [ -d /lite-wiki/data/docs ] && [ -z "$(ls -A /data/docs 2>/dev/null)" ]; then
    cp -r /lite-wiki/data/docs/* /data/docs/ || true
fi

if [ -d /lite-wiki/data/uploads ] && [ -z "$(ls -A /data/uploads 2>/dev/null)" ]; then
    cp -r /lite-wiki/data/uploads/* /data/uploads/ || true
fi

exec lite-wiki