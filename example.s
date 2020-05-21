; This is my simple program

    lda #$1a            ; Load 0 into the A register
    inc                 ; Add 1 to it
    sta $a45f           ; Store it in the zeropage at $00

    ; End
