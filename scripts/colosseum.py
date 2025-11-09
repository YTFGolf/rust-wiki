import subprocess
import sys
from pyperclip import copy

num = sys.argv[2]
extra = [
    f'DISPLAYTITLE:Round {int(num) + 1}',
    'LimitedContent',
    'StrategyNeeded',
]
if extra:
   extra = '\n'.join(('{{%s}}' % template for template in extra)) + '\n'
else:
   extra = ''
prepend = extra

append = "\n{{SpecialStages List}}\n[[Category:Event Stages]]\n[[Category:Restriction Stages]]"

map_num = int(sys.argv[1])
stage = f"sr {map_num} {num}"
result = subprocess.run(["cargo", "r", "stage", stage], capture_output=True, text=True, encoding="utf-8")
# if not (result.returncode == 0 and result.stderr == ""):
#     print("Something went wrong...")
#     print(result.stderr)

out = result.stdout
out = f'{prepend}{out}{append}'
out = out.replace(' en.png]]', ' ja.png]]')
import re
out = re.sub(fr"(File:Map\w+){map_num:03}", r"\1grarr", out)
out = out.replace('grarr', '000')
out = re.sub(r"\|(romaji|jpname|script) = \?", fr'|\1 = Round {int(num) + 1}', out)

copy(out)
print(out)
