import subprocess

subprocess.run(['trunk', 'build'], cwd='frontend')
subprocess.run(['cargo', 'build'])