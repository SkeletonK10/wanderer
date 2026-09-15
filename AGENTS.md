# wanderer

Rust로 만드는 텍스트 어드벤처 게임입니다.
작성자는 Rust가 처음이지만 OCaml을 한 학기 배워서 Option/Result, 패턴 매칭,
표현식 지향은 익숙합니다. 소유권과 빌림이 새로 배우는 부분입니다.

## 가장 중요한 요청 — 코드를 대신 쓰지 마세요

이 프로젝트의 목적은 작성자가 Rust를 배우는 것입니다.

- 파일을 직접 수정하지 마세요. `.claude/settings.json`에서 Edit/Write/Bash를 막아뒀습니다.
- cargo 명령도 작성자가 직접 칩니다. 빌드 결과가 필요하면 요청하세요.
- 답이 되는 코드를 통째로 보여주지 마세요. 함수 시그니처, 쓸 만한 표준 라이브러리
  함수 이름, 빠뜨리기 쉬운 함정 정도의 힌트가 좋습니다.
- 작성자가 코드를 써서 보여주면 리뷰해 주세요. 버그와 관용적이지 않은 부분을 짚어주면 됩니다.
- "그냥 짜줘"라고 해도 한 번은 되물어 주세요.

## 게임 컨셉

- 터미널에서 `wd look`, `wd go door` 처럼 명령어 하나씩 실행. REPL 아님.
  세이브 파일이 상태를 이어줌
- 여러 세계관을 넘나드는 탐험 게임
- 첫 세계는 중세 판타지. 이종족 없이 인간만 등장. 다른 세계는 나중에 확장
- 보이는 출구는 방 설명에 표시. 숨은 통로는 퍼즐로 단어 조각을 모아 직접 입력해서 진입
- 방향은 방위에 한정하지 않고 임의 문자열 (door, rift 등)

## 목표 구조 (나중에 도달할 곳, 지금은 main.rs 하나)

- 월드 정의는 TOML로 분리해 `include_str!`로 임베드, 세이브는 JSON
- `Command`(시도)와 `Event`(결과)를 분리
- `apply(world, state, cmd) -> (State, Vec<Event>)` 를 순수 함수로 유지. IO는 바깥에

## 현재 상태

0~1단계 완료: cargo 프로젝트, git(main 브랜치), GitHub 연결, `env::args()` 인자 받기.
2단계 진행 중. `read_save()`, `describe(room)`, `next_room(room, dir)` 작성 완료.
방은 home, void 두 개.

## 남은 것

- 2단계 마무리: main에서 배선 — `next_room`이 Some이면 `fs::write`로 저장 후
  `describe` 출력, None이면 거절. void → home 복귀 경로 추가
- 3단계: `Command` enum, 파싱 함수 분리, help
- 4단계: `State` 구조체, serde, JSON 세이브, take/inventory
- 5단계: 월드를 TOML로 분리, HashMap 인덱싱, 출구 ID 검증
- 6단계: 두 번째 세계. `RoomId`를 `(WorldId, String)`으로, 세계별 진행 상황 분리

`unwrap()`은 지금 단계에선 그냥 씁니다. 나중에 정리할 예정입니다.
