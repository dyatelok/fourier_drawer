# fourier_drawer

This rust progran activity approximates a user-drawn path with a complex Fourier series. It is inspired by the "But what is a Fourier series? From heat flow to circle drawings" video by 3Blue1Brown.

Run with ```cargo run --release```

Controls:
  - c (compute) - compute FT and start an animation
  - MOUSE_LEFT_BUTTON - draw
  - KeyUp - increase number of vectors by 2
  - KeyDown - decrease number of vectors by 2
  - r (reload) - clear
