# vphone

Searches vn edict for minimal phonetic pairs. Outputs a csv on stdout.

For example, if you search for words with the vowels `a` or `e` that
differ only in vowel:

```
vphone -v a -v e --mode vowel
```

You'll get a csv like this.

```
kind,isolated_right,isolated_left,right,left
Vowel,a,e,gan,geneve
Vowel,a,e,bả,bẻ
Vowel,a,e,chan,chen
Vowel,e,a,chét,chát
Vowel,a,e,da,de
Vowel,e,a,khen,khan
Vowel,e,a,khé,khá
Vowel,e,a,khét,khát
Vowel,a,e,kháp,khép
Vowel,e,a,kỳ hẹn,kỳ hạn
...
```

If you want to generate a training pair list for distinguishing
fast and slow rising tones for example, you could run this query:

```
vphone -v a -v ă -t rising -f t -f ch --mode tone
```

and get results like:

```
kind,isolated_right,isolated_left,right,left
FinalConsonant,t,ch,khát,khách
FinalConsonant,ch,t,khách,khát
FinalConsonant,ch,t,nhách,nhát
FinalConsonant,ch,t,rách,rát
FinalConsonant,ch,t,bách,bát
FinalConsonant,t,ch,lát,lách
FinalConsonant,t,ch,nát,nách
FinalConsonant,ch,t,sách,sát
FinalConsonant,t,ch,trát,trách
FinalConsonant,ch,t,trách,trát
FinalConsonant,t,ch,xát,xách
FinalConsonant,ch,t,ách,át
FinalConsonant,t,ch,thát,thách
FinalConsonant,t,ch,bát bộ,bách bộ
...
```
