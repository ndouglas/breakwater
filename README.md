# Breakwater
Breakwater is a library for environmental procedural content generation and related tools.

The general idea is that I want to be able to create maps, rooms, weather phenomena, geological features, astronomical features, constellations, that sort of thing.  Natural and artificial locations.  Tides!  Moons!  Crap like that.

I don't know, I'm just making this up as I go.

## Star System

Generation will begin with a star system.  In nature, star systems can have one, two, three, four, or even more bodies -- but I don't see much reason to go beyond four.  Systems of three or more stars can be _hierarchical_ - meaning that they _effectively_ can be described as a two-body system - or not, but the latter are so complicated and unstable that it's difficult to imagine intelligent life arising there.

So our process is as follows:

1. Determine an _arity_ of the star system, in which solitary stars are most common, followed by binary, trinary, and so forth.
2. Determine an _arrangement_ of the star system, the specific details.  

## Planetary System

Generation of the Planetary System begins with a Host Star.  This is what we'll term the star at the center of this conjectured extrasolar system.  It's possible for the host star to be part of a binary system; in a non-circumbinary (S-type) planetary orbit, the planet will circle only one of the stars, whereas a circumbinary (P-type) planetary orbit features the planet circling both.  This could lead to very interesting worlds!

## Hornvale Project
- [Hornvale](https://github.com/ndouglas/hornvale/): Frontend and connective logic
  - [Goldengrove](https://github.com/ndouglas/goldengrove/): Narrative/mythopoetic procedural content generation and tools
  - [Lasthearth](https://github.com/ndouglas/lasthearth/): Embedded programming language and domain-specific library
  - [Breakwater](https://github.com/ndouglas/breakwater/): Environmental procedural content generation and tools
  - [Brownhollow](https://github.com/ndouglas/brownhollow/): Artificial life/social/factional/economic procedural content generation and tools
  - [Honeyholt](https://github.com/ndouglas/honeyholt/): Tool for humanizing numbers, concepts, etc.
  - [Lemonwood](https://github.com/ndouglas/lemonwood/): Flexible and powerful test harness for roguelike projects.
