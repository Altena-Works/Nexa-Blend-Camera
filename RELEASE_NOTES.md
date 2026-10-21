v1.0.0は、BlenderのカメラをMMDへ取り込むWindows x64向けプラグイン「Nexa Blend Camera」のリリースです。

## ダウンロード（ビルド不要）

| ファイル | 内容 |
|---|---|
| `Nexa-Blend-Camera-1.0.0-x64.zip` | Windows x64 plugin and worker; unsigned |
| `SHA256SUMS.txt` | SHA-256 checksum for the ZIP |

MMDを終了し、ZIP内の`NexaCamera`をMMDの`Plugin`へ配置します。更新時は旧版を別の場所に保存し、DLLとexeを両方更新してください。MMD 9.32 x64・MMDPlugin v0.41 x64・Blenderは別途必要です。Blender 4.5.4で検証しています。導入手順は同梱`INSTALL.md`を参照してください。GitHubの`Source code`には実行ファイルがありません。

ZIP SHA-256: `28d480ad7f9fc564e83aa3c1ee58c947bfed86d66322854b2c8ce1aedf94036e`

## 主な変更

- インストール済みBlenderの自動検出、候補の選択と手動指定。
- MMDの日本語・英語設定に合わせたメニュー・設定画面・結果とエラー表示。
- 処理中表示、取消操作、Blender子プロセスの600秒制限。
- 平行投影の倍率と、日本語のカメラ操作モード判定を修正。
- 保存完了直後の取消でも、確定済みの出力と成功結果を保持。

## 確認範囲と制限

日英GUIから121カメラキーの自動読み込みを実MMDで確認しました。指定フレームでは既存モデル姿勢・モーション・照明と、範囲外カメラキーを保持しました。自作データの投影比較は12点すべて0.5px以内、最大誤差は約0.077pxでした。実変換中の取消ではVMDが生成されないことを確認しています。

ZIPのx64形式、全ファイルの一致、チェックサム、展開先workerのヘルプ起動を確認済みです。最終の監視処理整理後は実子プロセスの取消・短縮timeout試験とビルドで確認しており、600秒を経過させる実Blender試験は行っていません。詳しくは[検証状況](VERIFICATION.md)を参照してください。

MMDをカメラ・照明・アクセサリ操作にして使用し、作業は先に保存してください。標準読み込みは現在フレームを開始位置とするため、先頭から配置する場合は0へ戻します。自動で読み込まれない場合は保存済みVMDを標準モーション読み込みで開けます。

カメラのみを30fpsで出力します。モデル・モデルアニメーション・ライトの取り込み、ライブ同期には対応していません。レンズシフト・非剛体カメラ・パノラマ等は未対応で、非整数視野角は明示許可した場合だけ近似します。任意のblendとの互換性を保証するものではありません。未署名で、最低対応Windowsバージョンは未確定です。

## English

v1.0.0 of Nexa Blend Camera is available as a Windows x64 release. Download the Windows x64 ZIP listed above; it includes the plugin and worker, so no build is required. MMD 9.32 x64, MMDPlugin v0.41 x64 and Blender are external requirements; Blender 4.5.4 was tested. Close MMD, preserve the previous plugin folder and install `NexaCamera` inside `Plugin`, replacing both the DLL and exe.

This version adds Blender detection, Japanese/English interfaces and cancellable conversion, and fixes orthographic scaling and Japanese camera-mode detection. Automatic loading of 121 camera keys and preservation at selected frames were observed in MMD. All twelve self-authored projection markers were within 0.5 pixels (maximum 0.077 pixels).

The binaries are unsigned. The minimum Windows version is not established. Cameras are exported at 30 fps; models, model animation, lights and live synchronization are not imported. Lens shift, non-rigid transforms and panoramas are unsupported; non-integer FOV requires explicit approximation consent. Native MMD evidence predates the final process-monitor extraction; real subprocess tests verify the extracted cancellation/timeout behavior. A full 600-second native timeout run was not performed. See [verification](VERIFICATION.md) and the bundled README for details.

## ライセンス

自作部分はApache-2.0です。第三者通知を同梱しています。

## 謝辞

MikuMikuDance（MMD）の作者である樋口優氏をはじめ、MMDPlugin、MMEffect、Blender、Bullet、Saba、GLMの開発者・貢献者、モデル・モーション・エフェクト・資料の制作者、MMDコミュニティの皆様に感謝します。
