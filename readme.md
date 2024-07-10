# vphone

Searches vn edict for minimal phonetic pairs. Outputs a csv on stdout.

Run `cargo install` in this directory.

For example, if you search for words with the vowels `a` or `e` that
differ only in vowel:

```
vphone -v a -v e --mode vowel
```

You'll get a csv like this.

```
right,left
gan,geneve
bả,bẻ
chan,chen
chét,chát
da,de
khen,khan
khé,khá
khét,khát
kháp,khép
kỳ hẹn,kỳ hạn
...
```

If you want to generate a training pair list for distinguishing
fast and slow rising tones for example, you could run this query:

```
vphone -v a -v ă -t rising -f t -f ch --mode final_consonant
```

and get results like:

```
right,left
khát,khách
khách,khát
nhách,nhát
rách,rát
bách,bát
lát,lách
nát,nách
sách,sát
trát,trách
trách,trát
xát,xách
ách,át
thát,thách
bát bộ,bách bộ
...
```
