import subprocess
import sys
from pyperclip import copy

prepend = [
    # 'RemovedContent',
    'LimitedContent',
    'StrategyNeeded',
    'TranslationNeeded'
    # 'Stub',
]
if prepend:
   prepend = '\n'.join(('{{%s}}' % template for template in prepend)) + '\n'
else:
   prepend = ''

collab_name = "Merc Storia"

chapter = sys.argv[1]
map = sys.argv[2]
# release_mode = "--release"
release_mode = "--"
result = subprocess.run(["cargo", "r", release_mode, "gauntlet", chapter, map, "-l=warn"], stdout=subprocess.PIPE, text=True, encoding="utf-8")

out = result.stdout.strip()

if "<tabber>" in out and 'Lv.' in out:
    title = out[len("<tabber>"):out.index('Lv.')].strip()
    out = out.replace("|script = ?", f"|script = {title} Lv.X")
    st = 'rowspan="2" style="text-align: center;"'
    out = out.replace(f'! {st} |Stage', f'! {st} |{title}')

# check if collab
if out.__contains__("|event = [[name]]"):
    collab = True
else:
    collab = False

if collab:
    out = out.replace("|event = [[name]]", f"|event = [[{collab_name} Collaboration Event]]")
    templates = "{{%s}}\n{{CollaborationStages List}}" % collab_name
    categories = "[[Category:Collaboration Stages]]"
else:
    templates = "{{SpecialStages List}}"
    categories = ""

if out.__contains__("|restriction = [[No Continues]]"):
    categories += "\n[[Category:No Continue Stages]]"

categories += "\n[[Category:Gauntlets]]"

out = f'{prepend}{out}\n\n{templates}\n{categories}'

copy(out)
