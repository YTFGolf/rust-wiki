import subprocess
import sys
from pyperclip import copy

collab_name = "Street Fighter V"

collab_name = collab_name.replace(" Collaboration Event", '')
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

out = out.replace("|event = [[name]]", f"|event = [[{collab_name} Collaboration Event]]")
templates = "{{%s}}\n{{CollaborationStages List}}" % collab_name
categories = "[[Category:Collaboration Stages]]"

if out.__contains__("[[No Continues]]"):
    categories += "\n[[Category:No Continue Stages]]"
if out.__contains__("|score reward = "):
    categories += "\n[[Category:Timed Score Stages]]"

if stype == 'ca':
    categories += "\n[[Category:Gauntlets]]"

out = f'{prepend}{out}\n\n{templates}\n{categories}'
out = out.replace("|script = ?", "|script = {{subst:PAGENAME}}")
out = out.strip()

copy(out)
print(out)
