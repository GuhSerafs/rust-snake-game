# rust-snake-game
Neste repositório está a implementação do clássico jogo jogo da cobrinha escrito em Rust. A implementação foi feita a partir do tutorial do Tensor Programming, que pode ser acessado aqui. 

## Troubleshoot Info

A few bugs that I've found in the development were solved looking at the following links: 

* Error: CreationErrors([NoAvailablePixelFormat, OsError("Couldn't find any available vsync extension")]): 
Solution: Include .vsync(true) in the WindowSettings constructor. 
Source: https://githubmemory.com/repo/glium/glium/issues/1870

* Error: closure is expected to take 3 arguments, but it takes 2 arguments - expected closure that takes 3 argumentsrustc(E0593)
Solution: Change the .draw_2d arguments according to the docs.
Source: https://docs.piston.rs/piston_window/piston_window/