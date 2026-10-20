# Nexa Camera adapter

See the [project README](../../README.md) for installation, checks, and known limits.

The v0.2.0 adapter receives a bounded result from its own camera worker,
validates the camera-only VMD, and dispatches a standard file-drop notification
on the originating MMD UI thread. Cancellation, failed conversion, stale results,
non-camera operation mode and modal windows prevent automatic delivery.
Runtime loading and preservation of existing keys remain unverified in MMD;
delivery alone is not a confirmed import.

## Acknowledgements

We thank MikuMikuDance (MMD) creator 樋口優, the developers and contributors of MMDPlugin, MMEffect, Blender, Bullet, Saba and GLM, the creators of models, motions, effects and reference materials, and the MMD community.
