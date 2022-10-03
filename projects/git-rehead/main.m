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




rewriteUnix[commits_Integer, start_DateObject] := rewriteUnix[commits, start, DayPlus[start, commits]]
rewriteUnix[commits_Integer, start_DateObject, end_DateObject] := Block[
    {
        all, log, last
    },
    all = Sort@RandomInteger[UnixTime /@ {start, end}, commits];
    log[unix_] := TemplateApply["\
commitDateString=\"``\"
GIT_COMMITTER_DATE=$commitDateString git commit --amend --no-edit --date $commitDateString
git rebase --continue
",
        {
            DateString[FromUnixTime[unix], "ISODateTime"]
        }
    ];
    last = "
GIT_COMMITTER_DATE=\"\"
";
    StringRiffle[log /@ all, "\n"] <> last
];


(* git rebase -i --root *)
SetDirectory@NotebookDirectory[];
powershell = rewriteUnix[26, DateObject[{2019, 3, 22}]];
powershell // CopyToClipboard;
Export["time-travel.sh", powershell, "Text"]
