#!/usr/bin/python3

import datetime as dt
import json
import subprocess as sp

FORMAT = '%Y-%m-%d'

current = dt.datetime(2015, 8, 20)
print('start:', current.strftime(FORMAT))

end = dt.datetime(2016, 3, 15)
print('end:', end.strftime(FORMAT))

results_file = '/home/adam/rust_projects/rust-runtime-benchmarks/perf-benchmarks.json'

results = {}

while current <= end:
    date_str = current.strftime(FORMAT)
    toolchain_str = 'nightly-{}'.format(date_str)
    results[date_str] = {}

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
    benches = ['cbor'
               'crc',
               'csv',
               'itertools',
               'memchr',
               'optional',
               'permutohedron',
               'rand',
               'regex',
               'suffix',
               'uuid']

    for b in benches:
        cmd = ['perf', 'stat', '-x,', 'target/release/bench-suite-linux', b]
        completed = sp.run(cmd, universal_newlines=True, stderr=sp.STDOUT, stdout=sp.PIPE)
        try:
            completed.check_returncode()
        except sp.CalledProcessError as e:
            print('WARNING: benches failed to run on', toolchain_str)
            print(completed.stdout, '\n', completed.stderr)
            current += dt.timedelta(days=1)
            continue

        print('parsing benchmark output...')
        output = completed.stdout.split('\n')

        # get the json results from getrusage and average it
        single_result = json.loads(output[0])[b]
        for k in single_result:
            single_result[k] = sum(single_result[k]) / len(single_result[k])

        # get the line-by-line results from perf
        for l in output[1:]:
            tokens = [t for t in l.split(',') if len(t) > 0]

            if len(tokens) < 2:
                continue

            if tokens[0] == '<not supported>':
                continue

            if '.' in tokens[0]:
                count = float(tokens[0])
            else:
                count = int(tokens[0])

            single_result[tokens[1]] = count

        results[date_str][b] = single_result

    print('moving to next day')
    current += dt.timedelta(days=1)

print('writing benchmark results to', results_file)
with open(results_file, 'w') as f:
    f.write(json.dumps(results))
