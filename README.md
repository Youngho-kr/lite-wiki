# Lite Wiki
**Lite Wiki**는 Rust로 작성한 웹 기반 마크다운 위키 시스템입니다.
Docker로 실행 가능하며, 실시간 편집과 템플릿 기능을 제공합니다.

## 주요 기능
- 마크다운(.md) 기반 문서 편집 및 뷰어
- Toast UI Editor를 사용한 웹 기반 에디터
- Docker로 구성된 로컬/서버 실행 환경
- WebSocket 기반 실시간 문서 동기화 (예정)

## 기술 스택
- Rust
- Toast UI Editor / Viewer
- Docker, docker-compose

## 구조
```
lite-wiki/
├── backend/            # Rust 서버 소스코드 및 Dockerfile
├── data/               # 문서 및 템플릿 디렉토리
├── docker/             # 배포 관련 설정 파일 (예정)
└── docker-compose.yml  # 실행 환경 정의
```

## 실행 방법
Lite Wiki는 Docker를 통해 바로 실행 가능합니다.

```
docker compose up -d --build
```

## 웹 접속 (수정 예정)
http://localhost:3000

| 경로 | 설명 |
|------|------|
| `/index` | 기본 문서 |
| `/docs` | 전체 문서 목록 |
| `/edit/문서명` | 문서 편집 |
| `/templates` | 템플릿 목록 |