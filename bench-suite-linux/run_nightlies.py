#!/usr/bin/python3

import datetime as dt
import json
import subprocess as sp

FORMAT = '%Y-%m-%d'

current = dt.datetime(2015, 8, 20)
print('start:', current.strftime(FORMAT))

end = dt.datetime(2016, 2, 20)
print('end:', end.strftime(FORMAT))

results_file = '/home/adam/rust_projects/rust-runtime-benchmarks/getrusage-benchmarks.json'

results = {}

while current <= end:
    date_str = current.strftime(FORMAT)
    toolchain_str = 'nightly-{}'.format(date_str)

    print('getting toolchain', toolchain_str)
    cmd = ['multirust', 'update', toolchain_str]
    completed = sp.run(cmd, universal_newlines=True, stderr=sp.STDOUT, stdout=sp.PIPE)
    try:
        completed.check_returncode()
    except sp.CalledProcessError as e:
        print('WARNING: failed to update toolchain', toolchain_str)
        current += dt.timedelta(days=1)
        continue

    print('building benches on', toolchain_str)
    cmd = ['cargo', 'clean']
    sp.run(cmd)

    cmd = ['multirust', 'run', toolchain_str, 'cargo', 'build', '--release']
    completed = sp.run(cmd, universal_newlines=True, stderr=sp.STDOUT, stdout=sp.PIPE)
    try:
        completed.check_returncode()
    except sp.CalledProcessError as e:
        print('WARNING: failed to build benches on', toolchain_str)
        print(completed.stdout)
        current += dt.timedelta(days=1)

        continue

    print('running benches for', date_str)
    completed = sp.run(['target/release/bench-suite-linux'], universal_newlines=True, stderr=sp.PIPE, stdout=sp.PIPE)
    try:
        completed.check_returncode()
    except sp.CalledProcessError as e:
        print('WARNING: benches failed to run on', toolchain_str)
        print(completed.stdout, '\n', completed.stderr)
        current += dt.timedelta(days=1)
        continue

    print('parsing benchmark output...')
    single_result = json.loads(completed.stdout)
    results[date_str] = single_result

    print('moving to next day')
    current += dt.timedelta(days=1)

print('writing benchmark results to', results_file)
with open(results_file, 'w') as f:
    f.write(json.dumps(results))
