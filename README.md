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
                                  ;       ;     ─
                          ; ─   ─ c o c ; / \   ; / ─ ; / c ─   ;   c ─ ─       ;
          ; o ;       ;   ;   ─ ─ / | ;     / ─     ; ─             ; / / ─ o
              ;   ─ P ; / ;     ; ─ \ ; \ \ 0 \ ;       c       ; ─ / / / / ;       ; ;
    ; o     c ;   ; c ─ ─ /     ; ; / ─ \ | /     ; ; ; ; /     ─ ─ / ; c ─ c   c
    ; c c   ; c c ─ c ; P / ; ; ; ; ─ / ─ P | ; | ; ; ; ; ; ;     ; ─ ; / c ─   c
      ; ;   ; ─ ─ P ─     ; | P ─ \ ; ; \ ; | | | ;     ; ;   ;     \ ─ ─ \ / |     ;
        c c ; / ─ 0 | P ; / / \ | ─ | c ; ; c P ; | ;   ; ; ─ ; ─ c ─ | / / ; / P c \
      o 0 0 \ / ; | / c ; ; / | | o \ c | ; ; ; ; / ; ; / \ | c \ ─ ─ /   / | ─ \ \ P c
      o P P \ \   ; P | \ \ / ; P P o P ─ | c ; ;   / P o | / o ; \       ;     o / \ \ ─ ; ;
    c 0 P P o ─     ; P / c P \ ; ; c o | o c ; / c \ P o ; ─ P 0 c     ; / | \ ; / \ \ 0 ; c
    0 P o ; \ \ \     \ P c / ; ;   ; \ o c P o ; c ; | / c o c c         ─ ─ P ─     ; P o \ ;
  c | /   o o ─ ─     \ ─ P P \ P ; o P o c P P P / | / ; / o c ─     / ─ / / ─ ─     ; \ \ \   ;
  P o   ; P ─ ─ ─ ─   \ \ o \ \ / ; \ | c ; / ; /   ; o c / / /     ─ o P / ─ ─ ─ ─     | 0 c
  P c   ─ ─ ─ ─ ─ P ─   \ P P \ ─ / o c c ; P o /     / o c /         ; o ─ ─ ─ ─ ─     | \ \
  0 c   \ ; ─ ─ /   ─       \ P o \ / \ ; \ c ; /   c P P /                     ─ ─       | \
; 0 ;   ─ ─ /                 c P P o ; c ; ; ; ; ─ o P /             |         ─ ─ ─     c |
; P c   o ─ ─ ─ ;   |           o o / ; ;   ─ o | o / /               |   ; ─ ─ ─ \ ─     \ \
| P ;   o ─ ─ ─ ─   |           ; P \ c o c ─     ─ / /             c   ─ \ ─ ─ ─ ─ ─     / | ;
; P ; ; ─ ─ ─ ─ ─ | ; o       /   \ ─ ─     ─ ─ ─ ─ /           ; c o   o ─ ─ ─ ─ ─       |
  o c   o ─ ─ / P o   c P P c / ; ; P ─       \ \ ;           ─ c ;   c 0 \ ─ ─ ─ ─       |       ;
; c |   / ─ ─ / P 0 ─ ─   ─ / ─ P ; / / ;   ;   c \         \ \   ; ─ 0 ─ ─ \ \ \ ─     \
  ; \   / ─ ─ / / ─ 0 0 / ─ / 0 0 P | ; ; o 0 / o \ / ; \ o o \ \ \ \ ─ ─ \ \ \ \ \                 |
  ; \ ; ─ ─ o / / / / P / / 0 0 0 0 P o | P 0 o / | ─ \ \ o P \ ─   \ P ─ \ \ \ \
  ; \ ;   | / / / / ─ / o P 0 0 0 0 P P ; o P ; ; P P P P o P P ─ \ ─ \ ─ \ \ \
  c ; ;     ─ / / / / / / / 0 / P 0 0 P ; | o   ; o o o P P o c \ \ \ \ \ \ ─ ─           ; \
    ; ;     / ─ / / / / o o / / o 0 P o \ o o ; | | | | | | \ \ ─ ; ; \ \ \ \             ; \     ;
    ;         ─ \ ─ ─ ─ ─ / o o P P o | | / ;         ; | \ o c ─ ─ ─ ─ ─ ─             \ \ \   ; c
    ; ;       / / ─ ─ ─ / ─ P ─ P o c ; c c ;       ; ; c ─ ─   ─ ─ ─ ─ ─           c   ;       ; |
    ; /       ; ─ ─ o o ─ ─ ─ o P P o o | c ;     ; o o o / ; ─ / c ─ ; c ─ ─ \ ─ ─ \ \ \   ;   ; \
  ; ; c /       ─ ─ o / / / ─ P \ o P o c ;     ; o P o / ─ ─ / ; ;     ; ; ; P o ; \ \ ;   \
    ; c c   /     \ / / o ─ ─ \ | ─ P P P ;     c o c / / / ;       ; ; c   \ o P ; ; \ o c   \ \
    o P / ; / P / P 0 0 o   ─ / ; \ \ o ─ \     / ─ ─ / ─ ;     ;   o o ;               \ \ \ \
; c o c / o 0 c / P / 0 / ; ; ─ c ; ; c / |   ; ; ; ; c c |     ; ; c c     ;
      ; / o ; ; 0 / / 0 P / / P P o ;   c o o ;       ; ;       ; ; ; ;                     ;
      ; o / / P P ; o P o / ; c o 0 P o ; c P P o ; ; c ;     ; c ; ;
  /   / ; ; | / ; ; / / P / ;   ; 0 0 c     c o ;     ; ;   ; c ; c ;

```
