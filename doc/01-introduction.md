# RPG Architecture

* Map
* Movement / Animation / Collision Detection
* Entities
* User Interface
* Trigger system - interact with maps and other entities
* Cut Scenes
* World State Tracking
* Inventory System
* Saving and Loading
* Combat System

# 게임의 동작
게임은 표준적으로 60 FPS, 즉 1초에 60번 정보를 처리한 후에 화면에 그리게 된다. 이는 약 0.016666초이다.

게임에서 이 시간안에 처리되어야할 목록은 다음과 같다.

1. Player의 입력
2. AI
3. 적의 이동과 애니메이션
4. 파티클 등의 효과 
5. 사운드 효과 및 음악
6. 사용자 인터페이스 및 텍스트

이후에 화면 Rendering 이 발생하게된다.

# Tools

* Engine (C++, Rust, Go, ...)
* Script Language (Lua, Mun, ...)

# Game Loop

game loop에서는 매 프레임마다 실행되는 update 함수가 있다. update 함수 호출이 끝난 후에는 화면이 갱신된다. 이는 컴퓨터가 관리 가능한만큼 빠른 속도로 반복된다.

# Delta time

`getDeltaTime()` 같은 함수를 통해 지난 프레임에서 현재 프레임까지 경과시간을 계산하게된다. 일반적으로 이 값은 매우 작다. 초당 60프레임에서 이 값은 대략 `0.016666`이 된다.

detal time 값은 game의 프레임값이 어떻든 간에 부드러운 애니메이션을 얻기 위해 사용된다.

매 초당 5픽셀씩 이동하는 공이 있다고하면, 해당 하는 움직임은 아래와 같은 값으로 계산할 수 있다.

```
let dt: f64 = getDeltaTime();
ball.position.x += (5.0 * dt);
```


