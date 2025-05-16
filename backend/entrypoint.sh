#!/bin/bash
set -e

#!/bin/bash
set -e

# 기본 데이터 복사 (없을 때만)

# users.json
if [ ! -f /data/config/users.json ]; then
    mkdir -p /data/config
    cp /lite-wiki/data/config/users.json /data/config/users.json
fi

# settings.json
if [ ! -f /data/config/settings.json ]; then
    mkdir -p /data/config
    cp /lite-wiki/data/config/settings.json /data/config/settings.json
fi

if [ -d /lite-wiki/data/docs ] && [ -z "$(ls -A /data/docs 2>/dev/null)" ]; then
    cp -r /lite-wiki/data/docs/* /data/docs/ || true
fi

if [ -d /lite-wiki/data/uploads ] && [ -z "$(ls -A /data/uploads 2>/dev/null)" ]; then
    cp -r /lite-wiki/data/uploads/* /data/uploads/ || true
fi

exec lite-wiki