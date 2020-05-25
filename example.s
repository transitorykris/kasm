; This is my simple program

.org $1000

start:
    lda #$2b            ; Load 0x1a into the A register
    inc                 ; Add 1 to it
    sta $a45f           ; Store it at the absolute location $a45f

    ; The assembler is hitting only a few addressing modes at the moment
    sta ($43,x)         ; to be implemented
    sta ($01),y

no_opping:
    nop
    nop
    nop

    jmp start           ; Backward reference
    nop

    jmp wrap_it_up      ; Forware reference
    nop

    sta no_opping,x
    sta no_opping,y
    jmp (wrap_it_up)
    
    jmp out_of_order

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
out_of_order:
    nop
    nop
    nop
    brk

    ; End
