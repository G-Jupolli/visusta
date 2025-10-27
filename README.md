# Visusta

Pure rust image filters

---

## Origin

This started when I saw the video [I Tried Turning Games Into Text](https://www.youtube.com/watch?v=gg40RWiaHRY) where the creator used  \
shaders to compute a live ascii filter with edge detection.  \
This repo is my attempt at implementing the methods myself.

---

## Example

<div>
  <img src="https://images.pexels.com/photos/1543417/pexels-photo-1543417.jpeg?auto=compress&cs=tinysrgb&w=630&h=375&dpr=1"/>
</div>
<div>
  <sub>Photo by Jean van der Meulen from Pexels: https://www.pexels.com/photo/close-up-photography-of-owl-1543417/</sub>
</div>

```

                                        c . c . .   - ; ;   . c   ;     ;
                  .   .       .   o . \ @ P .   ; \   0 \ . . ;       o P c ; . 0 .
                    c .   P o   o     c 0 ?   . ; P .     ; ;       c o / c 0 o     o c
                c     P   ; 0 ? P .     \ / c - - P . .   .   .   o ? P . ; ;   c
              P P c . . ; o ; ; @ c   .   \ \ @ ? . ; .     . ;     o c o ? o c 0 .   .
              . ; o ; o o ? 0   . | @ o o . . o ; 0 P .     c . . .   c 0 \ 0 o     c
                . 0 ; ; 0 # 0 o   c ? 0 @ - ; . . 0 c o .   ; ; c - P o / / ; ; ? P ?
              ; ? # ? ? . o P c \ ; ? | ? @ P c ;     / ; . @ P o c ? \ .   / o P 0 # o
              ; # # # 0     0 ? P c c P @ # # @ ?   o o P # P | P ? o     ; ; . o ? 0 ? P o
              @ # ? o \ . ; ; # ? ; c .   ; ? | 0 P P o ; ? c c 0 # o     / \ # c . o 0 # ; . .
            c @ P ; @ @ \     # \ @ ; o . P @ 0 @ # 0 @ | P . ? o o .   . ; / o \     o ? - . o
            0 ?   P # \ \ ;   \ 0 0 ? ? . P @ . ; o ? ; c @ P 0 @ ;   ? # ? / \ \ ;     @ ? ;
            ? P   ? \ \ \ # \ ; ? # P @ 0 0 ? c P ? 0 . . ? # o ;       c ? \ o \ \     - ? ;
          . # P ; 0 c ? 0 . c     o @ # ? 0 P c P c c ; ? # # ;       ;         \ \     . ? c .
          ; # ; ; \ \ .             . # # ? . ; . ; o 0 @ # o         . ;       \ \ ;     ? . .
          ; # o c # \ \ 0   ?         o # P P c P P P @ / /           . ; ; \ \ \ \ \   . | ; . .
          c # c c # \ \ \ c ? o     . . ? 0 c o o c ; c / .         o # . \ - \ \ \ ;   ; | ;   .
            @ c ; # \ \ # 0 ; # ? 0 o . c # o .   0 P c .       \ # ? ; o # \ \ \ \ .   ; c     ;
          ; P o . @ # # # # c . o / \ @ o P o . ;   o o     . - c . . o # \ - - - -     P .     .
            c -   \ # / # # # @ ? / @ # # P \ . # @ @ P P . 0 ? P - @ @ ? \ - - \ -               o
            c 0 . \ @ # # / ? @ P @ # # # # # P # # P @ ? ? @ # # \ . - @ - - - - .         .
            o ;     \ / / / / / # # # # # # # ; @ 0 . # # # # # # @ P - - \ - - .         .       .
            o . .   \ / / / / / @ # 0 # # # # P # ; c @ ? # # @ P P P - - - - o           o .
              .     . / / / 0 / # # @ # # # @ @ # . . ; . P | ? 0 \ o \ - - -           - c .   ;
              ;       ; ? ? \ \ @ @ # # # ? . ? o         . 0 \ P \ \ \ 0 \ .       ; . c ;     0
              ; c       c 0 @ @ 0 0 ? # # # # ? c     . ? # # o ; 0 0 0 c o o \ c - ? . .   ; c P
            . ; @ c     0 ? 0 / ? \ ? @ # # # @ .   . @ # ? / 0 P P ; .   c . ? ? ; o c ; . c   .
              ; 0 ; . c   o @ # ? 0 c ? 0 # # #     0 # P / ; ; .     . P o   ? ? ; ; ? # . - ;
              @ # ; 0 # / # # # c . \ . o ? # 0 .   o P \ / ; .       # ?               ? - P .
          . o c . P # c ? # @ # @ o ? # o . o 0 o . ;   ; ? 0 .   . . c ;                   .
                c # ; P # P ? # ? 0 0 # # @ o c # 0 o .   o       P . .                   ; ;
            c   / o c @ c   P @ # . . ; # # P   c # 0   ; P .   o 0 c ;
          . 0 . o     P c . ; ; o ;     0 P       P             c . o
```
