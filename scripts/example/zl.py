import subprocess
from pyperclip import copy

extra = [
   # 'RemovedContent',
   'LimitedContent',
   # 'Stub',
]
if extra:
   extra = '\n'.join(('{{%s}}' % template for template in extra)) + '\n'
else:
   extra = ''
prepend = extra

append = "\n{{ZeroLegendStages}}\n[[Category:Sub-chapter 114 Stages]]\n[[Category:Zero Legends Stages]]"

map = input("Enter number: ")
stage = f"zl 15 {map}"
result = subprocess.run(["target/release/rust-wiki.exe", "stage", stage], capture_output=True, text=True)
if not (result.returncode == 0 and result.stderr == ""):
    print("Something went wrong...")

out = result.stdout
out = f'{prepend}{out}{append}'
out = out.replace(' en.png]]', ' ja.png]]')

copy(out)
print(out)
