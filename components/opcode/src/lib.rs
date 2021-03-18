mod disasm;

pub use disasm::{disasm, DisasmError};

#[derive(Debug)]
pub enum ArrayType {
    BOOLEAN = 4,
    CHAR = 5,
    FLOAT = 6,
    DOUBLE = 7,
    BYTE = 8,
    SHORT = 9,
    INT = 10,
    LONG = 11,
}

#[derive(Debug)]
pub enum Opcode {
    ///  		arrayref, index → value 	load onto the stack a reference from an array
    Aaload,

    ///  		arrayref, index, value → 	store a reference in an array
    Aastore,

    ///  		→ null 	push a null reference onto the stack
    AconstNull,

    ///  	1: index 	→ objectref 	load a reference onto the stack from a local variable #index
    Aload(u8),

    ///  		→ objectref 	load a reference onto the stack from local variable 0
    Aload0,

    ///  		→ objectref 	load a reference onto the stack from local variable 1
    Aload1,

    ///  		→ objectref 	load a reference onto the stack from local variable 2
    Aload2,

    ///  		→ objectref 	load a reference onto the stack from local variable 3
    Aload3,

    ///  	2: indexbyte1, indexbyte2 	count → arrayref 	create a new array of references of length count and component type identified by the class reference index (indexbyte1 << 8 | indexbyte2) in the constant pool
    Anewarray(u16),

    ///  		objectref → [empty] 	return a reference from a method
    Areturn,

    ///  		arrayref → length 	get the length of an array
    Arraylength,

    ///  	1: index 	objectref → 	store a reference into a local variable #index
    Astore(u8),

    ///  		objectref → 	store a reference into local variable 0
    Astore0,

    ///  		objectref → 	store a reference into local variable 1
    Astore1,

    ///  		objectref → 	store a reference into local variable 2
    Astore2,

    ///  		objectref → 	store a reference into local variable 3
    Astore3,

    ///  		objectref → [empty], objectref 	throws an error or exception (notice that the rest of the stack is cleared, leaving only a reference to the Throwable)
    Athrow,

    ///  		arrayref, index → value 	load a byte or Boolean value from an array
    Baload,

    ///  		arrayref, index, value → 	store a byte or Boolean value into an array
    Bastore,

    ///  	1: byte 	→ value 	push a byte onto the stack as an integer value
    Bipush(i8),

    ///  			reserved for breakpoints in Java debuggers; should not appear in any class file
    Breakpoint,

    ///  		arrayref, index → value 	load a char from an array
    Caload,

    ///  		arrayref, index, value → 	store a char into an array
    Castore,

    ///  	2: indexbyte1, indexbyte2 	objectref → objectref 	checks whether an objectref is of a certain type, the class reference of which is in the constant pool at index (indexbyte1 << 8 | indexbyte2)
    Checkcast(u16),

    ///  		value → result 	convert a double to a float
    D2f,

    ///  		value → result 	convert a double to an int
    D2i,

    ///  		value → result 	convert a double to a long
    D2l,

    ///  		value1, value2 → result 	add two doubles
    Dadd,

    ///  		arrayref, index → value 	load a double from an array
    Daload,

    ///  		arrayref, index, value → 	store a double into an array
    Dastore,

    ///  		value1, value2 → result 	compare two doubles, 1 on NaN
    Dcmpg,

    ///  		value1, value2 → result 	compare two doubles, -1 on NaN
    Dcmpl,

    ///  		→ 0.0 	push the constant 0.0 (a double) onto the stack
    Dconst0,

    ///  		→ 1.0 	push the constant 1.0 (a double) onto the stack
    Dconst1,

    ///  		value1, value2 → result 	divide two doubles
    Ddiv,

    ///  	1: index 	→ value 	load a double value from a local variable #index
    Dload(u8),

    ///  		→ value 	load a double from local variable 0
    Dload0,

    ///  		→ value 	load a double from local variable 1
    Dload1,

    ///  		→ value 	load a double from local variable 2
    Dload2,

    ///  		→ value 	load a double from local variable 3
    Dload3,

    ///  		value1, value2 → result 	multiply two doubles
    Dmul,

    ///  		value → result 	negate a double
    Dneg,

    ///  		value1, value2 → result 	get the remainder from a division between two doubles
    Drem,

    ///  		value → [empty] 	return a double from a method
    Dreturn,

    ///  	1: index 	value → 	store a double value into a local variable #index
    Dstore(u8),

    ///  		value → 	store a double into local variable 0
    Dstore0,

    ///  		value → 	store a double into local variable 1
    Dstore1,

    ///  		value → 	store a double into local variable 2
    Dstore2,

    ///  		value → 	store a double into local variable 3
    Dstore3,

    ///  		value1, value2 → result 	subtract a double from another
    Dsub,

    ///  		value → value, value 	duplicate the value on top of the stack
    Dup,

    ///  		value2, value1 → value1, value2, value1 	insert a copy of the top value into the stack two values from the top. value1 and value2 must not be of the type double or long.
    DupX1,

    ///  		value3, value2, value1 → value1, value3, value2, value1 	insert a copy of the top value into the stack two (if value2 is double or long it takes up the entry of value3, too) or three values (if value2 is neither double nor long) from the top
    DupX2,

    ///  		{value2, value1} → {value2, value1}, {value2, value1} 	duplicate top two stack words (two values, if value1 is not double nor long; a single value, if value1 is double or long)
    Dup2,

    ///  		value3, {value2, value1} → {value2, value1}, value3, {value2, value1} 	duplicate two words and insert beneath third word (see explanation above)
    Dup2X1,

    ///  		{value4, value3}, {value2, value1} → {value2, value1}, {value4, value3}, {value2, value1} 	duplicate two words and insert beneath fourth word
    Dup2X2,

    ///  		value → result 	convert a float to a double
    F2d,

    ///  		value → result 	convert a float to an int
    F2i,

    ///  		value → result 	convert a float to a long
    F2l,

    ///  		value1, value2 → result 	add two floats
    Fadd,

    ///  		arrayref, index → value 	load a float from an array
    Faload,

    ///  		arrayref, index, value → 	store a float in an array
    Fastore,

    ///  		value1, value2 → result 	compare two floats, 1 on NaN
    Fcmpg,

    ///  		value1, value2 → result 	compare two floats, -1 on NaN
    Fcmpl,

    ///  		→ 0.0f 	push 0.0f on the stack
    Fconst0,

    ///  		→ 1.0f 	push 1.0f on the stack
    Fconst1,

    ///  		→ 2.0f 	push 2.0f on the stack
    Fconst2,

    ///  		value1, value2 → result 	divide two floats
    Fdiv,

    ///  	1: index 	→ value 	load a float value from a local variable #index
    Fload(u8),

    ///  		→ value 	load a float value from local variable 0
    Fload0,

    ///  		→ value 	load a float value from local variable 1
    Fload1,

    ///  		→ value 	load a float value from local variable 2
    Fload2,

    ///  		→ value 	load a float value from local variable 3
    Fload3,

    ///  		value1, value2 → result 	multiply two floats
    Fmul,

    ///  		value → result 	negate a float
    Fneg,

    ///  		value1, value2 → result 	get the remainder from a division between two floats
    Frem,

    ///  		value → [empty] 	return a float
    Freturn,

    ///  	1: index 	value → 	store a float value into a local variable #index
    Fstore(u8),

    ///  		value → 	store a float value into local variable 0
    Fstore0,

    ///  		value → 	store a float value into local variable 1
    Fstore1,

    ///  		value → 	store a float value into local variable 2
    Fstore2,

    ///  		value → 	store a float value into local variable 3
    Fstore3,

    ///  		value1, value2 → result 	subtract two floats
    Fsub,

    ///  	2: indexbyte1, indexbyte2 	objectref → value 	get a field value of an object objectref, where the field is identified by field reference in the constant pool index (indexbyte1 << 8 | indexbyte2)
    Getfield(u16),

    ///  	2: indexbyte1, indexbyte2 	→ value 	get a static field value of a class, where the field is identified by field reference in the constant pool index (indexbyte1 << 8 | indexbyte2)
    Getstatic(u16),

    ///  	2: branchbyte1, branchbyte2 	[no change] 	goes to another instruction at branchoffset (signed short constructed from unsigned bytes branchbyte1 << 8 | branchbyte2)
    Goto(u16),

    ///  	4: branchbyte1, branchbyte2, branchbyte3, branchbyte4 	[no change] 	goes to another instruction at branchoffset (signed int constructed from unsigned bytes branchbyte1 << 24 | branchbyte2 << 16 | branchbyte3 << 8 | branchbyte4)
    GotoW(u32),

    ///  		value → result 	convert an int into a byte
    I2b,

    ///  		value → result 	convert an int into a character
    I2c,

    ///  		value → result 	convert an int into a double
    I2d,

    ///  		value → result 	convert an int into a float
    I2f,

    ///  		value → result 	convert an int into a long
    I2l,

    ///  		value → result 	convert an int into a short
    I2s,

    ///  		value1, value2 → result 	add two ints
    Iadd,

    ///  		arrayref, index → value 	load an int from an array
    Iaload,

    ///  		value1, value2 → result 	perform a bitwise AND on two integers
    Iand,

    ///  		arrayref, index, value → 	store an int into an array
    Iastore,

    ///  		→ -1 	load the int value −1 onto the stack
    IconstM1,

    ///  		→ 0 	load the int value 0 onto the stack
    Iconst0,

    ///  		→ 1 	load the int value 1 onto the stack
    Iconst1,

    ///  		→ 2 	load the int value 2 onto the stack
    Iconst2,

    ///  		→ 3 	load the int value 3 onto the stack
    Iconst3,

    ///  		→ 4 	load the int value 4 onto the stack
    Iconst4,

    ///  		→ 5 	load the int value 5 onto the stack
    Iconst5,

    ///  		value1, value2 → result 	divide two integers
    Idiv,

    ///  	2: branchbyte1, branchbyte2 	value1, value2 → 	if references are equal, branch to instruction at branchoffset (signed short constructed from unsigned bytes branchbyte1 << 8 | branchbyte2)
    IfAcmpeq(u16),

    ///  	2: branchbyte1, branchbyte2 	value1, value2 → 	if references are not equal, branch to instruction at branchoffset (signed short constructed from unsigned bytes branchbyte1 << 8 | branchbyte2)
    IfAcmpne(u16),

    ///  	2: branchbyte1, branchbyte2 	value1, value2 → 	if ints are equal, branch to instruction at branchoffset (signed short constructed from unsigned bytes branchbyte1 << 8 | branchbyte2)
    IfIcmpeq(u16),

    ///  	2: branchbyte1, branchbyte2 	value1, value2 → 	if value1 is greater than or equal to value2, branch to instruction at branchoffset (signed short constructed from unsigned bytes branchbyte1 << 8 | branchbyte2)
    IfIcmpge(u16),

    ///  	2: branchbyte1, branchbyte2 	value1, value2 → 	if value1 is greater than value2, branch to instruction at branchoffset (signed short constructed from unsigned bytes branchbyte1 << 8 | branchbyte2)
    IfIcmpgt(u16),

    ///  	2: branchbyte1, branchbyte2 	value1, value2 → 	if value1 is less than or equal to value2, branch to instruction at branchoffset (signed short constructed from unsigned bytes branchbyte1 << 8 | branchbyte2)
    IfIcmple(u16),

    ///  	2: branchbyte1, branchbyte2 	value1, value2 → 	if value1 is less than value2, branch to instruction at branchoffset (signed short constructed from unsigned bytes branchbyte1 << 8 | branchbyte2)
    IfIcmplt(u16),

    ///  	2: branchbyte1, branchbyte2 	value1, value2 → 	if ints are not equal, branch to instruction at branchoffset (signed short constructed from unsigned bytes branchbyte1 << 8 | branchbyte2)
    IfIcmpne(u16),

    ///  	2: branchbyte1, branchbyte2 	value → 	if value is 0, branch to instruction at branchoffset (signed short constructed from unsigned bytes branchbyte1 << 8 | branchbyte2)
    Ifeq(u16),

    ///  	2: branchbyte1, branchbyte2 	value → 	if value is greater than or equal to 0, branch to instruction at branchoffset (signed short constructed from unsigned bytes branchbyte1 << 8 | branchbyte2)
    Ifge(u16),

    ///  	2: branchbyte1, branchbyte2 	value → 	if value is greater than 0, branch to instruction at branchoffset (signed short constructed from unsigned bytes branchbyte1 << 8 | branchbyte2)
    Ifgt(u16),

    ///  	2: branchbyte1, branchbyte2 	value → 	if value is less than or equal to 0, branch to instruction at branchoffset (signed short constructed from unsigned bytes branchbyte1 << 8 | branchbyte2)
    Ifle(u16),

    ///  	2: branchbyte1, branchbyte2 	value → 	if value is less than 0, branch to instruction at branchoffset (signed short constructed from unsigned bytes branchbyte1 << 8 | branchbyte2)
    Iflt(u16),

    ///  	2: branchbyte1, branchbyte2 	value → 	if value is not 0, branch to instruction at branchoffset (signed short constructed from unsigned bytes branchbyte1 << 8 | branchbyte2)
    Ifne(u16),

    ///  	2: branchbyte1, branchbyte2 	value → 	if value is not null, branch to instruction at branchoffset (signed short constructed from unsigned bytes branchbyte1 << 8 | branchbyte2)
    Ifnonnull(u16),

    ///  	2: branchbyte1, branchbyte2 	value → 	if value is null, branch to instruction at branchoffset (signed short constructed from unsigned bytes branchbyte1 << 8 | branchbyte2)
    Ifnull(u16),

    ///  	2: index, const 	[No change] 	increment local variable #index by signed byte const
    Iinc(u8, u8),

    ///  	1: index 	→ value 	load an int value from a local variable #index
    Iload(u8),

    ///  		→ value 	load an int value from local variable 0
    Iload0,

    ///  		→ value 	load an int value from local variable 1
    Iload1,

    ///  		→ value 	load an int value from local variable 2
    Iload2,

    ///  		→ value 	load an int value from local variable 3
    Iload3,

    ///  			reserved for implementation-dependent operations within debuggers; should not appear in any class file
    Impdep1,

    ///  			reserved for implementation-dependent operations within debuggers; should not appear in any class file
    Impdep2,

    ///  		value1, value2 → result 	multiply two integers
    Imul,

    ///  		value → result 	negate int
    Ineg,

    ///  	2: indexbyte1, indexbyte2 	objectref → result 	determines if an object objectref is of a given type, identified by class reference index in constant pool (indexbyte1 << 8 | indexbyte2)
    Instanceof(u16),

    ///  	4: indexbyte1, indexbyte2, 0, 0 	[arg1, [arg2 ...]] → result 	invokes a dynamic method and puts the result on the stack (might be void); the method is identified by method reference index in constant pool (indexbyte1 << 8 | indexbyte2)
    Invokedynamic(u16),

    ///  	4: indexbyte1, indexbyte2, count, 0 	objectref, [arg1, arg2, ...] → result 	invokes an interface method on object objectref and puts the result on the stack (might be void); the interface method is identified by method reference index in constant pool (indexbyte1 << 8 | indexbyte2)
    Invokeinterface(u16, u8),

    ///  	2: indexbyte1, indexbyte2 	objectref, [arg1, arg2, ...] → result 	invoke instance method on object objectref and puts the result on the stack (might be void); the method is identified by method reference index in constant pool (indexbyte1 << 8 | indexbyte2)
    Invokespecial(u16),

    ///  	2: indexbyte1, indexbyte2 	[arg1, arg2, ...] → result 	invoke a static method and puts the result on the stack (might be void); the method is identified by method reference index in constant pool (indexbyte1 << 8 | indexbyte2)
    Invokestatic(u16),

    ///  	2: indexbyte1, indexbyte2 	objectref, [arg1, arg2, ...] → result 	invoke virtual method on object objectref and puts the result on the stack (might be void); the method is identified by method reference index in constant pool (indexbyte1 << 8 | indexbyte2)
    Invokevirtual(u16),

    ///  		value1, value2 → result 	bitwise int OR
    Ior,

    ///  		value1, value2 → result 	logical int remainder
    Irem,

    ///  		value → [empty] 	return an integer from a method
    Ireturn,

    ///  		value1, value2 → result 	int shift left
    Ishl,

    ///  		value1, value2 → result 	int arithmetic shift right
    Ishr,

    ///  	1: index 	value → 	store int value into variable #index
    Istore(u8),

    ///  		value → 	store int value into variable 0
    Istore0,

    ///  		value → 	store int value into variable 1
    Istore1,

    ///  		value → 	store int value into variable 2
    Istore2,

    ///  		value → 	store int value into variable 3
    Istore3,

    ///  		value1, value2 → result 	int subtract
    Isub,

    ///  		value1, value2 → result 	int logical shift right
    Iushr,

    ///  		value1, value2 → result 	int xor
    Ixor,

    ///  	2: branchbyte1, branchbyte2 	→ address 	jump to subroutine at branchoffset (signed short constructed from unsigned bytes branchbyte1 << 8 | branchbyte2) and place the return address on the stack
    Jsr(u16),

    ///  	4: branchbyte1, branchbyte2, branchbyte3, branchbyte4 	→ address 	jump to subroutine at branchoffset (signed int constructed from unsigned bytes branchbyte1 << 24 | branchbyte2 << 16 | branchbyte3 << 8 | branchbyte4) and place the return address on the stack
    JsrW(u32),

    ///  		value → result 	convert a long to a double
    L2d,

    ///  		value → result 	convert a long to a float
    L2f,

    ///  		value → result 	convert a long to a int
    L2i,

    ///  		value1, value2 → result 	add two longs
    Ladd,

    ///  		arrayref, index → value 	load a long from an array
    Laload,

    ///  		value1, value2 → result 	bitwise AND of two longs
    Land,

    ///  		arrayref, index, value → 	store a long to an array
    Lastore,

    ///  		value1, value2 → result 	push 0 if the two longs are the same, 1 if value1 is greater than value2, -1 otherwise
    Lcmp,

    ///  		→ 0L 	push 0L (the number zero with type long) onto the stack
    Lconst0,

    ///  		→ 1L 	push 1L (the number one with type long) onto the stack
    Lconst1,

    ///  	1: index 	→ value 	push a constant #index from a constant pool (String, int, float, Class, java.lang.invoke.MethodType, java.lang.invoke.MethodHandle, or a dynamically-computed constant) onto the stack
    Ldc(u8),

    ///  	2: indexbyte1, indexbyte2 	→ value 	push a constant #index from a constant pool (String, int, float, Class, java.lang.invoke.MethodType, java.lang.invoke.MethodHandle, or a dynamically-computed constant) onto the stack (wide index is constructed as indexbyte1 << 8 | indexbyte2)
    LdcW(u16),

    ///  	2: indexbyte1, indexbyte2 	→ value 	push a constant #index from a constant pool (double, long, or a dynamically-computed constant) onto the stack (wide index is constructed as indexbyte1 << 8 | indexbyte2)
    Ldc2W(u16),

    ///  		value1, value2 → result 	divide two longs
    Ldiv,

    ///  	1: index 	→ value 	load a long value from a local variable #index
    Lload(u8),

    ///  		→ value 	load a long value from a local variable 0
    Lload0,

    ///  		→ value 	load a long value from a local variable 1
    Lload1,

    ///  		→ value 	load a long value from a local variable 2
    Lload2,

    ///  		→ value 	load a long value from a local variable 3
    Lload3,

    ///  		value1, value2 → result 	multiply two longs
    Lmul,

    ///  		value → result 	negate a long
    Lneg,

    ///  	8+: <0–3 bytes padding>, defaultbyte1, defaultbyte2, defaultbyte3, defaultbyte4, npairs1, npairs2, npairs3, npairs4, match-offset pairs... 	key → 	a target address is looked up from a table using a key and execution continues from the instruction at that address
    Lookupswitch(),

    ///  		value1, value2 → result 	bitwise OR of two longs
    Lor,

    ///  		value1, value2 → result 	remainder of division of two longs
    Lrem,

    ///  		value → [empty] 	return a long value
    Lreturn,

    ///  		value1, value2 → result 	bitwise shift left of a long value1 by int value2 positions
    Lshl,

    ///  		value1, value2 → result 	bitwise shift right of a long value1 by int value2 positions
    Lshr,

    ///  	1: index 	value → 	store a long value in a local variable #index
    Lstore(u8),

    ///  		value → 	store a long value in a local variable 0
    Lstore0,

    ///  		value → 	store a long value in a local variable 1
    Lstore1,

    ///  		value → 	store a long value in a local variable 2
    Lstore2,

    ///  		value → 	store a long value in a local variable 3
    Lstore3,

    ///  		value1, value2 → result 	subtract two longs
    Lsub,

    ///  		value1, value2 → result 	bitwise shift right of a long value1 by int value2 positions, unsigned
    Lushr,

    ///  		value1, value2 → result 	bitwise XOR of two longs
    Lxor,

    ///  		objectref → 	enter monitor for object ("grab the lock" – start of synchronized() section)
    Monitorenter,

    ///  		objectref → 	exit monitor for object ("release the lock" – end of synchronized() section)
    Monitorexit,

    ///  	3: indexbyte1, indexbyte2, dimensions 	count1, [count2,...] → arrayref 	create a new array of dimensions dimensions of type identified by class reference in constant pool index (indexbyte1 << 8 | indexbyte2); the sizes of each dimension is identified by count1, [count2, etc.]
    Multianewarray(u16, u8),

    ///  	2: indexbyte1, indexbyte2 	→ objectref 	create new object of type identified by class reference in constant pool index (indexbyte1 << 8 | indexbyte2)
    New(u16),

    ///  	1: atype 	count → arrayref 	create new array with count elements of primitive type identified by atype
    Newarray(ArrayType),

    ///  		[No change] 	perform no operation
    Nop,

    ///  		value → 	discard the top value on the stack
    Pop,

    ///  		{value2, value1} → 	discard the top two values on the stack (or one value, if it is a double or long)
    Pop2,

    ///  	2: indexbyte1, indexbyte2 	objectref, value → 	set field to value in an object objectref, where the field is identified by a field reference index in constant pool (indexbyte1 << 8 | indexbyte2)
    Putfield(u16),

    ///  	2: indexbyte1, indexbyte2 	value → 	set static field to value in a class, where the field is identified by a field reference index in constant pool (indexbyte1 << 8 | indexbyte2)
    Putstatic(u16),

    ///  	1: index 	[No change] 	continue execution from address taken from a local variable #index (the asymmetry with jsr is intentional)
    Ret(u8),

    ///  		→ [empty] 	return void from method
    Return,

    ///  		arrayref, index → value 	load short from array
    Saload,

    ///  		arrayref, index, value → 	store short to array
    Sastore,

    ///  	2: byte1, byte2 	→ value 	push a short onto the stack as an integer value
    Sipush(i16),

    ///  		value2, value1 → value1, value2 	swaps two top words on the stack (note that value1 and value2 must not be double or long)
    Swap,

    ///  	16+: [0–3 bytes padding], defaultbyte1, defaultbyte2, defaultbyte3, defaultbyte4, lowbyte1, lowbyte2, lowbyte3, lowbyte4, highbyte1, highbyte2, highbyte3, highbyte4, jump offsets... 	index → 	continue execution from an address in the table at offset index
    Tableswitch(),

    ///  	3/5: opcode, indexbyte1, indexbyte2
    Wide(),
}
