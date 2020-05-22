; This is my simple program

    lda #$1a            ; Load 0x1a into the A register
    inc                 ; Add 1 to it
    sta $a45f           ; Store it at the absolute location $a45f

    nop
    nop
    nop
    
    pha
    stp
    
    ; End
