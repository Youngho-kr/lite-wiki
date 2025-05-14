# Lite Wiki
> **Lite Wiki**는 Rust로 작성한 경량화 마크다운 기반 웹 위키 시스템입니다.
- 백엔드: Rust
- 에디터: Toast UI Editor
- 데이터 저장: 로컬 파일 시스템 (.md)
- 배포: Docker

## 주요 기능
- 마크다운(.md) 기반 문서 편집 및 관리
- Toast UI Editor를 사용한 웹 기반 에디터
- 로컬 파일 시스템을 이용한 데이터 저장
- 이미지 업로드 (gif 포함)
- JWT 인증 기반 사용자 관리
- 태그 관리 및 태그 기반 검색
- 문서 버전 관리
- 관리자 페이지 (사용자 관리 및 설정)
- 문서 검색 및 무작위 문서 탐색
- Docker로 구성된 로컬/서버 실행 환경

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
│   └── .env
├── docker-compose.yml  # 실행 환경 정의
└── README.md
```

## 시작
### 초기 설정
```
# .env
DATA_PATH=./data/docs
TEMPLATE_PATH=./data/templates
USER_DB_PATH=./data/users.json
SETTINGS_PATH=./data/settings.json

JWT_SECRET_KEY=<your_secret_key>
```
1. `JWT_SECRET_KEY` 값을 반드시 설정하세요.
2. 필요한 경우 폴더 경로를 수정하세요.
3. 기본 관리자 계정으로 로그인
    - ID: admin
    - PW: 1234

### Docker로 실행
```
docker compose up --build
```

## 웹 접속 (수정 예정)
http://localhost:3000

| 경로 | 설명 |
|------|------|
| `/` | 위키 메인 페이지 |
| `/docs` | 전체 문서 목록 |
| `/tags` | 전체 태그 목록 |
| `/admin` | 관리자 페이지 |
| `/문서명` | 문서 내용 조회 |
| `/edit/문서명` | 문서 편집 |
| `/random` | 무작위 문서 이동 |