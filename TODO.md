# Float
- Implement a90float animation? Might be tough considering the variable float times. 
- If jump button is held during teleport, that should translate into a jump.
    - Bug: Float will activate for a frame if this were the case. 

# Up Smash 

# Up Air 

# Up Tilt
- Allow cancel if and only if the attack connects at the current end frame. 

# Down Air 
- Like Fox's shine, should give a tiny bit of verical velocity only once in the air and not in float mode. 

# Up Special 
- Bug: If attempting to chain teleports from grounded position on successful first teleport 
hit, Ganon freezes up. 
    - Maybe it's time to Remove the float mechanic from the teleport and have the up-special 
    spawn its own hitboxes like a normal move...

# Down Special (Air)
- If special button is held: 
    - Ganondorf will continue the piledriver. 
    - If Ganondorf hits the ground, do the landing hit box. 
- If special button is not held: 
    - Ganondorf will continue the dive for 10 frames, slow down, then transition to falling state.
    - If ganondorf hits the ground during...
        - the slow down state, do normal landing. 
        - prior to slow down state, do landing hit box.
    - If Ganondorf hits an opponent during ...
        - the slowdown state, a weak hitbox will send them somewhat flying 
        - prior to slowdown state, normal hitbox. 
- Landing hitbox 
    - Create another hitbox that extends out from explosion along the ground only, similar 
    to down smash, but farther out. 
    - If Ganondorf's dive lasts longer than 19 frames, 1.5x size of landing hitboxes / effects...
        - ... but landing lag is also increased by 1.5 times.
        - ... maybe take damage too?
- Increase startup time, increase end lag of landing hitbox. 
- Hand hitbox should act as a pseudo grab, ensuring opponent will be caught in explosion??? IDK if this is a good idea.

# Down Special (Ground)
- Replace with heavy ryu shoryuken 
    - At apex of shoryuken, transition immediately to a 120 frame float if jump button 
    is held. 

# Neutral Special
- IFF articles can be cloned now, look into bringing in Sephiroth's neutral special as a dead man's volley.

# Side special 
- Make change to swingblend.prc to have cape use animation instead of general physics.

# Down Tilt 

# Dash attack 
- Replace effects with electricity
- Multihit move launching on third hit. 
- Make dash attack startup faster. 

# General 
- Sounds, expressions, and effects 
- Redo common prc file due to grab air stall glitch

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
