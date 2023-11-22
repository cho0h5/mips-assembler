# mips-assembler

# Grammer
```
PROGRAM -> LINE PROGRAM | ㅅ

LINE -> LABEL : CODE | CODE

CODE -> R | I | J

R ->
  add
  addu
  and
  jr
  nor
  or
  slt
  sltu
  sll
  srl
  sub
  subu
  div
  divu
  mfhi
  mflo
  mult
  multu
  sra
  syscall

I ->
  addi
  addiu
  andi
  beq
  bne
  lbu
  lhu
  lui
  lw
  ori
  slti
  sltiu
  sb
  sh
  sw

J ->
  j
  jal

REG ->
  $zero |
  $at |
  $v0 |
  $v1 |
  $a0 |
  $a1 |
  $a2 |
  $a3 |
  $t0 |
  $t1 |
  $t2 |
  $t3 |
  $t4 |
  $t5 |
  $t6 |
  $t7 |
  $s0 |
  $s1 |
  $s2 |
  $s3 |
  $s4 |
  $s5 |
  $s6 |
  $s7 |
  $t8 |
  $t9 |
  $k0 |
  $k1 |
  $gp |
  $sp |
  $fp |
  $ra
```
