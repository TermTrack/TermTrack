# TermTrack
A terminal-rendered 3D platforming/maze game with focus on speedrunning and custom level creation

## Requirements
You will need a terminal to play this game. But all teminals are not created the same. For the moment we recommend that you use [windows-terminal](https://www.microsoft.com/store/productId/9N0DX20HK701?ocid=pdpshare) availible in the microsoft store (further testing will be done in the future)

## Install and Run
### Windows
Install the zip-folder from the releases section or using the link: [https://tagedan.github.io/TermTrack/;TERMTRACK;%20.zip](https://termtrack.github.io/TermTrack/;TERMTRACK;%20.zip)
Extract it into a folder of your choice and then run it using this terminal command in the "TermTrack" folder:
```bash
example/TermTrack> termtrack level_pack_0
```
where ```level_pack_0``` can be substituted for the name of the folder containing the levels you want to play.
### Linux/Unix
Coming up
### From source
Unfortunaly, due to the need of a secret salt to validate the leaderboard you cannot build this project from source and expect it to work with the leaderboard. We are working on a seperate branch where the leaderboard will instead be local and therefore can be built from source.
## Level Layout/Creation
A level is represented by a textfile with the format level_name.txt (or any other file extention, everything up until the last '.' will be the level name)
To build a level you write characters that will represent the grid of the actual level. There are 8 grid-types at the moment, these are:

- 'S' (start grid)
- 'E' (end grid)
- 'X' (wall)
- 'x' (half-wall / stair)
- 'v' (spike)
- '.' (floor)
- ' ' (hole)
- 'e' (enemy / angry-pixel spawn)

There is also the floor seperator represented by a new row containing only *sep* after wich the next floor can be built.


Example_level.txt:
``` 
XXX
XSX
XvX
X.X
XxX
XXX
sep
XXXXXXX
X....EX
X.XXXXX
X.X
X X
XXX
```
This level will have two floors with the lower floor containing the start and the stair to the second floor as well as a spike between them and the upper floor containing the end.
To then play the lavel you need to put it into a folder next to termtrack.exe and then run:
```bash
TermTrack> termtrack <level_folder_name>
```
replacing `<level_folder_name>` with the name of your folder.



