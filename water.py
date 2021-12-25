'Just a demonstration. Makes a top-down plot of water at y=62.'

import sys

sys.path.append('PyAnvilEditor')

import pyanvil

import dansplotcore as dpc

import argparse

parser = argparse.ArgumentParser()
parser.add_argument('world_path')
parser.add_argument('x', type=int)
parser.add_argument('z', type=int)
parser.add_argument('--size', '-s', type=int, default=20)
args = parser.parse_args()

world = pyanvil.world.World(args.world_path)
plot = dpc.Plot()

for x in range(args.x - args.size // 2, args.x + args.size // 2):  # west-east
    print(x)
    for z in range(args.z - args.size // 2, args.z + args.size // 2):  # north-south
        if world.get_block((x, 62, z)).get_state().name == 'minecraft:water':
            plot.rect(x, -z, x+1, -(z+1), r=0.0, g=0.0, b=1.0)

plot.show()
