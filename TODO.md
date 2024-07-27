# Float

- If off stage, reduce float time in half.
- BUG: If both double jumps used, and on ledge, cannot do ledge jump if float is available.
- BUG: jump button should be held from frames 1 - N for float to activate.
- Add float sound effects.

# Down Smash
- Extend z hitbox on both sides to 24.0 units (absolute value)
- Create pickup hitbox if down smash is done next to the opponent (frame 13?)
- Create center mass hitbox that will start once Ganondorf descends that will launch bigly far. 
- Do same for opposite end.
- Once that's done make ganondorf's jump more dramatic by playing around with motion rate 
    - Quick jump, stall in air a bit, then quick descent

# Forward Smash 
- Add small pickup hitbox in front of ganondorf (using top)
- Adjust damage.

# Up Air 
- Make hitbox in the hand itself to pick up to portal

# Up Tilt
- Fix furthest portal hitbox to scoop

# Down Air 
- Make spike hitbox a bit smaller
- All other hitboxes should send directly up.

# Down Special 
- Fix bug where if you die doing down special, you can do it again. 
- Depending on the number of frames spent falling, damage / explosion get substantially bigger. 
- Grant ability to cancel after 60 frames of free fall. 

# Side Special 
- Make it neutral special OR try to create a new motion kind so we can support down special and this. 

# Neutral Special
- IFF articles can be cloned now, look into bringing in Sephiroth's neutral special as a dead man's volley.
