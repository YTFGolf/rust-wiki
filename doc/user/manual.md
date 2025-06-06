## Quick start
You'll need to use the command line to run this program. If you really can't be bothered to learn how to use the command line, follow the [Running through Python](#running-through-python) guide and adapt it to whatever you're doing.

`rust-wiki` will be used as a generic name for running the program. If you are on Windows, substitute `rust-wiki.exe`. If you are using `cargo run`, substitute `cargo run -- {args}` any time command-line arguments are specified.

1. Run `rust-wiki get` to initialise your config (this needs to change lol).
2. Run `rust-wiki get` to download appropriate wiki files.
3. Edit `user-config.toml` and fill in with appropriate values.
4. Run `rust-wiki help` to see what commands are available.

## Available programs

The easiest way to see the available programs is to just run `rust-wiki help`. This will always be up to date with the code and for most programs should be sufficient to explain how to use it. However, some programs will take up a lot of space to explain exactly how to use them and every edge case you need to be aware of.

- `stage-info`: see [stage-info](./stage-info.md)
- `map-info`: uses selectors of the same form as `stage-info` but without the stage number. Also allows you to use map data file names as input.

## Running through Python
Here's a simple Python program you can run. Assuming your files are structured like this:

```
C:.
├── rust-wiki.exe
├── script.py
├── user-config.toml
└───data
    └───...
```

Then if `script.py` contains this:

```Python
import subprocess
from pyperclip import copy

extra = [
   # 'RemovedContent',
   'LimitedContent',
   'Stub',
]
if extra:
   extra = '\n'.join(('{{%s}}' % template for template in extra)) + '\n'
else:
   extra = ''
prepend = extra

append = "\n{{SpecialStages List}}\n[[Category:Event Stages]]"

stage = "l 0"
result = subprocess.run(["rust-wiki.exe", "stage", stage], capture_output=True, text=True)
if not (result.returncode == 0 and result.stderr == ""):
    print("Something went wrong...")

out = result.stdout
out = f'{prepend}{out}{append}'
out = out.replace(' en.png]]', ' ja.png]]')

copy(out)
print(out)
```

You can add in appropriate templates above the content, templates and categories below the content, replace all English stage/map name images with Japanese ones, and copy the output to your clipboard. With more modifications you could do things like automatically generating the stage selector or integrating the entire thing with Pywikibot!

My personal scripts are available at [scripts](https://battlecats.miraheze.org/wiki/User:TheWWRNerdGuy/scripts). Other example scripts are available in the [Archive](https://battlecats.miraheze.org/wiki/User:TheWWRNerdGuy/Archive?action=history), such as the [zl script](https://battlecats.miraheze.org/wiki/User:TheWWRNerdGuy/Archive?oldid=152115). You can write your own scripts in your local [scripts](../scripts) folder.
