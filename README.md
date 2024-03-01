# Ultimate Ganondorf

Ganondorf is my favorite character, and the team behind Ultimate-S made him so strong. 
This repository uses the Ultimate-S version of Ganondorf as a base,t hen cranks it 
up to 11, pulling inspiration from other mods such as Legacy XP and Project M.

## Master Changelog 

Anything that's *italicized* is a copy and paste port from Ultimate-S. You can find 
their changelog for Ganondorf [here](https://docs.google.com/document/d/1gys8XOEnWDPZlxPB0yOVv1fCWIXJwanWTrETpU23jp4/edit#heading=h.qsh70q)

### Fighter parameters

- *Runspeed increased 1.34 -> 1.45*
- *Walkspeed increased 0.767 -> 0.875*
- *DJ height increased 26 -> 29*
- *Airspeed increased 0.83 -> 0.92*
- *Weight reduced 118 -> 115*
- *Air accel multiplier increased 0.03 -> 0.05*
- *Dair autocancels earlier 32 > 30*
- *Up air LL increased 11 > 13*
- *Bair LL increased 11 > 12*
- *Dair LL increased 16 > 20*

---

### Neutral Special (Ground)

Ganondorf is granted an omni-directional teleport, whose distance / position is based 
on the direction / axis of the left analog stick. If Ganondorf teleports into an opponent, 
the portal acts as a multi-hit that launches the opponent upwards. If the portal does not hit 
an opponent, Ganondorf will leave the portal after 40 frames. Despite his invisibility, 
Ganondorf can be punished for missing a telport hitbox. Opponents beware, as soon as 
Ganondorf appears from the portal, he can act immediately with a punish of his own.

Range for the teleport can be anywhere between 5 and 60 units in direction. If you 
slam the left analog stick you'll go farther. Barely move the analog stick in a direction 
to teleport shorter distances.

- Startup: 16 frames
- Multihit damage: 1.1 units 
- Final hit: 
    - Damage: 7.0
    - Knockback growth: 108
    - Base knockback: 23 
    - Angle: 90
- Total Frames: 67


#### If Teleport misses opponent

- Cancellable via jump (if telport misses opponent) on frame 57. Can be bufferred.
- Grounded moves cannot be bufferred if teleport postiion is on ground. Instead, 
wait for Ganondorf to reappear before inputting a move.

#### If Teleport hits opponent

- Any follow up action can be bufferred, including another teleport.

---

### Neutral Special (Air)

*Ganondorf enters a float state, allowing him non-linear movement in the air depending on 
the direction of the left analog stick.* 

Float can only be activated once when in the air, and can only be reset once...

- You touch the ground.
- You lose a stock.
- You land a successful side-special or up-special whilst in the air.

Ganondorf will exit the float state if...

- It has been active for 90 frames.
- It is cancelled with a jump, air dodge, or special move.
- Ganondorf gets launched by a significant attack.

---

### Warlock Punch 

Ganondorf still has this ultra-powerful move; however, the move is tied to the left and 
right taunt buttons. By default, these taunt buttons will activate the "turn" version 
of this move, so it will hit HARD.

---

### Side Special (Air)

While in float, and using the correct inputs, Ganondorf can launch himself in a specific
direction at blinding speeds. If facing right, press the following inputs: 
```
->, <-, (-> + B)  // Where B is the special button.
```
This move can also be angled vertically, slightly. 

---

### Down Special 

- On frame 16 until frame 39, a reflector hitbox will spawn on Ganondorf's foot, eating
MOST projectiles that come his way. Fast projectiles such as Byleth's fully charged 
Failnaught arrow will be reflected back at the opponent. 
- Tapping the special button will yield a shorter Wizard's Kick animation ending on frame 24.
    - Reflector will only last until frame 24 as well.
- Holding the special button past frame 10 will use the old Wizard's Kick animation.

#### Grounded properties

- *Hitbox active on frame 16.*
- Crosses up shields if used close enough.

---


### Up-Tilt

*Utilt is now a 2 hit move:*
- *No longer has armor*
- *Damage 24/13 (early/late) -> 3/13 (hit 1 / 2)*
- *Startup 60 -> 12/15*
- *Angle 361/70 (early/late) -> 270/361 (grounded/aerial)*

---

### Up-Smash

- *Startup reduced 20 -> 11*
- Damage reduced from 24/21 -> 19/16

---

### Forward-Air

- *Autocancels earlier (Frame 45 -> Frame 36)*
- *Base knockback increased from 20 -> 40 units.*
- *Knockback growth reduced from 93 -> 85 units.*

---

### Up-Air 

- *Late Uair active frames altered 14-16 -> 14-18*


