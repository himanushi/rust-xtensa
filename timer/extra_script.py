import os
from SCons.Script import DefaultEnvironment

env = DefaultEnvironment()

def before_build(source, target, env):
    os.system('cargo build --release')

env.AddPreAction("buildprog", before_build)
