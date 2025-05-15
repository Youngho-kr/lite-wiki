#!/bin/bash
set -e


if [ ! -f /data/.initialized ]; then

    # 초기 docs 복사
    cp -r /lite-wiki/data/docs/* /data/docs/ 2>/dev/null || true

    # 초기 uploads 복사
    cp -r /lite-wiki/data/uploads/* /data/uploads/ 2>/dev/null || true

    [ ! -f /data/users.json ] && cp /lite-wiki/data/users.json /data/users.json 2>/dev/null || true
    [ ! -f /data/settings.json ] && cp /lite-wiki/data/settings.json /data/settings.json 2>/dev/null || true

    # 초기화 완료 마커 생성
    touch /data/.initialized
    echo "Initialization complete."
else
    echo "Data volume already initialized. Skipping initial data copy."
fi

exec lite-wiki