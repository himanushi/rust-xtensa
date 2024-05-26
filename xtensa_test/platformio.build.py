import os
import subprocess

def before_build(source, target, env):
    result = subprocess.run(
        ["platformio", "run", "-t", "menuconfig"],
        capture_output=True,
        text=True,
        check=True
    )
    env.ParseConfigFile("platformio.ini")

Import("env")

env.AddPreAction("build", before_build)
