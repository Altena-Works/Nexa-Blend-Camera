//! Shared, dependency-free catalog for plugin and worker owned UI.
use std::ffi::{OsStr, OsString};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Language { #[default] Ja, En }
impl Language {
    pub fn code(self) -> &'static str { match self { Self::Ja => "ja", Self::En => "en" } }
    pub fn parse(value: &OsStr) -> Option<Self> {
        if value == "ja" { Some(Self::Ja) } else if value == "en" { Some(Self::En) } else { None }
    }
    pub fn format(self, key: Text, args: &[(&str, &str)]) -> String {
        let mut result = String::new();
        let mut rest = self.text(key);
        while let Some(start) = rest.find('{') {
            result.push_str(&rest[..start]);
            let Some(end) = rest[start..].find('}').map(|n| start + n) else { break; };
            let name = &rest[start + 1..end];
            if let Some((_, value)) = args.iter().find(|(key, _)| *key == name) { result.push_str(value); }
            else { result.push_str(&rest[start..=end]); }
            rest = &rest[end + 1..];
        }
        result.push_str(rest);
        result
    }
}
macro_rules! catalog {
    ($($key:ident => ($ja:literal, $en:literal)),+ $(,)?) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub enum Text { $($key),+ }
        impl Language {
            pub fn text(self, key: Text) -> &'static str {
                match (self, key) { $((Self::Ja, Text::$key) => $ja, (Self::En, Text::$key) => $en),+ }
            }
        }
        #[cfg(test)] const ALL: &[Text] = &[$(Text::$key),+];
    };
}
catalog! {
    Title => ("Nexa Blend Camera", "Nexa Blend Camera"),
    Import => ("blendを取り込み", "Import .blend..."),
    Blender => ("Blender実行ファイル (.exe)", "Blender executable (.exe)"),
    Input => ("入力 (.blend)", "Input (.blend)"),
    Output => ("カメラ出力 (.vmd)", "Camera output (.vmd)"),
    Browse => ("参照...", "Browse..."),
    Scene => ("シーン名（空欄で現在のシーン）", "Scene (blank: current scene)"),
    Camera => ("カメラ名（空欄で切替も評価）", "Camera (blank: evaluate camera cuts)"),
    Start => ("開始（空欄でシーン設定）", "Start (blank: scene setting)"),
    End => ("終了（空欄でシーン設定）", "End (blank: scene setting)"),
    Units => ("MMD単位 / メートル", "MMD units / metre"),
    Width => ("出力幅 (px)", "Output width (px)"),
    Height => ("出力高さ (px)", "Output height (px)"),
    MatchDimensions => ("MMD側も同じ出力寸法に設定してください", "Use the same output dimensions in MMD."),
    Approximation => ("整数FOVへの近似を許可（誤差を結果に表示）", "Allow integer-FOV approximation (report error in results)"),
    Overwrite => ("既存の出力ファイルを上書き", "Overwrite the existing output file"),
    Convert => ("変換", "Convert"),
    Cancel => ("キャンセル", "Cancel"),
    Working => ("変換中（上限600秒）...", "Converting (600-second limit)..."),
    Cancelling => ("取消処理中...", "Cancelling..."),
    NotDetected => ("未検出・手動指定", "Not detected; select manually"),
    SelectFile => ("ファイルを選択してください", "Select a file."),
    SelectBlender => ("Blenderの実行ファイルが見つかりません。同じ画面で再指定してください。", "Blender executable not found. Select it again in this window."),
    SelectBlend => ("入力には.blendファイルを選択してください", "Select an existing .blend input file."),
    SelectOutput => ("既存フォルダー内の.vmd出力先を指定してください", "Choose a .vmd output in an existing folder."),
    Unicode => ("文字列が有効なUnicodeではありません。", "The text is not valid Unicode."),
    Number => ("{name}: 数値を入力してください", "{name}: enter a number."),
    FiniteNumber => ("{name}: 有限の数値を入力してください", "{name}: enter a finite number."),
    Dimensions => ("出力寸法は1〜16384の整数です", "Output dimensions must be integers from 1 to 16384."),
    StartEnd => ("開始は終了以下にしてください", "Start must not exceed end."),
    PositiveUnits => ("単位倍率は正数です", "Unit scale must be positive."),
    FilenameLong => ("ファイル名が長すぎます", "The filename is too long."),
    BlenderFilter => ("Blender実行ファイル\0*.exe\0すべてのファイル\0*.*\0", "Blender executable\0*.exe\0All files\0*.*\0"),
    BlendFilter => ("Blenderファイル\0*.blend\0", "Blender file\0*.blend\0"),
    VmdFilter => ("カメラVMD\0*.vmd\0", "VMD camera\0*.vmd\0"),
    DialogFailed => ("ファイル選択に失敗しました（0x{code}）。", "File dialog failed (0x{code})."),
    DialogPath => ("ファイル選択が終端のないパスを返しました。", "File dialog returned an unterminated path."),
    GuiFailed => ("画面操作に失敗しました。\n{detail}", "The window operation failed.\n{detail}"),
    GuiState => ("画面の状態を初期化できませんでした。", "Could not initialize the window state."),
    Io => ("ファイルまたはプロセス操作に失敗しました。\n{detail}", "A file or process operation failed.\n{detail}"),
    RegularOutput => ("出力は通常のファイルにしてください。シンボリックリンクは使用できません。", "Output must be a regular file, not a symlink."),
    OutputExists => ("出力が存在します。別名を指定するか、上書きを明示してください。", "Output exists; choose a new name or explicitly enable overwrite."),
    ExistingInputs => ("Blenderと入力.blendには既存のファイルを指定してください。", "Blender and input .blend must be existing files."),
    BlendExtension => ("入力の拡張子は.blendです。", "Input must have a .blend extension."),
    VmdExtension => ("出力の拡張子は.vmdです。", "Output must have a .vmd extension."),
    UnitsDimensions => ("正の単位倍率と1〜16384の出力寸法が必要です。", "Positive units and output dimensions in 1..16384 are required."),
    AnimatedUnits => ("アニメーションする単位倍率には対応していません。", "Animated unit scale is unsupported."),
    Diagnostic => ("診断フォルダー: {path}", "Diagnostic directory: {path}"),
    FrameError => ("出力フレーム {frame}、カメラ {camera}: {detail}", "Output frame {frame}, camera {camera}: {detail}"),
    WriteFailed => ("書き込みに失敗しました。一時出力を {path} に保持しました。\n{detail}", "Write failed; temporary output retained at {path}.\n{detail}"),
    PublishFailed => ("出力の確定に失敗しました。\n{detail}", "Could not publish output.\n{detail}"),
    Cancelled => ("変換を取り消しました。", "Conversion cancelled."),
    Timeout => ("変換が600秒の制限時間を超えました。", "Conversion exceeded the 600-second time limit."),
    ThreadFailed => ("変換処理を開始できませんでした。\n{detail}", "Could not start conversion.\n{detail}"),
    Report => ("カメラVMD: {output}\n30fpsで{keys}キー、カメラ切替{cuts}回\nMMD出力: {width} x {height} ピクセル\nMMD単位 / メートル: {units}\n整数FOV近似: {approximated}キー、最大画面端誤差 [{error_x}, {error_y}] ピクセル\n終端の延長: {extension} 秒\nVMDはクリップ面を保存しません。カメラ切替はMMDの整数フレームで評価し、サブフレームの不連続は保証しません。\nMMDをカメラ・照明操作に切り替え、標準モーション読み込みでこのVMDを開いてください。\n診断フォルダー: {diagnostic}", "Camera VMD: {output}\n{keys} keys at 30 fps; {cuts} active-camera cuts\nMMD output: {width} x {height} pixels\nMMD units / metre: {units}\nInteger-FOV approximation: {approximated} keys, maximum edge error [{error_x}, {error_y}] pixels\nEndpoint extension: {extension} seconds\nClipping planes are not stored in VMD. Camera cuts are sampled at integer MMD frames; subframe discontinuity is not guaranteed.\nSelect camera/light mode in MMD, then load this VMD using the standard motion loader.\nDiagnostic directory: {diagnostic}"),
    RangeTooLarge => ("カメラ範囲が大きすぎます。", "Camera range is too large."),
    RangeMagic => ("カメラ範囲の識別子が不正です。", "Invalid camera range magic."),
    SampleCount => ("サンプル数が不正です。", "Invalid sample count."),
    RangeTruncated => ("カメラ範囲が途中で切れています。", "Truncated camera range."),
    SampleSize => ("サンプルサイズが不正です。", "Invalid sample size."),
    SceneChanged => ("範囲内でシーン、ビューレイヤー、FPSが変化しました。", "Scene, view layer or FPS changed during the range."),
    RangeTrailing => ("カメラ範囲の末尾に余分なデータがあります。", "Trailing camera range bytes."),
    InputFps => ("入力FPSが不正です。", "Invalid input FPS."),
    FrameLimits => ("範囲端点がBlenderのフレーム制限を超えています。", "Range endpoint outside Blender frame limits."),
    RangeExists => ("カメラ範囲の出力が既に存在します。", "Range output already exists."),
    BlenderFailed => ("Blenderの範囲評価に失敗しました（終了コード {code}）。外部診断を確認してください。", "Blender range evaluation failed (exit code {code}). Check the external diagnostics."),
    Packet => ("カメラデータが不正です。\n{detail}", "Invalid camera data.\n{detail}"),
    PacketLarge => ("カメラデータのサイズ{size}が上限{limit}バイトを超えています。", "Camera packet size {size} exceeds {limit} bytes."),
    PacketMagic => ("カメラデータの識別子が不正です。", "Invalid camera packet magic."),
    PacketTruncated => ("カメラデータの{offset}バイト目で不足（必要{needed}、残り{remaining}）。", "Truncated camera packet at byte {offset}: need {needed}, have {remaining}."),
    PacketUtf8 => ("カメラデータの{offset}バイト目が不正なUTF-8です。", "Invalid UTF-8 payload at byte {offset}."),
    PacketEnum => ("カメラデータの{offset}バイト目に不正な列挙値{value}があります。", "Invalid camera enum {value} at byte {offset}."),
    PacketValue => ("カメラデータの値が不正です: {field}", "Invalid camera value: {field}"),
    PacketTrailing => ("カメラデータの{offset}バイト目以降に余分な{remaining}バイトがあります。", "Camera packet has {remaining} trailing bytes at byte {offset}."),
    SampleError => ("カメラサンプル {sample}: {detail}", "Camera sample {sample}: {detail}"),
    VmdKeys => ("VMDカメラキーが不正です。", "Invalid VMD camera keys."),
    F32 => ("カメラの値をVMDのf32形式で表現できません。", "Camera value cannot be represented as VMD f32."),
    ConvertUnits => ("正の有限な単位倍率と0でない出力寸法が必要です。", "Positive finite units and nonzero viewport are required."),
    PreviousRotation => ("前の回転値が不正です。", "Invalid previous rotation."),
    MatrixFinite => ("カメラ行列に有限でない値があります。", "Non-finite camera matrix."),
    MatrixAffine => ("カメラ行列がアフィン変換ではありません。", "Non-affine camera matrix."),
    MatrixRigid => ("拡縮、せん断、反転を含むカメラ変換には対応していません。", "Scaled, sheared or reflected camera transforms are unsupported."),
    LensShift => ("レンズシフト・軸外投影をVMDで表現できません。", "Lens shift/off-axis projection cannot be represented by VMD."),
    OffAxis => ("この軸外投影には対応していません。", "Unsupported off-axis camera projection."),
    Projection => ("このカメラ投影行列には対応していません。", "Unsupported camera projection matrix."),
    Aspect => ("出力の幅と高さの比が、評価したBlenderカメラと一致しません。", "Output width/height does not match the evaluated Blender camera aspect."),
    FovRange => ("MMDの垂直視野角は1〜135度です。", "Supported MMD vertical FOV is 1..135 degrees."),
    FovPermission => ("非整数のVMD視野角には近似の明示許可が必要です。", "Noninteger VMD FOV requires explicit approximation permission."),
    LanguageArgs => ("GUIの言語は --language ja または --language en で指定してください。", "Specify GUI language with --language ja or --language en."),
    OutputUnicode => ("自動読み込みの出力パスはUnicodeで指定してください。", "Output path must be Unicode for automatic loading."),
    OutputPathLong => ("自動読み込みの出力パスが長すぎます。", "Output path is too long for automatic loading."),
    CompletionPipe => ("workerの完了通知パイプを利用できません。", "Worker completion pipe unavailable."),
    CompletionLarge => ("変換完了通知が大きすぎます。", "Conversion completion notification is too large."),
    ConversionFailed => ("変換に失敗しました。VMDの自動読み込みは行っていません。", "Conversion failed. The VMD has not been loaded automatically."),
    CompletionMonitor => ("完了通知の監視に失敗しました。\n{detail}", "Completion monitor failed.\n{detail}"),
    Delivered => ("変換したカメラVMDをMMDの標準読み込み経路へ渡しました。\nタイムラインと構図を確認してください。", "The converted camera VMD was sent to MMD's standard loader.\nCheck the timeline and composition."),
    SavedVmd => ("\n変換済みVMD: {path}\n標準のモーション読み込みを使用できます。", "\nConverted VMD: {path}\nYou can use the standard motion loader."),
    DeliveryFailed => ("自動読み込みを完了できませんでした。\n{detail}{saved}", "Automatic loading could not be completed.\n{detail}{saved}"),
    CompletionInvalid => ("不正な変換完了通知です。", "Invalid conversion completion notification."),
    CompletionPath => ("完了通知のパスが不正です。", "Invalid path in completion notification."),
    AbsolutePath => ("完了通知には絶対パスが必要です。", "Completion notification requires an absolute path."),
    NotVmd => ("出力がVMDではありません。", "The output is not a VMD file."),
    NotRegularVmd => ("通常のカメラVMDファイルではありません。", "This is not a regular camera VMD file."),
    CameraOnly => ("生成ファイルがcamera-only VMD形式ではありません。", "The generated file is not a camera-only VMD."),
    HostUnavailable => ("MMDが閉じられているか、別のダイアログが開いています。", "MMD is closed or another dialog is open."),
    CameraMode => ("MMDをカメラ・照明・アクセサリ操作に切り替えてください。", "Switch MMD to camera/light/accessory mode."),
    PathLong => ("パスが長すぎます。", "The path is too long."),
    AllocateDrop => ("ファイル通知のメモリを確保できません。", "Could not allocate the file notification."),
    LockDrop => ("ファイル通知を作成できません。", "Could not create the file notification."),
    MenuFailed => ("メニュー通知を接続できませんでした。", "Could not attach menu notifications."),
    WorkerState => ("workerの状態を利用できません。", "Worker state unavailable."),
    RegisterCompletion => ("完了通知を登録できません。", "Cannot register completion notification."),
    PluginModule => ("プラグインモジュールを特定できません。", "Cannot locate the plugin module."),
    PluginDirectory => ("プラグインのフォルダーを特定できません。", "Cannot locate the plugin directory."),
    WorkerLaunch => ("workerを起動できませんでした。\n{path}\n{detail}", "Could not launch the worker.\n{path}\n{detail}")
}

pub fn external(language: Language, key: Text, detail: impl std::fmt::Display) -> String {
    language.format(key, &[("detail", &detail.to_string())])
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GuiArgs { pub language: Language, pub plugin_result: bool }
impl GuiArgs {
    pub fn parse(args: &[OsString]) -> Result<Option<Self>, &'static str> {
        if args.first().is_none_or(|v| v != "--gui") { return Ok(None); }
        let mut result = Self { language: Language::Ja, plugin_result: false };
        let mut language_seen = false;
        let mut i = 1;
        while i < args.len() {
            if args[i] == "--plugin-result" && !result.plugin_result { result.plugin_result = true; }
            else if args[i] == "--language" && !language_seen {
                i += 1;
                result.language = args.get(i).and_then(|v| Language::parse(v)).ok_or("--language must be ja or en")?;
                language_seen = true;
            } else { return Err("invalid or repeated GUI option"); }
            i += 1;
        }
        Ok(Some(result))
    }
}

#[cfg(test)] mod locale_tests {
    use super::*;
    #[test] fn catalog_is_complete_and_placeholders_match() {
        fn fields(s: &str) -> std::collections::BTreeSet<&str> {
            s.split('{').skip(1).map(|p| p.split('}').next().unwrap()).collect()
        }
        for &key in ALL {
            let ja = Language::Ja.text(key); let en = Language::En.text(key);
            assert!(!ja.is_empty() && !en.is_empty(), "{key:?}");
            assert_eq!(fields(ja), fields(en), "{key:?}");
        }
        assert_eq!(Language::En.text(Text::Import), "Import .blend...");
        assert_eq!(Language::En.text(Text::Convert), "Convert");
    }
    #[test] fn gui_args_keep_legacy_and_explicit_languages() {
        let parse = |args: &[&str]| GuiArgs::parse(&args.iter().map(OsString::from).collect::<Vec<_>>());
        assert_eq!(parse(&["--gui"]).unwrap().unwrap().language, Language::Ja);
        for code in ["ja", "en"] {
            let parsed = parse(&["--gui", "--plugin-result", "--language", code]).unwrap().unwrap();
            assert!(parsed.plugin_result); assert_eq!(parsed.language.code(), code);
            let error = parsed.language.format(Text::FrameError, &[("frame", "7"), ("camera", "Camera日本語"), ("detail", parsed.language.text(Text::Aspect))]);
            assert!(error.contains("Camera日本語") && error.contains("7"));
        }
        for args in [vec!["--gui", "--language"], vec!["--gui", "--language", "fr"], vec!["--gui", "--language", "en", "--language", "ja"], vec!["--gui", "--plugin-result", "--plugin-result"]] { assert!(parse(&args).is_err()); }
        assert!(parse(&["--blender", "x"]).unwrap().is_none());
    }
}
