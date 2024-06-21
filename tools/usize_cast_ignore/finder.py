from typing import Optional, Tuple

from const import LINT_REASON, LOOK_FOR

def find_usize_cast(lint: dict) -> Optional[Tuple[str, int]]:
    if lint.get('reason') != LINT_REASON:
        return
    msg = lint.get('message')
    if msg is None:
        return
    if LOOK_FOR not in msg.get('rendered'):
        return
    
    for child in msg.get('children'):
        spans = child.get('spans')
        if len(spans) == 0:
            continue
        
        filename = spans[0].get('file_name')
        line = spans[0].get('line_start')
        break
    
    return (filename, line)
