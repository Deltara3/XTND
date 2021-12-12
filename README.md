# XTND
SPWN extension manager.

## Important Disclaimer
**As this will allow any module to run as builtins, with little to no limitations, use of XTND may be unsafe without proper precautions in place. By using this program, you agree that any damage caused to your OS install and/or your computer, is not at the fault of the XTND developers. If you don't know what you are doing, don't use this program. If you continue, a good measure to put into place is as follows. ONLY USE MODULES FROM PEOPLE YOU TRUST. Be careful, and enjoy XTND.**

## Running
- Linux: `LD_PRELOAD="./libxtnd_base.so" [your spwn here]`
- Windows: Currently in the works.
- macOS: `DYLD_INSERT_LIBRARIES=$PWD/libxtnd_base.dylib DYLD_FORCE_FLAT_NAMESPACE=1 [your spwn here]`

Or use the [CLI](/cli).
