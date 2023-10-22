.section ".text.boot"

.global _start

_start:
    // check cpu ID is zero (executing on main core), else hang
    mrs     x0, mpidr_el1   // copy cpu affienity data 
    and     x0, x0, #3      // get the lower 2 bits which contain the cpu id 
    cbz     x0, .master     // if cpu id is 0 run master
    b       .wait_for_event // otherwise run wait_for_event

// we're not on the main core, so wait for an event
.wait_for_event:
    wfe
    b       .wait_for_event

// we're on the main core
.master:
    // clean the BSS section
    ldr     x0, =__bss_start    // load __bss_start to x0
    ldr     x1, =__bss_size     // load __bss_size to x1
    bl      memzero             // run memzero with x0 and x1

    // set stack to start below our code
    ldr     x0, =_start // load _start to x0
    mov     sp, x0      // set stack pointer under _start
    
    // jump to our _kernel() routin
    bl      _kernel         // run _kernel
    b       .wait_for_event // the _kernel should not return but if it does .wait_for_event

.globl memzero
// clears data from x0 with size x1
memzero:
    str     xzr, [x0], #8   // write zero byte to x0 addr and incrment x0 by a byte
    subs    x1, x1, #8      // subtract a byte from x1
    b.gt    memzero         // check if x1 is zero if not run memzero
    ret                     // otherwise return