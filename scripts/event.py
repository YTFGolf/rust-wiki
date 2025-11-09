import subprocess
import sys
from pyperclip import copy

extra = [
    # 'RemovedContent',
    'LimitedContent',
    'StrategyNeeded',
    'TranslationNeeded',
    # 'Stub',
]
if extra:
    extra = '\n'.join(('{{%s}}' % template for template in extra)) + '\n'
else:
    extra = ''
prepend = extra

stype = sys.argv[1]
chapter = sys.argv[2]
map = sys.argv[3]

stage = f"{stype} {chapter} {map}"
# release_mode = "--release"
release_mode = "--"
result = subprocess.run(["cargo", "r", release_mode, "stage", stage], capture_output=True, text=True, encoding="utf-8")
if not (result.returncode == 0 and result.stderr == ""):
    print("Something went wrong...", repr(result.stderr))

out = result.stdout.strip()
templates = "{{SpecialStages List}}"
categories = "[[Category:Event Stages]]"

if stype == "a":
    categories += '\n[[Category:Gauntlets]]'

if out.__contains__("[[No Continues]]"):
    categories += "\n[[Category:No Continue Stages]]"
if out.__contains__("|score reward = "):
    categories += "\n[[Category:Timed Score Stages]]"

out = f'{prepend}{out}\n\n{templates}\n{categories}'
out = out.replace(' en.png]]', ' ja.png]]')
out = out.replace("|script = ?", "|script = {{subst:PAGENAME}}")
out = out.strip()

copy(out)
print(out)
