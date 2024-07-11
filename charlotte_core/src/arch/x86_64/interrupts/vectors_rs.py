template = """fn iv_{num}();
"""

file_tmp = """
// handlers
extern "C" {{
    {handlers}
}}
// end of handlers
pub const IV_HANDLERS: [unsafe extern "C" fn(); 224] = [{refs}];
"""

if __name__ == '__main__':
    handlers = ""
    refs = []

    for i in range(32, 256):
        handlers += template.format(num=i)
        refs.append(f"iv_{i}")

    print(file_tmp.format(handlers=handlers, refs=", ".join(refs)))
