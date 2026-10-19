# Nexa Camera adapter

See the [project README](../../README.md) for installation, checks, and known limits.

The v0.2.0 adapter receives a bounded result from its own camera worker,
validates the camera-only VMD, and dispatches a standard file-drop notification
on the originating MMD UI thread. Cancellation, failed conversion, stale results,
non-camera operation mode and modal windows prevent automatic delivery.
Runtime loading and preservation of existing keys remain unverified in MMD;
delivery alone is not a confirmed import.

## 謝辞

MikuMikuDance（MMD）の作者である樋口優氏をはじめ、MMDPlugin、MMEffect、Blender、Bullet、Saba、GLMの開発者・貢献者、モデル・モーション・エフェクト・資料の制作者、MMDコミュニティの皆様に感謝します。
