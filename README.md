# fourier_drawer

This rust progran activity approximates a user-drawn path with a complex Fourier series. It is inspired by the "But what is a Fourier series? From heat flow to circle drawings" video by 3Blue1Brown.


https://github.com/dyatelok/fourier_drawer/assets/92210438/2bfb613b-8ea3-4645-96f3-000ffe5320f5

https://github.com/dyatelok/fourier_drawer/assets/92210438/a8b33f57-f9fe-41f3-9081-f446687ca8d6

https://github.com/dyatelok/fourier_drawer/assets/92210438/2060975b-9dc8-4dd3-9e25-e0a7c43dc3cb


Run with ```cargo run --release```

Controls:
  - c (compute) - compute FT and start an animation
  - MOUSE_LEFT_BUTTON - draw
  - KeyUp - increase number of vectors by 2
  - KeyDown - decrease number of vectors by 2
  - r (reload) - clear
