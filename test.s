		jal bong
	addi $v0 $zero 10
	syscall
fib:	addi $sp $sp -8
	sw $sp 0($a0)
	addi $sp $sp 8
	jr $ra
bong:	addi $sp $sp 0
	lui $t0 4096
	lw $t1 24($t0)
	sw $t1 20($t0)
	lui $t0 4096
	lw $t1 20($t0)
	sw $t1 16($t0)
	lui $t0 4096
	lw $t1 16($t0)
	sw $t1 12($t0)
	lui $t0 4096
	lw $t1 12($t0)
	sw $t1 8($t0)
	lui $t0 4096
	lw $t1 8($t0)
	sw $t1 4($t0)
	lui $t0 4096
	lw $t1 4($t0)
	sw $t1 0($t0)
	addi $sp $sp 0
	jr $ra
