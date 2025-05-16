# Lite Wiki
> **Lite Wiki**는 Rust로 작성한 경량화 마크다운 기반 웹 위키 시스템입니다.
> 설정이 간편하고, 마크다운 문서 관리와 이미지 업로드, 태그 분류, 문서 변경 내역 조회 등을 지원합니다.
- 백엔드: Rust
- 에디터: Toast UI Editor
- 데이터 저장: 로컬 파일 시스템 (.md)
- 배포: Docker

## 주요 기능
- 마크다운(.md) 기반 문서 편집 및 관리
- Toast UI Editor를 사용한 웹 기반 에디터
- 이미지 업로드 지원 (PNG, JPG, GIF, WEBP 등)
- JWT 인증 기반 사용자 관리
- 태그 관리 및 태그 기반 검색
- 문서 변경 내역 저장
- 관리자 페이지 (사용자 관리 및 설정)
- Docker로 간편한 배포

## 구조
```
lite-wiki/
├── backend/            # Rust 서버 소스코드 및 Dockerfile
│   ├── src/
│   ├── static/         # HTML/CSS 정적 리소스
│   ├── data/           # 문서, 이미지 파일 및 설정 파일
│   ├── Cargo.lock
│   ├── Cargo.toml
│   ├── Dockerfile
│   └── entrypoint.sh   # 데이터 초기화 스크립트
├── docker-compose.yml  # 실행 환경 정의
└── README.md
```

## 배포
### 1. 서버 준비
- OS: Ubuntu 20.04 이상 (권장)
- 필수 포트 오픈: 80(HTTP), 443(HTTPS)
- 도메인 준비

### 2. 소스 코드 다운로드
```bash
git clone [https://](https://github.com/Youngho-kr/lite-wiki.git)
cd lite-wiki
```

### 3. 환경 변수 및 포트 설정
`docker-compose.yml` 파일의 환경변수 값 설정
```yaml
ports:
      - "3001:3000"                     # 사용 도메인으로 수정 (e.g. 외부 포트 8080 사용시: "8080:3000")
environment:
      BASE_URL: 0.0.0.0:3000            # 실제 도메인으로 수정
      JWT_SECRET_KEY: your_secret_key   # 반드시 강력한 비밀키로 변경
      DOCS_PATH: /data/docs
      UPLOADS_PATH: /data/uploads
      USER_DB_PATH: /data/users.json
      SETTINGS_PATH: /data/settings.json
```
- `BASE_URL`: 실제 도메인으로 설정
- `JWT_SECRET_KEY`: **반드시** 강력한 키로 수정

### 4. Docker 실행 
```bash
docker compose up --build -d
```

### 5. NGINX 설정 (선택)
```nginx
server {
      listen 80;
      server_name localhost;  # 실제 도메인 설정 시 변경 (e.g., wiki.example.com)

	client_max_body_size 100M;

      # Static Files (CSS, JS, Images)
      location /wiki/static/ {
            proxy_pass http://localhost:3001/static/;
            proxy_http_version 1.1;
            proxy_set_header Host $host;
            proxy_set_header X-Real-IP $remote_addr;
            proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
            proxy_set_header X-Forwarded-Proto $scheme;
      }

      # Application Proxy
      location /wiki/ {
            proxy_pass http://localhost:3001/;
            proxy_http_version 1.1;
            proxy_set_header Host $host;
            proxy_set_header X-Real-IP $remote_addr;
            proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
            proxy_set_header X-Forwarded-Proto $scheme;
            proxy_request_buffering off;

            rewrite ^/wiki/(.*)$ /$1 break;
      }
    }
```


### 5. 웹 접속
`BASE_URL`에 작성한 링크로 접속

기본 관리자 계정으로 로그인 후 비밀번호 변경
- ID: admin
- PW: 1234

| 경로 | 설명 |
|------|------|
| `/` | 위키 메인 페이지 |
| `/docs` | 전체 문서 목록 |
| `/tags` | 전체 태그 목록 |
| `/admin` | 관리자 페이지 |
| `/문서명` | 문서 내용 조회 |
| `/edit/문서명` | 문서 편집 |
| `/random` | 무작위 문서 이동 |

### 6. 사용자 회원가입
새로운 사용자 회원가입 후 관리자 계정으로 `/admin` 접속 후 권한 부여

