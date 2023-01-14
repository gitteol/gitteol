# 깃털

깃털은 [엔트리](https://playentry.org/) 런타임/인터프리터/플레이어입니다.
[Rust 프로그래밍 언어](https://www.rust-lang.org/)와 [Bevy 엔진](https://bevyengine.org/)을 이용해 개발되고 있습니다.

> 참고: 깃털은 아직 매우 초기 단계의 프로젝트로 계속해서 개발 중입니다.

## 시연 영상

[오브젝트를 중심으로 회전하기 작품](https://playentry.org/project/61e032147f4507001a93e812)을 깃털로 실행한 모습입니다.

https://user-images.githubusercontent.com/35953764/212460743-616d815e-a9da-4553-9c5a-b90327371673.mov

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

현재 깃털은 `assets/project.ent`의 프로젝트 데이터를 읽어서 작품을 실행시킵니다.
실행하고 싶은 엔트리 프로젝트의 파일(`.ent`)로 `assets/project.ent` 파일을 교체하면 해당 작품을 실행시킬 수 있습니다.

다만 아직 깃털이 지원하지 않는 블록을 사용할 경우 작품을 실행할 수 없습니다. 깃털이 지원하는 블록들은 [여기서](/src/blocks) 확인할 수 있습니다.

## 문서
[이 링크](https://gitteol.github.io/book/)를 확인하세요.

## 기여
이 프로젝트에 관심을 가져주셔서 감사합니다! 기여는 언제든지 환영합니다. 편하게 이슈나 PR를 남겨주세요!

