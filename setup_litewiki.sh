#!/bin/bash
set -e

echo "🚀 Lite Wiki 배포 자동화 시작..."

# 1. 필수 정보 입력 받기
read -p "🌍 도메인 이름을 입력하세요 (예: wiki.example.com): " DOMAIN
if [ -z "$DOMAIN" ]; then
    echo "❌ 도메인은 반드시 입력해야 합니다. 종료합니다."
    exit 1
fi

read -sp "🔑 JWT Secret Key를 입력하세요: " JWT_SECRET_KEY
echo ""
if [ -z "$JWT_SECRET_KEY" ]; then
    echo "❌ JWT Secret Key는 반드시 입력해야 합니다. 종료합니다."
    exit 1
fi

# 1-1. 파일 경로 입력 
read -p "📁 문서 저장 경로 (기본값: ./data/docs): " DOCS_PATH
if [ -z "$DOCS_PATH" ]; then
    DOCS_PATH="./data/docs"
    echo "- 문서 저장 경로를 기본값으로 설정: $DOCS_PATH"
fi

read -p "📁 업로드 파일 저장 경로 (기본값: ./data/uploads): " UPLOADS_PATH
if [ -z "$UPLOADS_PATH" ]; then
    UPLOADS_PATH="./data/uploads"
    echo "- 업로드 파일 저장 경로를 기본값으로 설정: $UPLOADS_PATH"
fi

read -p "📁 사용자 DB 파일 경로 (기본값: ./data/users.json): " USER_DB_PATH
if [ -z "$USER_DB_PATH" ]; then
    USER_DB_PATH="./data/users.json"
    echo "- 사용자 DB 파일 경로를 기본값으로 설정: $USER_DB_PATH"
fi

read -p "📁 설정 파일 경로 (기본값: ./data/settings.json): " SETTINGS_PATH
if [ -z "$SETTINGS_PATH" ]; then
    SETTINGS_PATH="./data/settings.json"
    echo "- 설정 파일 경로를 기본값으로 설정: $SETTINGS_PATH"
fi

# 1-2. 포트 설정
read -p "🌐 외부에서 접근할 포트 (기본값: 3001): " EXTERNAL_PORT
EXTERNAL_PORT=${EXTERNAL_PORT:-3001}
echo "- 외부 포트: $EXTERNAL_PORT"

read -p "📦 컨테이너 내부 포트 (기본값: 3000): " INTERNAL_PORT
INTERNAL_PORT=${INTERNAL_PORT:-3000}
echo "- 컨테이너 내부 포트: $INTERNAL_PORT"

# 2. 필수 패키지 설치
sudo apt update
sudo apt install -y docker.io docker-compose nginx certbot python3-certbot-nginx ufw

# 3. Docker 시작
sudo systemctl enable --now docker

# 3-1. Docker Compose 파일 자동 생성
mkdir -p ~/lite-wiki
cd ~/lite-wiki

cat <<EOF > docker-compose.yml
services:
  lite-wiki:
    build: ./backend
    ports:
      - "${CONTAINER_PORT}:${INTERNAL_PORT}"
    environment:
      BASE_URL: https://${DOMAIN}/wiki
      JWT_SECRET_KEY: ${JWT_SECRET_KEY}
      DOCS_PATH: ${DOCS_PATH}
      UPLOADS_PATH: ${UPLOADS_PATH}
      USER_DB_PATH: ${USER_DB_PATH}
      SETTINGS_PATH: ${SETTINGS_PATH}
EOF

# 3-2. Docker 컨테이너 실행
sudo docker compose up --build -d

# 4. NGINX 설정
sudo tee /etc/nginx/sites-available/wiki <<EOF
server {
    listen 80;
    server_name ${DOMAIN};

    location /wiki/ {
        proxy_pass http://localhost:${CONTAINER_PORT}/;
        proxy_set_header Host \$host;
        proxy_set_header X-Real-IP \$remote_addr;
        rewrite ^/wiki/(.*)\$ /\$1 break;
    }
}
EOF

sudo ln -sf /etc/nginx/sites-available/wiki /etc/nginx/sites-enabled/
sudo nginx -t
sudo systemctl reload nginx

# 4-1. HTTPS 인증서 발급 (Let’s Encrypt)
sudo certbot --nginx -d ${DOMAIN} --non-interactive --agree-tos -m admin@${DOMAIN} --redirect

echo ""
echo "✅ Lite Wiki 배포 완료!"
echo "🌍 접속 주소: https://${DOMAIN}/wiki"
echo "📢 관리자 계정: admin / 1234 (로그인 후 반드시 비밀번호를 변경하세요)"