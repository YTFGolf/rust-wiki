import subprocess
import sys
from pyperclip import copy

extra = [
   # 'RemovedContent',
   'LimitedContent',
   'StrategyNeeded',
   # 'Stub',
]
if extra:
   extra = '\n'.join(('{{%s}}' % template for template in extra)) + '\n'
else:
   extra = ''
prepend = extra

chapter = int(sys.argv[1])
append = "\n{{LegendStages}}\n[[Category:Sub-chapter %d Stages]]\n[[Category:Zero Legends Stages]]" % (99 + chapter)

map = sys.argv[2]

stage = f"zl {chapter} {map}"

release_mode = "--"
result = subprocess.run(["cargo", "r", release_mode, "stage", stage], stdout=subprocess.PIPE, text=True, encoding="utf-8")
if not (result.returncode == 0 and result.stderr == ""):
    print("Something went wrong...")

out = result.stdout
out = f'{prepend}{out}{append}'
out = out.replace(' en.png]]', ' ja.png]]')

copy(out)
print(out)
