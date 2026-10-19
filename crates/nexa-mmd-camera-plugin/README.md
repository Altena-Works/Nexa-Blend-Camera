# Nexa Camera adapter

See the [project README](../../README.md) for installation, checks, and known limits.

The development adapter receives a bounded result from its own camera worker,
validates the camera-only VMD, and dispatches a standard file-drop notification
on the originating MMD UI thread. Cancellation, failed conversion, stale results,
non-camera operation mode and modal windows prevent automatic delivery.
Runtime loading and preservation of existing keys still require acceptance;
delivery alone is not a confirmed import. Published v0.1.0 remains unchanged.

## 謝辞

MikuMikuDance（MMD）を制作・公開された樋口優氏に、深く感謝します。また、MMDPlugin、MMEffect、Blender、Bullet、Saba、GLMをはじめとする関連ソフトウェア・ライブラリの開発者と貢献者、モデル・モーション・エフェクト・資料を制作・公開してきた皆様、そしてMMDコミュニティを支えてきたすべての皆様に感謝します。Nexaとその関連成果物は、皆様が築いてきた技術と創作文化に支えられています。
