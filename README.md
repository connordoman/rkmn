# RKMN

RKMN take on the following format in a 64-bit architecture:

```
|   1   |   2   |   3   |   4   |   5   |   6   |   7   |   8   |
|-------|-------|-------|-------|-------|-------|-------|-------|
|          Personality          |             OT ID             |
|---------------------------------------------------------------|
|                           Nickname [0:7]                      |
|---------------------------------------------------------------|
|                           Nickname [8:15]                     |
|---------------------------------------------------------------|
|                           Nickname [16:23]                    |
|---------------------------------------------------------------|
|                           Nickname [24:31]                    |
|---------------------------------------------------------------|
|                           Nickname [32:39]                    |
|---------------------------------------------------------------|
|                           OT Name [0:7]                       |
|---------------------------------------------------------------|
|                           OT Name [8:15]                      |
|---------------------------------------------------------------|
|                           OT Name [16:23]                     |
|---------------------------------------------------------------|
|                           OT Name [24:31]                     |
|---------------------------------------------------------------|
|                           OT Name [32:39]                     |
|---------------------------------------------------------------|
| Lang. | Misc. | Mark. |   Checksum    | Level | Mail  |  ???  |
|---------------------------------------------------------------|
|                           Data [0:7]                          |
|---------------------------------------------------------------|
|                           Data [8:15]                         |
|---------------------------------------------------------------|
|                           Data [16:23]                        |
|---------------------------------------------------------------|
|                           Data [24:31]                        |
|---------------------------------------------------------------|
|                           Data [32:39]                        |
|---------------------------------------------------------------|
|                           Data [40:47]                        |
|---------------------------------------------------------------|
|   Total HP    |  Current HP   |    Attack     |   Defense     |
|---------------------------------------------------------------|
|    Speed      |    Sp. Atk    |    Sp. Def    |   [Unused]    |
|---------------------------------------------------------------|
```

Where `Nickname` and `OT Name` are 10 character Unicode strings occupying up to 40 bytes each.
