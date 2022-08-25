# 깃털

깃털은 [엔트리](https://playentry.org/) 런타임/인터프리터/플레이어입니다.
[Rust 프로그래밍 언어](https://www.rust-lang.org/)와 [Bevy 엔진](https://bevyengine.org/)을 이용해 개발되고 있습니다.

> 참고: 깃털은 아직 매우 초기 단계의 프로젝트로 계속해서 개발 중입니다.

## 사용법

로컬에서 깃털을 빌드하고 실행해 보려면 우선 Rust를 설치해야 합니다.
[Rust Getting started 가이드](https://www.rust-lang.org/learn/get-started)를 참고해 Rust를 설치하세요.

그리고 이 저장소를 클론하세요.

```sh
git clone https://github.com/gitteol/gitteol.git
cd gitteol
```

그 후 `cargo`를 이용해 깃털을 실행할 수 있습니다.

```sh
cargo run
```

[Bevy Setup 가이드](https://bevyengine.org/learn/book/getting-started/setup/#compile-with-performance-optimizations)를 참고해 디버그 빌드 속도를 높일 수 있습니다.

### 다른 엔트리 작품 실행하기

현재 깃털은 `assets/project.json`의 프로젝트 데이터를 읽어서 작품을 실행시킵니다.
이 파일의 내용을 바꾸면 다른 엔트리 작품을 실행시킬 수 있습니다.

1. 엔트리 프로젝트를 엔트리 파일(`.ent`)로 저장하세요.
2. 해당 파일의 확장자를 `.tgz`로 변경한 뒤 압축을 해제하세요.
3. 압축 해제된 `temp` 디렉터리 내부의 파일들을 모두 복사해 이 프로젝트의 `assets` 디렉터리에 붙여 넣으세요.
4. `cargo run` 명렁어를 실행시키세요.

다만 아직 깃털이 지원하지 않는 블록을 사용할 경우 작품을 실행할 수 없습니다. 깃털이 지원하는 블록들은 [여기서](/src/blocks) 확인할 수 있습니다.

## 문서
[이 링크](https://gitteol.github.io/book/)를 확인하세요.

## 기여
이 프로젝트에 관심을 가져주셔서 감사합니다! 기여는 언제든지 환영합니다. 편하게 이슈나 PR를 남겨주세요!

