# Lite Wiki
**Lite Wiki**는 실시간 편집이 가능한 가벼운 마크다운 기반 위키 시스템입니다.

## 주요 기능
- 마크다운(.md) 기반 문서 편집 및 뷰어
- Toast UI Editor를 사용한 웹 기반 에디터
- WebSocket 기반 실시간 문서 동기화
- Docker로 구성된 로컬/서버 실행 환경

## 기술 스택
- Rust
- Toast UI Editor / Viewer
- Docker, docker-compose

## 구조
```
lite-wiki/
├── frontend/
├── backend/
├── data/
├── docker/
└── docker-compose.yml
```

## 실행 방법 (예정)
```bash
docker-compose up --build
```

