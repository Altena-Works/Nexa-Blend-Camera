# Nexa Blend Camera adapter

See the [project README](../../README.md) for installation, checks, and known limits.

The v0.2.0 adapter receives a bounded result from its own camera worker,
validates the camera-only VMD, and dispatches a standard file-drop notification
on the originating MMD UI thread. Cancellation, failed conversion, stale results,
non-camera operation mode and modal windows prevent automatic delivery.
Automatic loading and preservation of existing model poses, motion, lighting and
out-of-range camera keys have been observed in MMD at the documented sample
frames. Delivery alone is not a confirmed import; see the project verification
record for the tested data and limits.

The v1.0.0 adapter follows MMD's Japanese/English setting and passes
that language to the worker. English actions are **Import .blend...** and
**Convert**. Both languages have been exercised in MMD. Cancellation after successful output
publication retains the saved result instead of reporting a discarded conversion.

## Acknowledgements

We thank MikuMikuDance (MMD) creator 樋口優, the developers and contributors of MMDPlugin, MMEffect, Blender, Bullet, Saba and GLM, the creators of models, motions, effects and reference materials, and the MMD community.
