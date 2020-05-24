; This is my simple program

    lda #$2b            ; Load 0x1a into the A register
    inc                 ; Add 1 to it
    sta $a45f           ; Store it at the absolute location $a45f

    ; The assembler is hitting only a few addressing modes at the moment
    sta ($4321,x)      ; to be implemented
    sta ($0001),y

    nop
    nop
    nop
    
    pha
    stp

    ; End
