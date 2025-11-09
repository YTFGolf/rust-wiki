import subprocess
import sys
from pyperclip import copy

cat = sys.argv[1]
result = subprocess.run(["cargo", "r", "--release", "encounters", cat], stdout=subprocess.PIPE, text=True, encoding="utf-8")

copy(result.stdout)
