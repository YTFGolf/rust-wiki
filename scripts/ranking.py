import subprocess
import re
import sys
from pyperclip import copy

extra = [
   # 'RemovedContent',
   'LimitedContent',
   # 'StrategyNeeded',
   # 'Stub',
]
if extra:
   extra = '\n'.join(('{{%s}}' % template for template in extra)) + '\n'
else:
   extra = ''
prepend = extra

if len(sys.argv) > 1:
    map = sys.argv[1]
else:
    map = input("Enter number: ")

stage = f"rank {map} {0}"
result = subprocess.run(["cargo", "r", "stage", stage, "-l=warn"], capture_output=True, text=True, encoding="utf-8")
if not (result.returncode == 0 and result.stderr == ""):
    print(f"Something went wrong... {repr(result.stderr)}")

explanation = '''*You have three minutes to defeat as many enemies as possible.
*You have unlimited Speed Ups and Cat CPUs but no other power ups.
*The base has unlimited health and can attack Cats that are close to it.
*You gain no money for defeating enemies.
*Defeating enemies will score points and the number varies.
*The total points will decrease overtime.'''

out = result.stdout
out = out.replace(' en.png]]', ' ja.png]]')
out = out.replace('|script = ?', '|script = {{subst:SUBPAGENAME}}')
out = out.replace("==Strategy==\n-", f"==Explanation==\n{explanation}")
limit = re.search(r' = (\d minutes)', out).group(1)
out = re.sub('\w+ minutes', limit, out)
out = prepend + out.strip() + '\n\n{{DojoStages}}\n[[Category:Ranking Dojo Stages]]'

copy(out)
print("Copied")
