; This is my simple program

.org $5000

.ascii "this does not ; have a comment"
.ascii "this one is also ; comment free"    ; hello comment!
.ascii "check this ->"  ; this is just one ; comment
.ascii "oh no, \" double quotes"

.org $0000

zp:thisiszeropage:
    brk
    nop
    brk
    nop

zp:alsozp:

.org $1000

.equ myvalue = $acbd

start:
    lda #$2b            ; Load 0x1a into the A register
    inc                 ; Add 1 to it
    sta $a45f           ; Store it at the absolute location $a45f

    ; The assembler is hitting only a few addressing modes at the moment
    sta ($43,x)
    sta ($01),y
    sta (zp:thisiszeropage),y
    lda zp:alsozp
    lda myvalue         ; backwards
    lda another_value   ; forwards

no_opping:
    nop
    nop
    nop
    .equ another_value=$4321

    jmp start           ; Backward reference
    nop

    jmp wrap_it_up      ; Forware reference
    nop

    sta no_opping,x
    sta no_opping,y
    jmp (wrap_it_up)
    
    jmp out_of_order

.byte $01,$02, $a3,$b4,$c5

    .org $2abc

wrap_it_up:
    pha
    stp

.org $2000
    brk
    nop
    brk
    nop
    brk
    nop
    .ascii "Hello, \"6502\" World!\n\r"
out_of_order:
    nop
    nop
    nop
    brk

.org $4000
    .ascii "This data exists far away from other code!"

    ; End
