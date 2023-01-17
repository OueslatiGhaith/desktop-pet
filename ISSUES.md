## features
    - [x] animation states that move across the screen
    - [ ] screen edge detection (to prevent walking off-screen) (seems to be impossible currently, as there is no possible way to get information about the current monitor or it's current size/resolution, the best that can be done is to randomly change direction of the sprite)
    - [ ] window always on top (will be probably be added in bevy 0.9.2, for now, nothing can be done)
    - [ ] add icon, currently, this is done through a hacky way, so I decided to omit it for now until the next version of bevy that supports it

## bugs
    - [x] intially the windows does take the correct scale and position, this is solved after the first state transition
