# fourier_drawer

This rust progran activity approximates a user-drawn path with a complex Fourier series. It is inspired by the "But what is a Fourier series? From heat flow to circle drawings" video by 3Blue1Brown.



https://github.com/dyatelok/fourier_drawer/assets/92210438/e67150ec-bf6b-4efb-974d-ca2891ed7f0e



https://github.com/dyatelok/fourier_drawer/assets/92210438/5b2c290e-3dcb-4f59-a31d-cc5059b50255



https://github.com/dyatelok/fourier_drawer/assets/92210438/c107d5fe-4a50-4b1e-b8ec-51ed1d08dd30



Run with ```cargo run --release```

Controls:
  - c (compute) - compute FT and start an animation
  - MOUSE_LEFT_BUTTON - draw
  - KeyUp - increase number of vectors by 2
  - KeyDown - decrease number of vectors by 2
  - r (reload) - clear
