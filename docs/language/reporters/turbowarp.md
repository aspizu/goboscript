# TurboWarp Reporters

Use these boolean reporters without parentheses:

| Reporter            | Reports true when                                            |
| ------------------- | ------------------------------------------------------------ |
| `$tw_is_compiled`   | The script is running through a supported player's compiler. |
| `$tw_is_turbowarp`  | The project is running in TurboWarp.                         |
| `$tw_is_forkphorus` | The project is running in forkphorus.                        |

The compiler check happens at runtime in the player. See the
[TurboWarp blocks guide](https://docs.turbowarp.org/blocks).
