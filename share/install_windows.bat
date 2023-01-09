@echo Moonwatch.rs installer
@echo ----------------------

@set MOONWATCHDIR=%USERPROFILE%\.moonwatch-rs

@echo Installing into %MOONWATCHDIR%

mkdir "%MOONWATCHDIR%"

copy moonwatcher.exe "%MOONWATCHDIR%"

@if exist "%MOONWATCHDIR%\config.json" (
    @echo config.json already exists, not copying default
) else (
    @echo copying default config.json
    copy config.json "%MOONWATCHDIR%"
)

@echo Installing to Startup menu

@set SHORTCUT='%APPDATA%\Microsoft\Windows\Start Menu\Programs\Startup\moonwatcher.lnk'
@set WORKINGDIRECTORY='%MOONWATCHDIR%'
@set TARGET='%MOONWATCHDIR%\moonwatcher.exe'
@set ARGUMENTS='config.json'
@set DESCRIPTION='Moonwatch.rs daemon'
@set PWS=powershell.exe -ExecutionPolicy Bypass -NoLogo -NonInteractive -NoProfile

%PWS% -Command "$ws = New-Object -ComObject WScript.Shell; $s = $ws.CreateShortcut(%SHORTCUT%); $S.WorkingDirectory = %WORKINGDIRECTORY%; $S.TargetPath = %TARGET%; $S.Arguments = %ARGUMENTS%; $S.Description = %DESCRIPTION%; $S.Save()"

@echo Installation is finished.
@pause
