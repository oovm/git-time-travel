# Git Time Travel

将 git 的提交平均分布到一段时间内

## PowerShell Version

```sh
git-utils reset Head~20 -i # 开启交互式变基
# 然后把 pick 全部替换成 edit 并关闭 editor
# 直接执行 time-travel.ps1 即可
```

## git

```sh
git-root "🎂 Project initialized!"
git-time 2a990148 2019-01-01
```


## Development

```sh
cargo install --path .
```