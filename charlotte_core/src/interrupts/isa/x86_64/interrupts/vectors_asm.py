template = """
.global iv_{num}
iv_{num}:
    call save_regs
    mov rdi, {num}
    call isr_handler
    call restore_regs
    iretq
"""

file_tmp = """
// From vector 32 to 255 create a handler
.code64

.text

.extern save_regs
.extern restore_regs
.extern isr_handler
{handlers}

// end of handlers
"""

if __name__ == '__main__':
    handlers = ""

    for i in range(32, 256):
        handlers += template.format(num=i)

    print(file_tmp.format(handlers=handlers))
