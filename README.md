Experiment for sending rendering images from `bevy` to `rerun` throuhg the python API.

Notes for running the example:
 - Start rerun: https://github.com/rerun-io/rerun/tree/main/crates/rerun#hosting-an-sdk-server
 - Build the python package: `maturin develop`
 - Start simulation: `python examples/run_bevy.py`