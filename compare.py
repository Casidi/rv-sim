from pathlib import Path
import os
import subprocess as sp
import shlex

spike_path = '../riscv-isa-sim/build/spike'
rvsim_path = './target/debug/rv-sim'
trace_dir = 'trace_dir'

if not os.path.isdir(trace_dir):
    os.mkdir(trace_dir)

# Collect test binaries
#test_paths = ['../dhrystone.riscv']
#test_paths = ['../riscv-tests/isa/rv64ui-p-add']
test_paths = []

for p in Path('../riscv-tests/isa').glob('rv64*-p-*'):
#for p in Path('../riscv-tests/benchmarks').rglob('*.riscv'):
#for p in Path('../riscv-tests/isa').glob('rv64uc-p-*'):
    if p.suffix == '.dump' or p.is_dir():
        continue
    test_paths.append(str(p))

pass_count = 0
for test_path in test_paths:
    test_name = os.path.basename(test_path)
    print(f'Running {test_name}...', end='', flush=True)

    spike_trace_file_name = os.path.join(trace_dir, f'{test_name}.spike')
    if not os.path.isfile(spike_trace_file_name):
        run_result = sp.run(shlex.split(f'{spike_path} {test_path}'),
                                check=False, stdout=sp.PIPE, stderr=sp.STDOUT)
        spike_output = run_result.stdout
        with open(spike_trace_file_name, 'w') as f:
            f.write(spike_output.decode())

    #print('Running RV-Sim')
    run_result = sp.run(shlex.split(f'{rvsim_path} {test_path}'),
                        check=False, stdout=sp.PIPE, stderr=sp.STDOUT)
    rvsim_output = run_result.stdout
    with open(os.path.join(trace_dir, f'{test_name}.rvsim'), 'w') as f:
        f.write(rvsim_output.decode())

    #print('Comparing the result')

    '''
    i = 0
    while True:
        if i == len(rvsim_output) or i == len(spike_output):
            break

        if spike_output[i] != rvsim_output[i]:
            print(f'Mismatch at line {i}, spike = ')
            print(spike_output[i])
            print('  rvsim = ')
            print(rvsim_output[i])
            exit()

        i += 1
    '''
    if rvsim_output.find(b'RISCV_TEST_PASS') != -1:
        pass_count += 1
        print('Pass')
    elif rvsim_output.find(b'RISCV_TEST_FAIL') != -1:
        print('Fail normal')
    else:
        print('Fail crash')

print(f'Summary: {pass_count} out of {len(test_paths)} tests passed')
