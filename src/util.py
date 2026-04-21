import hashlib
import shutil


def hash_seed(s: str) -> int:
    h = hashlib.md5(s.encode()).hexdigest()
    return int(h, 16) & 0xFFFFFFFFFFFFFFFF


def center_text(s: str) -> str:
    columns, _ = shutil.get_terminal_size()

    return s.center(columns)
