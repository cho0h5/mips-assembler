# mips-assembler

# Grammer
```
PROGRAM -> LINE PROGRAM | ㅅ

LINE -> LABEL : CODE | CODE

CODE -> R | I | J

R ->
  add REG REG REG |
  addu REG REG REG |
  and REG REG REG |
  jr REG |
  nor REG REG REG |
  or REG REG REG |
  slt REG REG REG |
  sltu REG REG REG |
  sll REG REG IMM5 |
  srl REG REG IMM5 |
  sub REG REG REG |
  subu REG REG REG |
  div REG REG |
  divu REG REG |
  mfhi REG |
  mflo REG |
  mult REG REG |
  multu REG REG |
  sra REG REG IMM5 |
  syscall

I ->
  addi REG REG IMM16 |
  addiu REG REG IMM16 |
  andi REG REG IMM16 |
  beq REG REG LABEL |
  bne REG REG LABEL |
  lbu REG IMM16(REG) |
  lhu REG IMM16(REG) |
  lui REG IMM16 |
  lw REG IMM16(REG) |
  ori REG REG IMM16 |
  slti REG REG IMM16 |
  sltiu REG REG IMM16 |
  sb REG IMM16(REG) |
  sh REG IMM16(REG) |
  sw REG IMM16(REG)

J ->
  j LABEL |
  jal LABEL

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

IMM5 -> `5bit constant`
IMM16 -> `16bit constant`

LABEL -> `string`
```
## Example
```asm
		addi $a0   $zero 6
		jal fact
		add $a0 $v0 $zero
		addi $v0 $zero 1
		syscall
		addi $a0 $zero 10
		addi $v0 $zero 11
		syscall
		addi $v0 $zero 10
		syscall

fact:	addi $sp $sp -8
		sw $ra 4($sp)
		sw $a0 0($sp)
		slti $t0 $a0 1
		beq $t0 $zero L1
		addi $v0 $zero 1
		addi $sp $sp 8
		jr $ra
L1:		addi $a0 $a0 -1
		jal fact
		lw $a0 0($sp)
		lw $ra 4($sp)
		addi $sp $sp 8
		mult $v0 $a0
		mflo $v0
		jr $ra   
```
