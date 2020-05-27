# kasm

This is Kris's Assembler for the [K64](https://github.com/transitorykris/krisos).

## Goals

 * Implement the WDC 6502 [instruction set](instructions.md) -- using the [WDC6502 datasheet](https://eater.net/datasheets/w65c02s.pdf)
 * Provide basic [assembler directives](directives.md)

I'll know when this is done because I'll be able to assemble [Microchess](https://en.wikipedia.org/wiki/Microchess) and it'll be byte for byte identical to ca65 or vasm.

## Non-goals

 * Implement macros

## Warning

I do not know Rust, and I don't know how to build an assembler. This is probably not the code you're looking for.

## Assembler completeness

### Directives

|Directive|Implemented|Notes|
|---------|-----------|-----|
|.ascii   |partial    |The scanner probably handles ;'s poorly|
|.bytes   |complte    ||
|.equ     |partial    |Only hands $1234 words, and will panic on bad input|
|.org     |partial    |Does not work correctly for $00 zeropage addresses|

### Instructions
|OpC|Accum|absolute|abs,X|abs,Y|#immed|implied|indirect|X-indexed|Indirect-Y|relative|zeropage|zerop,X|zerop,Y|
|---|-----|--------|-----|-----|------|-------|--------|---------|----------|--------|--------|-------|-------|
|ADC|     |        |     |     |      |       |        |         |          |        |        |       |       |
|AND|     |        |     |     |      |       |        |         |          |        |        |       |       |
|ASL|     |        |     |     |      |       |        |         |          |        |        |       |       |
|BBR|     |        |     |     |      |       |        |         |          |        |        |       |       |
|BCC|     |        |     |     |      |       |        |         |          |        |        |       |       |
|BCS|     |        |     |     |      |       |        |         |          |        |        |       |       |
|BEQ|     |        |     |     |      |       |        |         |          |        |        |       |       |
|BIT|     |        |     |     |      |       |        |         |          |        |        |       |       |
|BMI|     |        |     |     |      |       |        |         |          |        |        |       |       |
|BNE|     |        |     |     |      |       |        |         |          |        |        |       |       |
|BPL|     |        |     |     |      |       |        |         |          |        |        |       |       |
|BRA|     |        |     |     |      |       |        |         |          |        |        |       |       |
|BRK|     |        |     |     |Yes   |       |        |         |          |        |        |       |       |
|BVC|     |        |     |     |      |       |        |         |          |        |        |       |       |
|BVS|     |        |     |     |      |       |        |         |          |        |        |       |       |
|CLC|     |        |     |     |Yes   |       |        |         |          |        |        |       |       |
|CLD|     |        |     |     |Yes   |       |        |         |          |        |        |       |       |
|CLI|     |        |     |     |Yes   |       |        |         |          |        |        |       |       |
|CLV|     |        |     |     |Yes   |       |        |         |          |        |        |       |       |
|CMP|     |        |     |     |      |       |        |         |          |        |        |       |       |
|CPX|     |        |     |     |      |       |        |         |          |        |        |       |       |
|CPY|     |        |     |     |      |       |        |         |          |        |        |       |       |
|DEC|     |        |     |     |      |       |        |         |          |        |        |       |       |
|DEX|     |        |     |     |Yes   |       |        |         |          |        |        |       |       |
|DEY|     |        |     |     |Yes   |       |        |         |          |        |        |       |       |
|EOR|     |        |     |     |      |       |        |         |          |        |        |       |       |
|INC|     |        |     |     |      |       |        |         |          |        |        |       |       |
|INX|     |        |     |     |Yes   |       |        |         |          |        |        |       |       |
|INY|     |        |     |     |Yes   |       |        |         |          |        |        |       |       |
|JMP|     |        |     |     |      |       |        |         |          |        |        |       |       |
|JSR|     |        |     |     |      |       |        |         |          |        |        |       |       |
|LDA|     |        |     |     |      |       |        |         |          |        |        |       |       |
|LDX|     |        |     |     |      |       |        |         |          |        |        |       |       |
|LDY|     |        |     |     |      |       |        |         |          |        |        |       |       |
|LSR|     |        |     |     |      |       |        |         |          |        |        |       |       |
|NOP|     |        |     |     |Yes   |       |        |         |          |        |        |       |       |
|ORA|     |        |     |     |      |       |        |         |          |        |        |       |       |
|PHA|     |        |     |     |Yes   |       |        |         |          |        |        |       |       |
|PHP|     |        |     |     |Yes   |       |        |         |          |        |        |       |       |
|PHX|     |        |     |     |      |       |        |         |          |        |        |       |       |
|PHY|     |        |     |     |      |       |        |         |          |        |        |       |       |
|PLA|     |        |     |     |      |       |        |         |          |        |        |       |       |
|PLP|     |        |     |     |      |       |        |         |          |        |        |       |       |
|PLX|     |        |     |     |      |       |        |         |          |        |        |       |       |
|PLY|     |        |     |     |      |       |        |         |          |        |        |       |       |
|RMB|     |        |     |     |      |       |        |         |          |        |        |       |       |
|ROL|     |        |     |     |      |       |        |         |          |        |        |       |       |
|ROR|     |        |     |     |      |       |        |         |          |        |        |       |       |
|RTI|     |        |     |     |Yes   |       |        |         |          |        |        |       |       |
|RTS|     |        |     |     |      |       |        |         |          |        |        |       |       |
|SBC|     |        |     |     |      |       |        |         |          |        |        |       |       |
|SEC|     |        |     |     |Yes   |       |        |         |          |        |        |       |       |
|SED|     |        |     |     |Yes   |       |        |         |          |        |        |       |       |
|SEI|     |        |     |     |Yes   |       |        |         |          |        |        |       |       |
|SMB|     |        |     |     |      |       |        |         |          |        |        |       |       |
|STA|     |        |     |     |      |       |        |         |          |        |        |       |       |
|STP|     |        |     |     |      |       |        |         |          |        |        |       |       |
|STX|     |        |     |     |      |       |        |         |          |        |        |       |       |
|STY|     |        |     |     |      |       |        |         |          |        |        |       |       |
|STZ|     |        |     |     |      |       |        |         |          |        |        |       |       |
|TAX|     |        |     |     |Yes   |       |        |         |          |        |        |       |       |
|TAY|     |        |     |     |Yes   |       |        |         |          |        |        |       |       |
|TRB|     |        |     |     |      |       |        |         |          |        |        |       |       |
|TSB|     |        |     |     |      |       |        |         |          |        |        |       |       |
|TSX|     |        |     |     |      |       |        |         |          |        |        |       |       |
|TXA|     |        |     |     |Yes   |       |        |         |          |        |        |       |       |
|TXS|     |        |     |     |      |       |        |         |          |        |        |       |       |
|TYA|     |        |     |     |Yes   |       |        |         |          |        |        |       |       |
|WAI|     |        |     |     |      |       |        |         |          |        |        |       |       |


## License

Copyright 2020 Kris Foster