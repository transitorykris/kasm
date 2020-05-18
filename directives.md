# Directives

## .ascii

Stores a string of ascii characters at the current address

Example:

```
.ascii "Hello, World!\n\r"
```

Stores ```48656C6C6F2C20576F726C06421A0C``` at the current address

## .byte

Stores a string of bytes at the current address

Example:

```
.byte $1, $02, $FF
```

Stores `0102FF` at the current address

## .org

Sets the location counter to this value

Example:

```
.org $FFFC
```

Puts the next instruction in location `$FFFC`
