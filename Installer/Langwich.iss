; Langwich Installer — Inno Setup 7
#define MyAppName      "Langwich"
#define MyAppVersion   "1.0.0"
#define MyAppPublisher "Langwich"
#define MyAppExeName   "Langwich.exe"
#define MyAppSourceDir "..\DevOver\publish"

[Setup]
AppId={{B5E3A7F2-9C4D-4E8A-A1B2-C3D4E5F6A7B8}
AppName={#MyAppName}
AppVersion={#MyAppVersion}
AppPublisher={#MyAppPublisher}
DefaultDirName={autopf}\{#MyAppName}
DefaultGroupName={#MyAppName}
OutputDir=Output
OutputBaseFilename=Langwich-Setup-{#MyAppVersion}
Compression=lzma2/ultra64
SolidCompression=yes
WizardStyle=modern
PrivilegesRequired=lowest
PrivilegesRequiredOverridesAllowed=dialog

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Tasks]
Name: "desktopicon"; Description: "Create a desktop icon"; GroupDescription: "Additional icons:"; Flags: unchecked

[Files]
Source: "{#MyAppSourceDir}\*"; DestDir: "{app}"; Flags: ignoreversion recursesubdirs createallsubdirs
Source: "{#MyAppSourceDir}\Resources\Icons\langwich.ico"; DestDir: "{app}\Resources\Icons"; Flags: ignoreversion

[Icons]
Name: "{group}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; IconFilename: "{app}\Resources\Icons\langwich.ico"
Name: "{group}\Uninstall {#MyAppName}"; Filename: "{uninstallexe}"
Name: "{autodesktop}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; IconFilename: "{app}\Resources\Icons\langwich.ico"; Tasks: desktopicon

[Registry]
Root: HKCU; Subkey: "Software\Microsoft\Windows\CurrentVersion\Run"; ValueType: string; ValueName: "Langwich"; ValueData: """{app}\{#MyAppExeName}"""; Flags: uninsdeletevalue

[Run]
Filename: "{app}\{#MyAppExeName}"; Description: "Launch {#MyAppName}"; Flags: nowait postinstall skipifsilent

[UninstallDelete]
Type: filesandordirs; Name: "{localappdata}\Langwich"
Type: filesandordirs; Name: "{localappdata}\DevOver"
Type: filesandordirs; Name: "{app}"

[Code]
function IsDotNetInstalled(): Boolean;
var
  ResultCode: Integer;
  Output: AnsiString;
  TmpFile: String;
begin
  Result := False;
  TmpFile := ExpandConstant('{tmp}\dotnet_check.txt');
  if Exec('cmd.exe', '/c "dotnet --list-runtimes > "' + TmpFile + '" 2>&1"', '', SW_HIDE, ewWaitUntilTerminated, ResultCode) then
  begin
    if LoadStringFromFile(TmpFile, Output) then
    begin
      if Pos('Microsoft.NETCore.App', String(Output)) > 0 then
        Result := True;
    end;
  end;
  DeleteFile(TmpFile);
end;

function PrepareToInstall(var NeedsRestart: Boolean): String;
var
  ResultCode: Integer;
  NeedsInstall: Boolean;
begin
  Result := '';
  NeedsInstall := not IsDotNetInstalled();
  if NeedsInstall then
  begin
    if MsgBox(
      'Langwich needs .NET Runtime to run.'#13#10#13#10 +
      'Would you like to download and install it automatically?'#13#10 +
      '(Approximately 25 MB)',
      mbConfirmation, MB_YESNO) = IDYES then
    begin
      Exec(
        'powershell.exe',
        '-Command "Start-Process ''https://dotnet.microsoft.com/download/dotnet/thank-you/runtime-desktop-10.0.0-windows-x64-installer'' -Wait"',
        '', SW_SHOWNORMAL, ewWaitUntilTerminated, ResultCode);
      MsgBox('Please complete the .NET Runtime installation, then run Langwich again.', mbInformation, MB_OK);
    end
    else
    begin
      Result := 'Langwich requires .NET 10 Desktop Runtime to run.';
      Exit;
    end;
  end;
end;
