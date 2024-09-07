# Float

- If off stage, reduce float time in half.
- Bug: Air dodge controls are reversed when / after floating
- Add float sound effects.

# Up Smash 
- buff the launch strength
- Bring Robin's circle up higher so it's visible as an effect. 

# Up Air 

# Up Tilt
- Move all effects / hitboxes ten units away from ganondorf
- Bug: Furthest hitbox can send opponents zooming across the stage, fix the 368 angle on it. 

# Down Air 
- Reduce lignting bolt length / hitboxes by 10%.
- Non-spike hitboxes should send directly up.
- Like Fox's shine, should give a tiny bit of verical velocity only once in the air. 

# Up Special 
- If teleporting from ground, make sure to add the 0.1 y offset and do telepor to float logic. 
- If teleporting from air into ground, subtract the 0.1 offset and do not do the teleport to float logic. Ganondorf should just teleport and be able to immediately act. 

# Down Special (Air)
- If special button is held: 
    - Ganondorf will continue the piledriver. 
    - If Ganondorf hits the ground, do the landing hit box. 
- If special button is not held: 
    - Ganondorf will continue the dive for 30 frames, slow down, then transition to falling state.
    - If ganondorf hits the ground during...
        - the slow down state, do normal landing. 
        - prior to slow down state, do landing hit box.
- Landing hitbox 
    - Create another hitbox that extends out from explosion along the ground only, similar 
    to down smash, but farther out. 
- Increase startup time, increase end lag of landing hitbox. 

# Down Special (Ground)
- Replace with heavy ryu shoryuken 
    - At apex of shoryuken, transition immediately to a 120 frame float if jump button 
    is held. 

# Neutral Special
- IFF articles can be cloned now, look into bringing in Sephiroth's neutral special as a dead man's volley.

# General 
- New sounds 
- Cape Physics
- Redo common prc file due to grab air stall glitch

# Down Tilt
- Programmable A input 
    - Transition to standing on attack 2
- Hitboxes on both

# Dash attack 
- Replace effects with electricity
- Multihit move launching on third hit. 
- Make dash attack startup faster. 


## Sounds safe to replace
- se_ganon_swing_s
    - Dash attack
        - Replace this with generic electrical rehit sound effects
- se_ganon_attackhardH01/2/3 
    - All three are attack up air 
        - Can use one for portal sound hitbox, and use the rest for custom.
-  se_ganon_smash_L01/2 
    - Both are down smash 
        - At least one should be replaced with the "heavy hit" sound effect like steve's down air.
- se_ganon_smash_H01
    - Up smash 
        - Will probably need more sound effects.
- se_ganon_smash_s01
    - Fsmash 
- se_ganon_special_h(1/3/4/5)
    - Up special 
    - 2 is used for general grab sfx. 
- 
