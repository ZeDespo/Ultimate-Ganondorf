# Ultimate-S Ganondorf

Ganondorf is my favorite character, and the team behind Ultimate-S made him so strong. 
This repository is largely a port from Ultimate S, using Smashline 2.

## What was ported from Ult-S 

*Note: All entries in this section can be found in Ultimate S's change log [here](https://docs.google.com/document/d/1gys8XOEnWDPZlxPB0yOVv1fCWIXJwanWTrETpU23jp4/edit#heading=h.qsh70q). Anything in italics is something I added from the source.*

### Fighter parameters

- Runspeed increased 1.34 -> 1.45
- Walkspeed increased 0.767 -> 0.875
- DJ height increased 26 -> 29
- Airspeed increased 0.83 -> 0.92
- Weight reduced 118 -> 115
- Air accel multiplier increased 0.03 -> 0.05
- Late Uair active frames altered 14-16 -> 14-18
- Dair autocancels earlier 32 > 30


### Up-Tilt

Utilt is now a 2 hit move:
    - No longer has armor
    - Damage 24/13 (early/late) -> 3/13 (hit 1 / 2)
    - Startup 60 -> 12/15
    - Angle 361/70 (early/late) -> 270/361 (grounded/aerial)


### Neutral Special (Ground)

Ganondorf teleports a set distance, releasing a burst of dark energy to blow his enemies away:
    - Startup - 23
    - FaF - 62
    - Damage - 24%
    - Set Knockback - 150/120 (ground/air)
    - Angle - 361
    - Bonus Shield Damage - -10

### Neutral Special (Air)

Ganondorf enters a floating state.
    - While he is in this state, he will move in the direction of your stick
    - This lasts for a maximum of 90 frames
    - You can do aerials while floating

*Float can only be activated once when in the air, and can only be reset once
    - You touch the ground.
    - You lose a stock.
    - You land a successful side-special or up-special.
*

### Down Special 

- Downb can now be cancelled early with a second B press. *Cancellable on frame 24.*
- Wizard Kick Hitbox size increased 3/4 -> 4/5 *on ground only*.
- Grounded downb is faster. *Starts at frame 11 instead of frame 16.*
- Grounded downb now crosses shields up.


### Warlock Punch 

*Ganondorf still has this ultra-powerful move; however, it is activated 
differently. See the `Changes from Ult-S` section for more details.*


## Changes from Ult-S 

This section highlights all added features to Ganondorf to make him more accessible and fun.

### Neutral Special (Air)

- Floating is an automatic state. One no longer needs to hold the special button to keep 
the float going. 
- If you already used your float and press the special button again, Ganondorf will 
not be locked in the float animation. 

### Side Special (Air)

- If used during a float status, you can launch a super-charged grab with the right 
inputs! If facing right, move your control stick all the way left, then all the way right, 
and quickly hit the special button.

### Down Special

- When used on the ground, instead of making the move intangible, any projectiles that 
collide with Ganondorf's kick will be erased.
- Grounded Wizard's Kick is faster than vanilla, but slower than Ultimate-S.
    - Hitboxes are active 4 frames earlier (16 -> 12) and animation ends 4 frames earlier (60 - 56). 
- Can perform a "light" Wizard's kick if special button is held down.
    - Active hitbox ends on frame 24, can act on frame 40.

### Warlock Punch

- Instead of the high taunt activating this move, the left and right taunt buttons control 
the Warlock Punch as well as Ganondorf's turn direction. 
- If floating and you press the special button, Ganondorf will use the Warlock Punch in 
the direction he is currently facing.

