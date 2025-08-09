export type Exercise = { id:string; title:string; durationSec:number; steps:string[] };
export const EXERCISES: Exercise[] = [
  { id:'box-breathing', title:'Box breathing 4-4-4-4', durationSec:120, steps:[
    'Andas in 4 sek', 'Håll 4 sek', 'Andas ut 4 sek', 'Håll 4 sek', 'Upprepa 6–8 varv'
  ]},
  { id:'478', title:'4–7–8-ande', durationSec:90, steps:[
    'Andas in 4 sek', 'Håll 7 sek', 'Andas ut 8 sek', 'Upprepa 4–6 varv'
  ]},
  { id:'eye-palming', title:'Ögonpalmering', durationSec:60, steps:[
    'Gnugga handflator varma', 'Kupa över slutna ögon 30–60 sek', 'Slappna av i käken'
  ]},
  { id:'neck-rolls', title:'Nackrullningar', durationSec:60, steps:[
    'Rulla långsamt höger → vänster 3×', 'Axellyft 5×', 'Håll axlar sänkta och andas'
  ]},
  { id:'wrist-flexors', title:'Handleder & fingrar', durationSec:60, steps:[
    'Sträck arm, böj handled, håll 10 sek/hand', 'Knäpp 10 långsamma öppna/stäng'
  ]},
  { id:'posture-reset', title:'Posture reset', durationSec:45, steps:[
    'Fötter i golvet', 'Lätt svank, bröst upp', 'Axlar bak & ner', '3 djupa andetag'
  ]},
  { id:'micro-walk', title:'Mikropromenad', durationSec:120, steps:[
    'Res dig upp', 'Gå långsamt, rör axlar/handleder', 'Titta långt bort i 20 sek'
  ]}
];
