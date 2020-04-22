(* ::Package:: *)

SetDirectory@NotebookDirectory[];
all = Sort@RandomInteger[UnixTime /@ {DateObject[{2020, 4, 20}], Now}, 18];
log[unix_] := TemplateApply["\
$commitDateString = \"``\"
$env:GIT_COMMITTER_DATE = $commitDateString
git commit --amend --no-edit --date $commitDateString
git rebase --continue
$env:GIT_COMMITTER_DATE = \"\"
", {DateString[FromUnixTime[unix], "ISODateTime"]}
];
powershell = StringRiffle[log /@ all, "\n"];
powershell // CopyToClipboard
Export["time-travel.ps1", powershell, "Text"]
