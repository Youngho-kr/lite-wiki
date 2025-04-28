# Lite Wiki
> **Lite Wiki**는 Rust로 작성한 경량화 마크다운 기반 웹 위키 시스템입니다.
- 백엔드: Rust
- 에디터: Toast UI Editor
- 데이터 저장: 로컬 파일 시스템 (.md)
- 배포: Docker

## 주요 기능
- 마크다운(.md) 기반 문서 편집 및 관리
- Toast UI Editor를 사용한 웹 기반 에디터
- Docker로 구성된 로컬/서버 실행 환경

## 구조
```
lite-wiki/
├── backend/            # Rust 서버 소스코드 및 Dockerfile
│   ├── src/
│   ├── static/         # HTML 파일
│   ├── Cargo.lock
│   ├── Cargo.toml
│   ├── Dockerfile
│   └── .env
├── data/               # 문서 및 템플릿 디렉토리
├── docker/             # 배포 관련 설정 파일 (예정)
├── docker-compose.yml  # 실행 환경 정의
└── README.md
```

## 시작
### 환경 변수 설정
```
# .env
DATA_PATH=./data/docs
TEMPLATE_PATH=./data/templates
```
### Docker 배포
```
docker build -t litewiki .
docker run -d -p 3000:3000 --name litewiki_app litewiki
```

## 웹 접속 (수정 예정)
http://localhost:3000

| 경로 | 설명 |
|------|------|
| `/index` | 기본 문서 |
| `/docs` | 전체 문서 목록 |
| `/edit/문서명` | 문서 편집 |
| `/templates` | 템플릿 목록 |