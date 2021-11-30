(* ::Package:: *)

SetDirectory@NotebookDirectory[];
all = Sort@RandomInteger[UnixTime /@ {
    DateObject[{2020, 4, 1}], 
    DateObject[{2020, 4, 15}]
}, 18];
log[unix_] := TemplateApply["\
$commitDateString = \"``\"
$env:GIT_COMMITTER_DATE = $commitDateString
git commit --amend --no-edit --date $commitDateString
git rebase --continue
", {DateString[FromUnixTime[unix], "ISODateTime"]}
];
last="
$env:GIT_COMMITTER_DATE =\"\"
";
powershell = StringRiffle[log /@ all, "\n"]<>last;
powershell // CopyToClipboard
Export["time-travel.ps1", powershell, "Text"]




(* ::Package:: *)
(* git rebase -i --root *)
SetDirectory@NotebookDirectory[];
all = Sort@RandomInteger[UnixTime /@ {
    DateObject[{2021,7, 27}], 
    DateObject[{2021,8, 10}]
}, 19];
log[unix_] := TemplateApply["\
commitDateString=\"``\"
GIT_COMMITTER_DATE=$commitDateString git commit --amend --no-edit --date $commitDateString
git rebase --continue
", {DateString[FromUnixTime[unix], "ISODateTime"]}
];
last="
GIT_COMMITTER_DATE=\"\"
";

powershell = StringRiffle[log /@ all, "\n"]<>last;

powershell // CopyToClipboard
Export["time-travel.sh", powershell, "Text"]
