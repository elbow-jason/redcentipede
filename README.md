![alt text](https://raw.githubusercontent.com/jlgoldb2/redcentipede/master/img/red_centipede_logo.png "The Red Centipede")

Red Centipede
============

### A Genetic Algorithm Project in Rust
The concepts of this project borrow heavily from [andrewrk's planet-evo project](https://github.com/andrewrk/planet-evo "andrewrk's planet-evo"). 

### General Game Theory of Red Centipede


There exists:

+ The World 
+ Some Agents
+ Some Resources
+ Units of Time


__The World__ - the World is a  2-dimensional grid with X,Y coordinates.  Agents and Resources that exist have both an X and a Y that refers to that entity's coordinates in the World. The World is a plane as viewed from above, and the World has no physics.

__Agents__ -  An Agent is an action-capable entity whose actions/reactions are determined by that Agent's genetic code. An Agent has an X and Y coordinate that represents that Agent's placement in the World. Additionally, an Agent has its own reserves of Resources that are consumed when that Agent acts.

__Resources__ - A Resource is an action-incapable entity which is spawned into the World and (hopefully) collected by Agents. A Resource has X and Y coordinates and an numerical value which represents that Resources ummm value. When an Agent acquires a Resource that Resource's value is added to the Agent's reserves of Resource and then that Resource is destroyed.

__Time__ - Time is a discrete, turn-based cycle. During each cycle of Time, Agents act or not and Resources are spawned or not. At the beginning of each cycle of Time, a new World is spawned, then each Agent has a turn to act based on that Agent's perception of the previous World. Upon completion of all Agents' actions for the current cycle outcomes are assessed (e.g. If during the current cycle of Time an Agent's action was to acquire a Resource, that Resouce's value is transferred to the Agent's reserves and the Resource is destroyed.). Once all outcomes are assessed, the World's coordinate system is updated with each Agent's X and Y. Then, if necessary, new Resources are created in the World. (Finally, the World is drawn to the screen?)
